<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputGroup from "primevue/inputgroup";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import Tab from "primevue/tab";
import TabList from "primevue/tablist";
import Tabs from "primevue/tabs";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useWorkspace } from "../composables/useWorkspace";
import { useTerminal } from "@/features/terminal/composables/useTerminal";
import { useToast } from "@/shared/composables/useToast";
import { friendlyError } from "@/tauri/commands/_base";
import { explorerOpen } from "@/tauri/commands/explorer";
import { gitOpenVscode } from "@/tauri/commands/git";
import type { GitRepo } from "@/models/git";
import type { Workspace } from "@/models/workspace";
import { DEFAULT_WORKSPACE_ICON } from "@/models/workspace";

const ACTIVE_REPO_KEY = "git.activeRepoId";

const { t } = useI18n();
const router = useRouter();
const term = useTerminal();
const toast = useToast();
const ctrl = useWorkspace();

// --- New/Edit workspace dialog (shared fields, only one open at a time) ---
const showWorkspaceDialog = ref(false);
const editingWorkspaceId = ref<number | null>(null);
const selectedRepoId = ref<number | null>(null);
const wsName = ref("");
const wsIcon = ref(DEFAULT_WORKSPACE_ICON);
const showWorkspaceIconPicker = ref(false);

const repoOptions = computed(() => ctrl.gitRepos.value.map((r) => ({ label: r.name, value: r.id })));

function openNewWorkspaceDialog() {
  editingWorkspaceId.value = null;
  selectedRepoId.value = null;
  wsName.value = "";
  wsIcon.value = DEFAULT_WORKSPACE_ICON;
  showWorkspaceDialog.value = true;
}

function openEditWorkspaceDialog(ws: Workspace) {
  editingWorkspaceId.value = ws.id;
  selectedRepoId.value = null;
  wsName.value = ws.name;
  wsIcon.value = ws.icon;
  showWorkspaceDialog.value = true;
}

function onRepoSelected() {
  const repo = ctrl.gitRepos.value.find((r) => r.id === selectedRepoId.value);
  if (repo) wsName.value = repo.name;
}

async function browseForFolder() {
  const repo = await ctrl.pickFolder();
  if (!repo) return;
  selectedRepoId.value = repo.id;
  wsName.value = repo.name;
}

function onTabChange(value: string | number) {
  ctrl.selectWorkspace(Number(value));
}

async function saveWorkspace() {
  const name = wsName.value.trim();
  if (!name) return;
  if (editingWorkspaceId.value !== null) {
    await ctrl.updateWorkspace(editingWorkspaceId.value, { name, icon: wsIcon.value });
  } else {
    const repo = ctrl.gitRepos.value.find((r: GitRepo) => r.id === selectedRepoId.value);
    if (!repo) return;
    await ctrl.createFromRepo(repo, name, wsIcon.value);
  }
  showWorkspaceDialog.value = false;
}

// --- Quick actions (fire-and-forget helpers to existing tools; not yet embedded — see Phase 2) ---
function repoIdFor(ws: Workspace): number | null {
  const repo = ctrl.gitRepos.value.find((r) => r.path === ws.project_path);
  return repo?.id ?? null;
}

function openEmbeddedTerminal(ws: Workspace) {
  term.addTab({ title: ws.name, startDir: ws.project_path });
  void router.push("/terminal");
}

async function openInGitDesktop(ws: Workspace) {
  const id = repoIdFor(ws);
  if (id !== null) localStorage.setItem(ACTIVE_REPO_KEY, String(id));
  await router.push("/git");
}

async function openInVscode(ws: Workspace) {
  try {
    await gitOpenVscode(ws.project_path);
  } catch (e) {
    toast.error(friendlyError(e));
  }
}

async function showInExplorer(ws: Workspace) {
  try {
    await explorerOpen(ws.project_path);
  } catch (e) {
    toast.error(friendlyError(e));
  }
}

const selectPt = {
  root: { class: "!bg-panel !border-divider" },
  label: { class: "!flex !items-center !text-xs !py-1.5 !text-ink" },
  option: { class: "!text-xs" },
};
</script>

<template>
  <div class="flex flex-1 flex-col gap-3 overflow-hidden">
    <!-- Tab bar -->
    <div class="flex shrink-0 items-center gap-1 rounded-lg border border-divider bg-panel p-1.5 shadow-sm">
      <Tabs :value="ctrl.activeId.value ?? -1" scrollable class="min-w-0 flex-1" @update:value="onTabChange">
        <TabList>
          <Tab v-for="ws in ctrl.workspaces.value" :key="ws.id" :value="ws.id" as="div" class="cursor-pointer">
            <span class="flex items-center gap-2">
              <i :class="ws.icon" />
              <span class="max-w-[140px] truncate text-xs font-bold">{{ ws.name }}</span>
              <Button
                icon="pi pi-times"
                text
                rounded
                size="small"
                class="!h-5 !w-5"
                :title="t('workspaces.close')"
                @click.stop="ctrl.removeWorkspace(ws.id)"
              />
            </span>
          </Tab>
        </TabList>
      </Tabs>
      <Button icon="pi pi-plus" text rounded size="small" :title="t('workspaces.newWorkspace')" @click="openNewWorkspaceDialog" />
    </div>

    <!-- Empty state -->
    <div v-if="ctrl.workspaces.value.length === 0" class="flex flex-1 items-center justify-center rounded-lg border border-dashed border-divider bg-panel/50 p-12">
      <div class="text-center">
        <i class="pi pi-th-large text-4xl text-muted/60" />
        <p class="mt-2 text-sm text-muted">{{ t("workspaces.emptyState") }}</p>
        <Button icon="pi pi-plus" :label="t('workspaces.newWorkspace')" class="mt-4" @click="openNewWorkspaceDialog" />
      </div>
    </div>

    <!-- Active workspace -->
    <div v-else-if="ctrl.activeWorkspace.value" class="flex flex-1 flex-col gap-4 overflow-auto">
      <div class="shrink-0 rounded-lg border border-divider bg-panel p-6 shadow-sm">
        <div class="flex flex-wrap items-center gap-3">
          <i :class="[ctrl.activeWorkspace.value.icon, 'text-2xl text-muted']" />
          <div class="min-w-0">
            <h2 class="page-title">{{ ctrl.activeWorkspace.value.name }}</h2>
            <p class="truncate text-sm text-muted">{{ ctrl.activeWorkspace.value.project_path }}</p>
          </div>
          <div class="ml-auto flex shrink-0 items-center gap-2">
            <Button icon="pi pi-pencil" :label="t('workspaces.edit')" severity="secondary" size="small" @click="openEditWorkspaceDialog(ctrl.activeWorkspace.value)" />
            <Button icon="pi pi-times" :label="t('workspaces.close')" severity="danger" text size="small" @click="ctrl.removeWorkspace(ctrl.activeWorkspace.value.id)" />
          </div>
        </div>
      </div>

      <div class="grid shrink-0 grid-cols-2 gap-3 sm:grid-cols-4">
        <button
          class="flex flex-col items-center gap-2 rounded-lg border border-divider bg-panel p-4 shadow-sm transition-shadow hover:shadow-float"
          @click="openEmbeddedTerminal(ctrl.activeWorkspace.value)"
        >
          <i class="pi pi-desktop text-xl text-brand" />
          <span class="text-xs font-bold text-ink">{{ t("workspaces.action.terminal") }}</span>
        </button>
        <button
          class="flex flex-col items-center gap-2 rounded-lg border border-divider bg-panel p-4 shadow-sm transition-shadow hover:shadow-float"
          @click="openInGitDesktop(ctrl.activeWorkspace.value)"
        >
          <i class="pi pi-github text-xl text-brand" />
          <span class="text-xs font-bold text-ink">{{ t("workspaces.action.git") }}</span>
        </button>
        <button
          class="flex flex-col items-center gap-2 rounded-lg border border-divider bg-panel p-4 shadow-sm transition-shadow hover:shadow-float"
          @click="openInVscode(ctrl.activeWorkspace.value)"
        >
          <i class="pi pi-code text-xl text-brand" />
          <span class="text-xs font-bold text-ink">{{ t("workspaces.action.vscode") }}</span>
        </button>
        <button
          class="flex flex-col items-center gap-2 rounded-lg border border-divider bg-panel p-4 shadow-sm transition-shadow hover:shadow-float"
          @click="showInExplorer(ctrl.activeWorkspace.value)"
        >
          <i class="pi pi-folder-open text-xl text-brand" />
          <span class="text-xs font-bold text-ink">{{ t("workspaces.action.explorer") }}</span>
        </button>
      </div>

      <div class="flex-1 rounded-lg border border-dashed border-divider bg-panel/50 p-6 text-center text-sm text-muted">
        {{ t("workspaces.comingSoon") }}
      </div>
    </div>

    <!-- New/Edit Workspace Dialog -->
    <Dialog
      :visible="showWorkspaceDialog"
      class="w-full max-w-md rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showWorkspaceDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ editingWorkspaceId !== null ? t("workspaces.dialog.editTitle") : t("workspaces.dialog.newTitle") }}</h3>
      </template>

      <div class="space-y-4">
        <div v-if="editingWorkspaceId === null" class="block">
          <span class="text-xs font-bold text-muted">{{ t("workspaces.dialog.project") }} <span class="text-red-500">*</span></span>
          <InputGroup class="mt-1">
            <Select
              v-model="selectedRepoId"
              :options="repoOptions"
              optionLabel="label"
              optionValue="value"
              :placeholder="t('workspaces.dialog.projectPlaceholder')"
              class="min-w-0 flex-1"
              :pt="selectPt"
              @change="onRepoSelected"
            />
            <Button icon="pi pi-folder-open" severity="secondary" outlined :title="t('workspaces.dialog.browse')" @click="browseForFolder" />
          </InputGroup>
        </div>

        <div class="flex items-end gap-3">
          <label class="block min-w-0 flex-1">
            <span class="text-xs font-bold text-muted">{{ t("workspaces.dialog.name") }} <span class="text-red-500">*</span></span>
            <InputText v-model="wsName" class="mt-1 w-full" :placeholder="t('workspaces.dialog.namePlaceholder')" autofocus />
          </label>
          <div class="block">
            <span class="text-xs font-bold text-muted">{{ t("workspaces.dialog.icon") }}</span>
            <div class="mt-1 flex items-center gap-2">
              <div class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3">
                <i :class="[wsIcon, 'text-brand']" />
                <InputText
                  v-model="wsIcon"
                  class="embedded-input w-24 border-0 !bg-transparent !p-0 !text-sm"
                  placeholder="pi pi-folder"
                />
              </div>
              <Button icon="pi pi-th-large" severity="secondary" outlined :title="t('workspaces.dialog.browseIcons')" @click="showWorkspaceIconPicker = true" />
            </div>
          </div>
        </div>
      </div>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="editingWorkspaceId !== null ? t('common.save') : t('workspaces.dialog.create')"
          :confirm-icon="editingWorkspaceId !== null ? 'pi pi-check' : 'pi pi-plus'"
          :confirm-disabled="!wsName.trim() || (editingWorkspaceId === null && selectedRepoId === null)"
          @cancel="showWorkspaceDialog = false"
          @confirm="saveWorkspace"
        />
      </template>
    </Dialog>

    <!-- Workspace Icon Picker Dialog -->
    <IconPickerDialog
      :visible="showWorkspaceIconPicker"
      :selected="wsIcon"
      @update:visible="showWorkspaceIconPicker = $event"
      @select="(icon: string) => (wsIcon = 'pi ' + icon)"
    />
  </div>
</template>
