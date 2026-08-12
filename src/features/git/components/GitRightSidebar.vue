<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import FileTreePanel from "@/shared/components/FileTreePanel.vue";
import GitRunnerPanel from "./GitRunnerPanel.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

const EXPLORER_KEY = "git.explorerTreeExpanded";
const RUNNER_KEY = "git.runnerExpanded";

const props = defineProps<{
  git: GitApi;
  root: string;
  width: number;
  isResizing: boolean;
}>();

const emit = defineEmits<{ resize: [e: MouseEvent] }>();

const explorerExpanded = ref(localStorage.getItem(EXPLORER_KEY) !== "false");
const runnerExpanded = ref(localStorage.getItem(RUNNER_KEY) !== "false");

const visible = computed(() => (explorerExpanded.value || runnerExpanded.value) && !!props.root);

function toggleExplorer() {
  explorerExpanded.value = !explorerExpanded.value;
  localStorage.setItem(EXPLORER_KEY, String(explorerExpanded.value));
}

function toggleRunner() {
  runnerExpanded.value = !runnerExpanded.value;
  localStorage.setItem(RUNNER_KEY, String(runnerExpanded.value));
}

const panelRef = ref<HTMLElement | null>(null);

defineExpose({ visible, panelRef });
</script>

<template>
  <!-- Expanded sidebar -->
  <template v-if="visible">
    <div
      class="flex w-2 shrink-0 cursor-col-resize items-center justify-center self-stretch hover:bg-sidebar-hover"
      :class="isResizing ? 'bg-sidebar-hover' : ''"
      @mousedown="emit('resize', $event)"
    >
      <div class="h-8 w-0.5 rounded-full bg-sidebar-border" :class="isResizing ? 'bg-brand' : ''" />
    </div>
    <div
      ref="panelRef"
      class="flex shrink-0 flex-col self-stretch overflow-hidden rounded-lg border border-sidebar-border bg-sidebar shadow-md"
      :style="{ width: width + 'px' }"
    >
      <!-- Explorer header -->
      <button
        class="flex shrink-0 items-center gap-2 border-b border-sidebar-border px-3 py-2 text-left transition-colors hover:bg-sidebar-hover"
        @click="toggleExplorer"
      >
        <i class="pi shrink-0 text-[9px] text-sidebar-text" :class="explorerExpanded ? 'pi-chevron-down' : 'pi-chevron-right'" />
        <i class="pi pi-folder shrink-0 text-[11px] text-amber-500" />
        <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide text-sidebar-text">Explorer</span>
      </button>
      <FileTreePanel v-if="explorerExpanded" :root="root" hide-header class="min-h-0 flex-1 border-b border-sidebar-border" />

      <!-- Runner header -->
      <button
        class="flex shrink-0 items-center gap-2 border-b border-sidebar-border px-3 py-2 text-left transition-colors hover:bg-sidebar-hover"
        @click="toggleRunner"
      >
        <i class="pi shrink-0 text-[9px] text-sidebar-text" :class="runnerExpanded ? 'pi-chevron-down' : 'pi-chevron-right'" />
        <i class="pi pi-play shrink-0 text-[11px] text-green-500" />
        <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide text-sidebar-text">Runner</span>
      </button>
      <GitRunnerPanel v-if="runnerExpanded" :git="git" class="min-h-0 flex-1" />
    </div>
  </template>

  <!-- Collapsed bubbles -->
  <div v-else-if="root" class="ml-1 flex shrink-0 flex-col gap-1.5 self-stretch rounded-lg border border-sidebar-border bg-sidebar p-1.5 shadow-md">
    <button
      class="flex items-center justify-center rounded-full p-1.5 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
      :title="t('git.page.expandExplorer')"
      @click="toggleExplorer"
    >
      <i class="pi pi-folder text-sm" />
    </button>
    <button
      class="flex items-center justify-center rounded-full p-1.5 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
      :title="t('git.runner.title')"
      @click="toggleRunner"
    >
      <i class="pi pi-play text-sm" />
    </button>
  </div>
</template>
