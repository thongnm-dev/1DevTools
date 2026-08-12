<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import InputText from "primevue/inputtext";

import { canUseTauriRuntime } from "@/tauri/commands/_base";
import { detectDevCommands, loadCustomCommands, saveCustomCommands } from "@/tauri/commands/dev_runner";
import { useTerminal } from "@/features/terminal/composables/useTerminal";
import type { DevCommand, CommandCategory } from "@/models/dev_runner";
import type { GitApi } from "../composables/useGit";

import GitRunnerAddDialog from "./GitRunnerAddDialog.vue";

const props = defineProps<{ git: GitApi }>();

const { t } = useI18n();
const router = useRouter();
const term = useTerminal();
const runtimeAvailable = canUseTauriRuntime();

const autoCommands = ref<DevCommand[]>([]);
const customCommands = ref<DevCommand[]>([]);
const loading = ref(false);
const addDialogVisible = ref(false);
const editingCommand = ref<DevCommand | null>(null);
const filterText = ref("");

const CATEGORY_META: Record<CommandCategory, { icon: string; color: string }> = {
  npm: { icon: "pi-box", color: "text-red-500" },
  flutter: { icon: "pi-mobile", color: "text-sky-500" },
  maven: { icon: "pi-server", color: "text-orange-600" },
  gradle: { icon: "pi-server", color: "text-green-600" },
  cargo: { icon: "pi-cog", color: "text-amber-600" },
  go: { icon: "pi-bolt", color: "text-cyan-500" },
  python: { icon: "pi-code", color: "text-yellow-500" },
  dotnet: { icon: "pi-microsoft", color: "text-purple-500" },
  make: { icon: "pi-wrench", color: "text-slate-500" },
  docker: { icon: "pi-box", color: "text-blue-500" },
  custom: { icon: "pi-star", color: "text-brand" },
};

function catMeta(cat: CommandCategory) {
  return CATEGORY_META[cat] ?? CATEGORY_META.custom;
}

const repoPath = computed(() => props.git.info.value?.path || props.git.activeRepo.value?.path || "");

const allCommands = computed(() => [...autoCommands.value, ...customCommands.value]);

const filteredCommands = computed(() => {
  const q = filterText.value.trim().toLowerCase();
  if (!q) return allCommands.value;
  return allCommands.value.filter(
    (c) => c.label.toLowerCase().includes(q) || c.command.toLowerCase().includes(q) || c.category.includes(q),
  );
});

const grouped = computed(() => {
  const map = new Map<string, DevCommand[]>();
  for (const cmd of filteredCommands.value) {
    const key = cmd.source === "custom" ? "custom" : cmd.category;
    if (!map.has(key)) map.set(key, []);
    map.get(key)!.push(cmd);
  }
  return map;
});

async function refresh() {
  const path = repoPath.value;
  if (!path || !runtimeAvailable) return;
  loading.value = true;
  try {
    const [auto, custom] = await Promise.all([detectDevCommands(path), loadCustomCommands(path)]);
    autoCommands.value = auto;
    customCommands.value = custom;
  } catch {
    // silent
  } finally {
    loading.value = false;
  }
}

watch(repoPath, (p) => {
  if (p) void refresh();
});

onMounted(() => {
  if (repoPath.value) void refresh();
});

function runCommand(cmd: DevCommand) {
  const path = repoPath.value;
  if (!path) return;
  const repoName = props.git.activeRepo.value?.name ?? path.split(/[\\/]/).filter(Boolean).pop() ?? "runner";
  term.addTab({ title: `${cmd.label} · ${repoName}`, startDir: path, autoCommand: cmd.command });
  void router.push("/terminal");
}

function openAddDialog() {
  editingCommand.value = null;
  addDialogVisible.value = true;
}

function openEditDialog(cmd: DevCommand) {
  editingCommand.value = cmd;
  addDialogVisible.value = true;
}

async function onSaveCommand(cmd: DevCommand) {
  const path = repoPath.value;
  if (!path) return;

  if (editingCommand.value) {
    customCommands.value = customCommands.value.map((c) => (c.id === editingCommand.value!.id ? cmd : c));
  } else {
    customCommands.value = [...customCommands.value, cmd];
  }
  await saveCustomCommands(path, customCommands.value);
}

async function removeCustomCommand(cmd: DevCommand) {
  const path = repoPath.value;
  if (!path) return;
  customCommands.value = customCommands.value.filter((c) => c.id !== cmd.id);
  await saveCustomCommands(path, customCommands.value);
}
</script>

<template>
  <div class="flex h-full flex-col bg-sidebar text-sidebar-text">
    <!-- Toolbar -->
    <div class="flex items-center gap-1 px-2 py-1.5">
      <div class="relative flex-1">
        <i class="pi pi-search pointer-events-none absolute left-2 top-1/2 -translate-y-1/2 text-[10px] text-sidebar-text" />
        <InputText
          v-model="filterText"
          class="h-6 w-full !pl-6 text-[11px]"
          :placeholder="t('git.runner.filterPlaceholder')"
        />
      </div>
      <button
        class="rounded p-1 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
        :title="t('git.tabs.refresh')"
        @click="refresh"
      >
        <i v-if="loading" class="pi pi-spinner pi-spin text-[10px]" />
        <i v-else class="pi pi-sync text-[10px]" />
      </button>
      <button
        class="rounded p-1 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
        :title="t('git.runner.addCommand')"
        @click="openAddDialog"
      >
        <i class="pi pi-plus text-[10px]" />
      </button>
    </div>

    <!-- Command list -->
    <div class="min-h-0 flex-1 overflow-y-auto px-1 pb-1">
      <div v-if="loading && !allCommands.length" class="p-4 text-center text-[11px] text-sidebar-text">
        <i class="pi pi-spinner pi-spin mr-1" /> {{ t("common.loading") }}
      </div>
      <div v-else-if="!allCommands.length" class="flex flex-col items-center gap-2 p-4 text-center">
        <i class="pi pi-play-circle text-xl text-sidebar-text" />
        <p class="text-[11px] text-sidebar-text">{{ t("git.runner.empty") }}</p>
        <button
          class="rounded px-2 py-1 text-[11px] font-medium text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
          @click="openAddDialog"
        >
          <i class="pi pi-plus mr-1 text-[10px]" />{{ t("git.runner.addCommand") }}
        </button>
      </div>

      <template v-else>
        <div v-for="[group, cmds] in grouped" :key="group" class="mb-2 last:mb-0">
          <!-- Group header -->
          <div class="mb-0.5 flex items-center gap-1.5 px-2 py-0.5">
            <i class="pi text-[10px]" :class="[catMeta(group as CommandCategory).icon, catMeta(group as CommandCategory).color]" />
            <span class="text-[10px] font-bold uppercase tracking-wide text-sidebar-text">
              {{ group === "custom" ? t("git.runner.customGroup") : group }}
            </span>
            <span class="text-[9px] text-sidebar-text">({{ cmds.length }})</span>
          </div>

          <!-- Commands -->
          <div class="flex flex-col gap-0.5">
            <div
              v-for="cmd in cmds"
              :key="cmd.id"
              class="group flex items-center gap-1.5 rounded-md px-2 py-1.5 transition-colors hover:bg-sidebar-hover"
            >
              <div class="min-w-0 flex-1">
                <div class="truncate text-[11px] font-medium text-sidebar-text-active">{{ cmd.label }}</div>
                <div class="truncate font-mono text-[10px] text-sidebar-text">{{ cmd.command }}</div>
              </div>

              <div class="flex shrink-0 items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100">
                <button
                  class="rounded p-0.5 text-green-500 transition-colors hover:bg-sidebar-hover"
                  :title="t('git.runner.run')"
                  @click="runCommand(cmd)"
                >
                  <i class="pi pi-play text-[10px]" />
                </button>
                <template v-if="cmd.source === 'custom'">
                  <button
                    class="rounded p-0.5 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
                    :title="t('git.runner.edit')"
                    @click="openEditDialog(cmd)"
                  >
                    <i class="pi pi-pencil text-[10px]" />
                  </button>
                  <button
                    class="rounded p-0.5 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-red-500"
                    :title="t('git.runner.remove')"
                    @click="removeCustomCommand(cmd)"
                  >
                    <i class="pi pi-trash text-[10px]" />
                  </button>
                </template>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>

    <!-- Add/Edit dialog -->
    <GitRunnerAddDialog
      v-model:visible="addDialogVisible"
      :editing="editingCommand"
      @save="onSaveCommand"
    />
  </div>
</template>
