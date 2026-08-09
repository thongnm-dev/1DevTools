<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Menu from "primevue/menu";
import type { MenuItem } from "primevue/menuitem";

import { useDocker } from "../composables/useDocker";
import ToggleChip from "@/shared/components/ToggleChip.vue";
import { friendlyError } from "@/tauri/commands/_base";
import type { DockerContainer, DockerImage, DockerProject } from "@/models/docker";

import DockerBuildDialog, { type AdHocBuildPayload } from "./DockerBuildDialog.vue";
import DockerProjectDialog from "./DockerProjectDialog.vue";
import DockerBuildLogDialog from "./DockerBuildLogDialog.vue";
import DockerTerminalDialog from "./DockerTerminalDialog.vue";
import DockerConfirmDialog from "./DockerConfirmDialog.vue";

const { t } = useI18n();
const docker = useDocker();

onMounted(() => {
  void docker.init();
});

function containerLabel(c: DockerContainer) {
  return c.name.replace(/^\//, "");
}

function imageLabel(img: DockerImage) {
  return img.repository === "<none>" ? img.id.slice(0, 12) : `${img.repository}:${img.tag}`;
}

const STATE_BADGE: Record<string, string> = {
  running: "bg-emerald-500/15 text-emerald-500",
  exited: "bg-slate-500/15 text-secondary",
  paused: "bg-amber-500/15 text-amber-500",
  restarting: "bg-sky-500/15 text-sky-500",
  dead: "bg-red-500/15 text-red-500",
  created: "bg-slate-500/15 text-secondary",
};

function stateBadgeClass(state: string) {
  return STATE_BADGE[state] ?? "bg-slate-500/15 text-secondary";
}

const STATE_DOT: Record<string, string> = {
  running: "bg-emerald-500",
  exited: "bg-slate-400",
  paused: "bg-amber-500",
  restarting: "bg-sky-500",
  dead: "bg-red-500",
  created: "bg-slate-400",
};

function stateDotClass(state: string) {
  return STATE_DOT[state] ?? "bg-slate-400";
}

// Nhóm container theo thư mục compose (`compose_working_dir`) — các container cùng
// khởi tạo từ một `docker compose up` (vd. cùng thư mục `deploy/`) được gom lại;
// container chạy trực tiếp bằng `docker run` (không có label compose) rơi vào nhóm
// "Standalone" chung. DataTable subheader-grouping yêu cầu dữ liệu đã sắp theo
// đúng key nhóm, nên sort thủ công ở đây trước khi truyền vào bảng.
const groupedContainers = computed(() => {
  return [...docker.containers.value].sort((a, b) => {
    if (a.compose_working_dir === b.compose_working_dir) {
      return containerLabel(a).localeCompare(containerLabel(b));
    }
    if (!a.compose_working_dir) return 1;
    if (!b.compose_working_dir) return -1;
    return a.compose_working_dir.localeCompare(b.compose_working_dir);
  });
});

// Trạng thái mở/thu gọn từng nhóm — mặc định mở, giữ nguyên trạng thái người dùng
// đã chọn khi danh sách refresh; nhóm mới xuất hiện thì tự mở.
const expandedGroups = ref<string[]>([]);

watch(
  groupedContainers,
  (containers) => {
    const known = new Set(expandedGroups.value);
    for (const c of containers) {
      if (!known.has(c.compose_working_dir)) {
        known.add(c.compose_working_dir);
        expandedGroups.value.push(c.compose_working_dir);
      }
    }
  },
  { immediate: true },
);

// === Điều hướng trái (danh mục) / danh sách phải ===

type Category = "containers" | "images" | "projects";
const activeCategory = ref<Category>("containers");

const categories = computed(() => [
  { key: "containers" as Category, label: t("docker.tabs.containers"), icon: "pi-box", count: docker.containers.value.length },
  { key: "images" as Category, label: t("docker.tabs.images"), icon: "pi-images", count: docker.images.value.length },
  { key: "projects" as Category, label: t("docker.tabs.projects"), icon: "pi-folder", count: docker.projects.value.length },
]);

// === Drag-to-resize giữa cột danh mục (trái) và cột danh sách (phải) ===

const NAV_WIDTH_KEY = "docker.width.nav";
function loadNavWidth() {
  const raw = Number(localStorage.getItem(NAV_WIDTH_KEY) ?? "");
  return Number.isFinite(raw) && raw > 0 ? Math.max(160, Math.min(400, raw)) : 200;
}
const navWidth = ref(loadNavWidth());
const isResizingNav = ref(false);
const splitRowRef = ref<HTMLElement | null>(null);
let activeNavResizeMove: ((e: MouseEvent) => void) | null = null;

function startResizeNav(e: MouseEvent) {
  e.preventDefault();
  isResizingNav.value = true;
  const move = (ev: MouseEvent) => {
    const left = splitRowRef.value?.getBoundingClientRect().left ?? 0;
    navWidth.value = Math.max(160, Math.min(400, ev.clientX - left));
  };
  activeNavResizeMove = move;
  document.addEventListener("mousemove", move);
  document.addEventListener("mouseup", endResizeNav);
}

function endResizeNav() {
  isResizingNav.value = false;
  if (activeNavResizeMove) document.removeEventListener("mousemove", activeNavResizeMove);
  document.removeEventListener("mouseup", endResizeNav);
  activeNavResizeMove = null;
  localStorage.setItem(NAV_WIDTH_KEY, String(Math.round(navWidth.value)));
}

onBeforeUnmount(() => {
  if (activeNavResizeMove) document.removeEventListener("mousemove", activeNavResizeMove);
  document.removeEventListener("mouseup", endResizeNav);
  docker.stopPollingDockerReady();
});

// === Menu "..." (more actions) trên mỗi hàng container: restart / open shell / logs ===

const containerMenuRef = ref<InstanceType<typeof Menu> | null>(null);
const containerMenuTarget = ref<DockerContainer | null>(null);

const containerMenuItems = computed<MenuItem[]>(() => {
  const c = containerMenuTarget.value;
  if (!c) return [];
  const items: MenuItem[] = [
    { label: t("docker.actions.restart"), icon: "pi pi-refresh", command: () => docker.restartContainer(c.id) },
  ];
  if (c.state === "running") {
    items.push({ label: t("docker.actions.exec"), icon: "pi pi-desktop", command: () => openExec(c) });
  }
  items.push({ label: t("docker.actions.logs"), icon: "pi pi-file", command: () => openLogs(c) });
  return items;
});

function toggleContainerMenu(event: Event, c: DockerContainer) {
  containerMenuTarget.value = c;
  containerMenuRef.value?.toggle(event);
}

// === Cleanup menu ===

const cleanupMenuRef = ref<InstanceType<typeof Menu> | null>(null);

function toggleCleanupMenu(event: Event) {
  cleanupMenuRef.value?.toggle(event);
}

const cleanupMenuItems = computed<MenuItem[]>(() => [
  {
    label: t("docker.cleanup.containers"),
    icon: "pi pi-box",
    command: () =>
      askConfirm(t("docker.cleanup.containersTitle"), t("docker.cleanup.containersMessage"), t("docker.cleanup.confirm"), () =>
        docker.pruneContainers(),
      ),
  },
  {
    label: t("docker.cleanup.danglingImages"),
    icon: "pi pi-images",
    command: () =>
      askConfirm(t("docker.cleanup.danglingImagesTitle"), t("docker.cleanup.danglingImagesMessage"), t("docker.cleanup.confirm"), () =>
        docker.pruneImages(true),
      ),
  },
  {
    label: t("docker.cleanup.unusedImages"),
    icon: "pi pi-images",
    command: () =>
      askConfirm(t("docker.cleanup.unusedImagesTitle"), t("docker.cleanup.unusedImagesMessage"), t("docker.cleanup.confirm"), () =>
        docker.pruneImages(false),
      ),
  },
  {
    label: t("docker.cleanup.system"),
    icon: "pi pi-exclamation-triangle",
    command: () =>
      askConfirm(t("docker.cleanup.systemTitle"), t("docker.cleanup.systemMessage"), t("docker.cleanup.confirm"), () =>
        docker.pruneSystem(),
      ),
  },
]);

// === Confirm dialog (dùng chung cho mọi thao tác nguy hiểm) ===

const confirmVisible = ref(false);
const confirmTitle = ref("");
const confirmMessage = ref("");
const confirmLabel = ref("");
const confirmBusy = ref(false);
let confirmAction: (() => Promise<void>) | null = null;

function askConfirm(title: string, message: string, label: string, action: () => Promise<void>) {
  confirmTitle.value = title;
  confirmMessage.value = message;
  confirmLabel.value = label;
  confirmAction = action;
  confirmVisible.value = true;
}

async function runConfirmAction() {
  if (!confirmAction) return;
  confirmBusy.value = true;
  try {
    await confirmAction();
    confirmVisible.value = false;
  } finally {
    confirmBusy.value = false;
  }
}

function askRemoveContainer(c: DockerContainer) {
  askConfirm(
    t("docker.confirm.removeContainerTitle"),
    t("docker.confirm.removeContainerMessage", { name: containerLabel(c) }),
    t("docker.confirm.remove"),
    () => docker.removeContainer(c.id, true),
  );
}

function askRemoveImage(img: DockerImage) {
  askConfirm(
    t("docker.confirm.removeImageTitle"),
    t("docker.confirm.removeImageMessage", { name: imageLabel(img) }),
    t("docker.confirm.remove"),
    () => docker.removeImage(img.id, true),
  );
}

function askComposeDown(p: DockerProject) {
  askConfirm(
    t("docker.confirm.composeDownTitle"),
    t("docker.confirm.composeDownMessage", { name: p.name }),
    t("docker.actions.composeDown"),
    () => docker.composeDown(p),
  );
}

function askRemoveProject(p: DockerProject) {
  askConfirm(
    t("docker.confirm.removeProjectTitle"),
    t("docker.confirm.removeProjectMessage", { name: p.name }),
    t("docker.confirm.remove"),
    () => docker.removeProject(p.id),
  );
}

// === Build ad-hoc / project dialogs ===

const buildDialogVisible = ref(false);
const projectDialogVisible = ref(false);
const editingProject = ref<DockerProject | null>(null);

function openAddProject() {
  editingProject.value = null;
  projectDialogVisible.value = true;
}

function openEditProject(p: DockerProject) {
  editingProject.value = p;
  projectDialogVisible.value = true;
}

// === Build log streaming dialog (dùng chung cho build ad-hoc / rebuild project / compose up) ===

const buildLogVisible = ref(false);
const buildLogTitle = ref("");
const buildLogLines = ref<string[]>([]);
const buildLogStatus = ref<"running" | "success" | "error">("running");

function appendBuildLine(line: string) {
  buildLogLines.value.push(line);
}

async function runBuild(title: string, action: () => Promise<void>) {
  buildLogTitle.value = title;
  buildLogLines.value = [];
  buildLogStatus.value = "running";
  buildLogVisible.value = true;
  try {
    await action();
    buildLogStatus.value = "success";
  } catch (e) {
    buildLogLines.value.push(friendlyError(e));
    buildLogStatus.value = "error";
  }
}

async function onAdHocBuild(payload: AdHocBuildPayload) {
  let projectId: number | undefined;
  if (payload.saveName) {
    const project = await docker.addProject(
      payload.saveName,
      "dockerfile",
      payload.contextPath,
      payload.dockerfilePath,
      payload.tag,
      "",
    );
    projectId = project?.id;
  }
  const titleKey = payload.noCache ? "docker.buildLog.titleBuildClean" : "docker.buildLog.titleBuild";
  await runBuild(t(titleKey, { name: payload.tag || payload.contextPath }), () =>
    docker.build(payload.contextPath, payload.dockerfilePath, payload.tag, appendBuildLine, projectId, payload.noCache),
  );
}

/**
 * Build/rebuild lại một project đã lưu. `clean` = true → "Clean and build":
 * bỏ qua cache, kéo lại base image mới nhất (chậm hơn nhưng đảm bảo sạch).
 */
async function rebuildProject(p: DockerProject, clean = false) {
  if (p.kind === "compose") {
    const titleKey = clean ? "docker.buildLog.titleComposeClean" : "docker.buildLog.titleCompose";
    await runBuild(t(titleKey, { name: p.name }), () => docker.composeUp(p.compose_file, appendBuildLine, p.id, clean));
  } else {
    const titleKey = clean ? "docker.buildLog.titleBuildClean" : "docker.buildLog.titleBuild";
    await runBuild(t(titleKey, { name: p.name }), () =>
      docker.build(p.context_path, p.dockerfile_path, p.image_tag, appendBuildLine, p.id, clean),
    );
  }
}

// === Exec / Logs terminal panel ===

const termVisible = ref(false);
const termMode = ref<"exec" | "logs">("exec");
const termTarget = ref<DockerContainer | null>(null);

function openExec(c: DockerContainer) {
  termTarget.value = c;
  termMode.value = "exec";
  termVisible.value = true;
}

function openLogs(c: DockerContainer) {
  termTarget.value = c;
  termMode.value = "logs";
  termVisible.value = true;
}
</script>

<template>
  <div class="flex h-full flex-col gap-3 overflow-hidden">
    <div v-if="!docker.runtimeAvailable" class="flex flex-1 items-center justify-center text-sm text-muted">
      {{ t("common.tauriRuntimeNotAvailable") }}
    </div>

    <template v-else>
      <!-- Header / toolbar -->
      <div class="flex shrink-0 flex-wrap items-center gap-2">
        <h1 class="text-sm font-bold text-ink">{{ t("docker.page.title") }}</h1>
        <span
          v-if="!docker.dockerReady.value"
          class="flex items-center gap-1 rounded-full bg-red-500/10 px-2 py-0.5 text-[11px] font-medium text-red-500"
        >
          <i class="pi pi-exclamation-triangle" />{{ t("docker.page.notRunning") }}
        </span>
        <Button
          v-if="!docker.dockerReady.value"
          size="small"
          outlined
          severity="secondary"
          :loading="docker.startingDesktop.value"
          @click="docker.startDockerDesktop()"
        >
          <i class="pi pi-play mr-1.5" />{{ t("docker.page.startDocker") }}
        </Button>

        <div class="ml-auto flex items-center gap-2">
          <Button
            size="small"
            outlined
            severity="secondary"
            :loading="docker.loadingContainers.value || docker.loadingImages.value || docker.loadingProjects.value"
            @click="docker.refreshAll()"
          >
            <i class="pi pi-sync mr-1.5" />{{ t("docker.page.refresh") }}
          </Button>
          <Button size="small" outlined severity="danger" @click="toggleCleanupMenu">
            <i class="pi pi-trash mr-1.5" />{{ t("docker.page.cleanup") }}
          </Button>
          <Menu ref="cleanupMenuRef" :model="cleanupMenuItems" :popup="true" />
        </div>
      </div>

      <!-- LEFT: danh mục / RIGHT: danh sách -->
      <div ref="splitRowRef" class="flex min-h-0 flex-1" :class="isResizingNav ? 'select-none' : ''">
        <!-- LEFT: điều hướng danh mục -->
        <nav
          class="flex shrink-0 flex-col gap-1 overflow-auto rounded-lg border border-divider bg-panel p-2"
          :style="{ width: navWidth + 'px' }"
        >
          <button
            v-for="cat in categories"
            :key="cat.key"
            type="button"
            class="flex items-center gap-2 rounded-md px-2 py-2 text-left text-xs font-medium transition-colors"
            :class="activeCategory === cat.key ? 'bg-brand text-white' : 'text-secondary hover:bg-canvas'"
            @click="activeCategory = cat.key"
          >
            <i class="pi" :class="cat.icon" />
            <span class="flex-1 truncate">{{ cat.label }}</span>
            <span
              class="rounded-full px-1.5 text-[10px] font-bold"
              :class="activeCategory === cat.key ? 'bg-white/25' : 'bg-canvas text-secondary'"
            >
              {{ cat.count }}
            </span>
          </button>
        </nav>

        <!-- Resize handle: danh mục | danh sách -->
        <div
          class="flex w-2 shrink-0 cursor-col-resize items-center justify-center hover:bg-brand/10"
          :class="isResizingNav ? 'bg-brand/20' : ''"
          @mousedown="startResizeNav"
        >
          <div class="h-8 w-0.5 rounded-full bg-divider" :class="isResizingNav ? 'bg-brand' : ''" />
        </div>

        <!-- RIGHT: danh sách container/image/project theo danh mục đang chọn -->
        <section class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel">
          <!-- Containers -->
          <template v-if="activeCategory === 'containers'">
            <header class="flex shrink-0 items-center gap-2 border-b border-divider p-2">
              <span class="text-xs font-bold text-ink">
                {{ t("docker.tabs.containers") }} ({{ docker.containers.value.length }})
              </span>
              <ToggleChip
                class="ml-auto"
                variant="filter"
                :active="docker.showStopped.value"
                :label="t('docker.page.showStopped')"
                icon="pi-eye"
                @click="docker.toggleShowStopped()"
              />
            </header>
            <div class="min-h-0 flex-1 overflow-auto p-2">
              <DataTable
                :value="groupedContainers"
                :loading="docker.loadingContainers.value"
                class="text-sm"
                data-key="id"
                :table-style="{ 'min-width': '760px' }"
                row-group-mode="subheader"
                group-rows-by="compose_working_dir"
                expandable-row-groups
                v-model:expandedRowGroups="expandedGroups"
                :pt="{ rowGroupHeaderCell: { colspan: 5 } }"
              >
                <template #empty>
                  <div class="p-6 text-center text-xs text-muted">{{ t("common.noData") }}</div>
                </template>
                <template #groupheader="{ data }: { data: DockerContainer }">
                  <div class="ml-1.5 inline-flex items-center gap-2 py-0.5 align-middle">
                    <i class="pi text-[11px] text-muted" :class="data.compose_working_dir ? 'pi-folder' : 'pi-box'" />
                    <span class="truncate text-[11px] font-semibold text-ink">
                      {{ data.compose_working_dir || t("docker.page.standaloneGroup") }}
                    </span>
                    <span v-if="data.compose_project" class="shrink-0 text-[10px] text-muted">
                      ({{ data.compose_project }})
                    </span>
                  </div>
                </template>
                <Column field="name" :header="t('docker.columns.name')">
                  <template #body="{ data }: { data: DockerContainer }">
                    <div class="flex items-center gap-2">
                      <span class="h-2 w-2 shrink-0 rounded-full" :class="stateDotClass(data.state)" />
                      <div class="min-w-0">
                        <div class="truncate font-medium text-ink">{{ containerLabel(data) }}</div>
                        <div class="truncate text-[10px] text-muted">{{ data.image }}</div>
                      </div>
                    </div>
                  </template>
                </Column>
                <Column field="ports" :header="t('docker.columns.ports')">
                  <template #body="{ data }: { data: DockerContainer }">
                    <span class="text-[11px] text-secondary">{{ data.ports || "—" }}</span>
                  </template>
                </Column>
                <Column field="cpu" :header="t('docker.columns.cpu')" style="width: 90px">
                  <template #body="{ data }: { data: DockerContainer }">
                    <span class="text-[11px] text-secondary">{{ data.cpu }}</span>
                  </template>
                </Column>
                <Column field="memory" :header="t('docker.columns.memory')" style="width: 160px">
                  <template #body="{ data }: { data: DockerContainer }">
                    <span class="text-[11px] text-secondary">{{ data.memory }}</span>
                  </template>
                </Column>
                <Column :header="t('docker.columns.actions')" style="width: 140px; white-space: nowrap">
                  <template #body="{ data }: { data: DockerContainer }">
                    <div class="flex items-center gap-0.5 whitespace-nowrap">
                      <Button
                        v-if="data.state !== 'running'"
                        size="small"
                        text
                        rounded
                        :loading="docker.isBusy(data.id)"
                        icon="pi pi-play"
                        severity="success"
                        :title="t('docker.actions.start')"
                        @click="docker.startContainer(data.id)"
                      />
                      <Button
                        v-if="data.state === 'running'"
                        size="small"
                        text
                        rounded
                        :loading="docker.isBusy(data.id)"
                        icon="pi pi-stop"
                        severity="warn"
                        :title="t('docker.actions.stop')"
                        @click="docker.stopContainer(data.id)"
                      />
                      <Button
                        size="small"
                        text
                        rounded
                        :loading="docker.isBusy(data.id)"
                        icon="pi pi-trash"
                        severity="danger"
                        :title="t('docker.actions.remove')"
                        @click="askRemoveContainer(data)"
                      />
                      <Button
                        size="small"
                        text
                        rounded
                        icon="pi pi-ellipsis-v"
                        severity="secondary"
                        style="color: rgb(var(--color-text-secondary)) !important"
                        :title="t('docker.actions.more')"
                        @click="toggleContainerMenu($event, data)"
                      />
                    </div>
                  </template>
                </Column>
              </DataTable>
              <Menu ref="containerMenuRef" :model="containerMenuItems" :popup="true" />
            </div>
          </template>

          <!-- Images -->
          <template v-else-if="activeCategory === 'images'">
            <header class="flex shrink-0 items-center gap-2 border-b border-divider p-2">
              <span class="text-xs font-bold text-ink">{{ t("docker.tabs.images") }} ({{ docker.images.value.length }})</span>
              <Button class="ml-auto" size="small" @click="buildDialogVisible = true">
                <i class="pi pi-hammer mr-1.5" />{{ t("docker.page.buildImage") }}
              </Button>
            </header>
            <div class="min-h-0 flex-1 overflow-auto p-2">
              <div v-if="docker.loadingImages.value" class="p-6 text-center text-xs text-muted">
                {{ t("common.loading") }}
              </div>
              <div v-else-if="!docker.images.value.length" class="p-6 text-center text-xs text-muted">
                {{ t("common.noData") }}
              </div>
              <div v-else class="flex flex-col gap-2">
                <div
                  v-for="img in docker.images.value"
                  :key="img.id"
                  class="flex items-center gap-3 rounded-md border border-divider bg-canvas p-2"
                >
                  <div class="min-w-0 flex-1">
                    <div class="truncate text-xs font-medium text-ink">{{ img.repository }}:{{ img.tag }}</div>
                    <div class="truncate text-[10px] text-muted">
                      {{ img.id.slice(0, 12) }} · {{ img.size }} · {{ img.created }}
                    </div>
                  </div>
                  <Button
                    class="shrink-0"
                    size="small"
                    text
                    rounded
                    :loading="docker.isBusy(img.id)"
                    icon="pi pi-trash"
                    severity="danger"
                    :title="t('docker.actions.remove')"
                    @click="askRemoveImage(img)"
                  />
                </div>
              </div>
            </div>
          </template>

          <!-- Projects -->
          <template v-else>
            <header class="flex shrink-0 items-center gap-2 border-b border-divider p-2">
              <span class="text-xs font-bold text-ink">{{ t("docker.tabs.projects") }} ({{ docker.projects.value.length }})</span>
              <Button class="ml-auto" size="small" @click="openAddProject">
                <i class="pi pi-plus mr-1.5" />{{ t("docker.page.addProject") }}
              </Button>
            </header>
            <div class="min-h-0 flex-1 overflow-auto p-2">
              <div v-if="docker.loadingProjects.value" class="p-6 text-center text-xs text-muted">
                {{ t("common.loading") }}
              </div>
              <div v-else-if="!docker.projects.value.length" class="p-6 text-center text-xs text-muted">
                {{ t("docker.page.noProjects") }}
              </div>
              <div v-else class="flex flex-col gap-2">
                <div
                  v-for="p in docker.projects.value"
                  :key="p.id"
                  class="flex items-center gap-3 rounded-md border border-divider bg-canvas p-2"
                >
                  <div class="min-w-0 flex-1">
                    <div class="truncate text-xs font-medium text-ink">{{ p.name }}</div>
                    <div class="truncate text-[10px] text-muted">
                      {{ p.kind === "compose" ? p.compose_file : p.context_path }} ·
                      {{ t("docker.columns.lastBuilt") }}: {{ p.last_built || "—" }}
                    </div>
                  </div>
                  <span class="shrink-0 rounded-full bg-brand/10 px-1.5 py-0.5 text-[9px] font-semibold text-brand">
                    {{ p.kind === "compose" ? t("docker.projectDialog.kindCompose") : t("docker.projectDialog.kindDockerfile") }}
                  </span>
                  <div class="flex shrink-0 items-center gap-0.5">
                    <Button
                      size="small"
                      text
                      rounded
                      icon="pi pi-hammer"
                      severity="success"
                      :title="p.kind === 'compose' ? t('docker.actions.composeUp') : t('docker.actions.build')"
                      @click="rebuildProject(p)"
                    />
                    <Button
                      size="small"
                      text
                      rounded
                      icon="pi pi-eraser"
                      severity="warn"
                      :title="t('docker.actions.cleanBuild')"
                      @click="rebuildProject(p, true)"
                    />
                    <Button
                      v-if="p.kind === 'compose'"
                      size="small"
                      text
                      rounded
                      :loading="docker.isBusy(`project-${p.id}`)"
                      icon="pi pi-stop"
                      severity="warn"
                      :title="t('docker.actions.composeDown')"
                      @click="askComposeDown(p)"
                    />
                    <Button
                      size="small"
                      text
                      rounded
                      icon="pi pi-pencil"
                      severity="secondary"
                      :title="t('docker.actions.edit')"
                      @click="openEditProject(p)"
                    />
                    <Button
                      size="small"
                      text
                      rounded
                      icon="pi pi-trash"
                      severity="danger"
                      :title="t('docker.actions.remove')"
                      @click="askRemoveProject(p)"
                    />
                  </div>
                </div>
              </div>
            </div>
          </template>
        </section>
      </div>
    </template>

    <!-- Dialogs -->
    <DockerBuildDialog v-model:visible="buildDialogVisible" @build="onAdHocBuild" />
    <DockerProjectDialog v-model:visible="projectDialogVisible" :docker="docker" :editing="editingProject" />
    <DockerBuildLogDialog v-model:visible="buildLogVisible" :title="buildLogTitle" :lines="buildLogLines" :status="buildLogStatus" />
    <DockerTerminalDialog
      v-if="termTarget"
      v-model:visible="termVisible"
      :container-id="termTarget.id"
      :container-name="containerLabel(termTarget)"
      :mode="termMode"
    />
    <DockerConfirmDialog
      v-model:visible="confirmVisible"
      :title="confirmTitle"
      :confirm-label="confirmLabel"
      :busy="confirmBusy"
      @confirm="runConfirmAction"
    >
      {{ confirmMessage }}
    </DockerConfirmDialog>
  </div>
</template>

<style scoped>
/* PrimeVue render nút thu gọn/mở nhóm và nội dung slot #groupheader là 2 phần tử
   anh em trong cùng 1 <td> — canh giữa theo chiều dọc bằng `vertical-align` (thuộc
   tính hợp lệ của table-cell) thay vì đổi `display` của <td>: đổi display sẽ làm
   nó không còn được coi là table-cell nữa, khiến trình duyệt không giãn nó đủ
   rộng theo colspan → border-bottom bị hụt, không phủ hết các cột đã colspan. */
:deep(tr.p-datatable-row-group-header > td) {
  vertical-align: middle;
}
:deep(tr.p-datatable-row-group-header .p-datatable-row-toggle-button) {
  vertical-align: middle;
}

/* Hover mặc định của PrimeVue tô nền hover riêng cho <tr> và riêng cho nút
   toggle (2 lớp nền chồng nhau, sắc độ khác nhau) — vô hiệu hoá cả 2 lớp hover
   này để hàng group-header luôn hiển thị đồng nhất một màu, không bị "xé" khi
   trỏ chuột vào. */
:deep(tr.p-datatable-row-group-header:hover),
:deep(tr.p-datatable-row-group-header:hover > td),
:deep(tr.p-datatable-row-group-header .p-datatable-row-toggle-button:hover) {
  background: transparent !important;
}
</style>
