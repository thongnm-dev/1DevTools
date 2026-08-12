<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import FileTreePanel from "@/shared/components/FileTreePanel.vue";
import GitRunnerPanel from "./GitRunnerPanel.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

type PanelId = "explorer" | "runner";
const ACTIVE_KEY = "git.rightSidebarPanel";

const props = defineProps<{
  git: GitApi;
  root: string;
  width: number;
  isResizing: boolean;
}>();

const emit = defineEmits<{ resize: [e: MouseEvent] }>();

const stored = localStorage.getItem(ACTIVE_KEY);
const activePanel = ref<PanelId | null>(stored === "explorer" || stored === "runner" ? stored : "explorer");

const panelVisible = computed(() => activePanel.value !== null && !!props.root);

function toggle(panel: PanelId) {
  activePanel.value = activePanel.value === panel ? null : panel;
  localStorage.setItem(ACTIVE_KEY, activePanel.value ?? "");
}

const panelRef = ref<HTMLElement | null>(null);

defineExpose({ visible: panelVisible, panelRef });
</script>

<template>
  <template v-if="root">
    <!-- Resize handle (only when panel open) -->
    <div
      v-if="panelVisible"
      class="flex w-2 shrink-0 cursor-col-resize items-center justify-center self-stretch hover:bg-sidebar-hover"
      :class="isResizing ? 'bg-sidebar-hover' : ''"
      @mousedown="emit('resize', $event)"
    >
      <div class="h-8 w-0.5 rounded-full bg-sidebar-border" :class="isResizing ? 'bg-brand' : ''" />
    </div>

    <!-- Panel -->
    <div
      v-if="panelVisible"
      ref="panelRef"
      class="flex shrink-0 flex-col self-stretch overflow-hidden rounded-l-lg border border-sidebar-border bg-sidebar shadow-md"
      :style="{ width: width + 'px' }"
    >
      <FileTreePanel v-if="activePanel === 'explorer'" :root="root" class="min-h-0 flex-1" />
      <GitRunnerPanel v-else-if="activePanel === 'runner'" :git="git" class="min-h-0 flex-1" />
    </div>

    <!-- Activity bar (always visible) -->
    <div :class="['flex shrink-0 flex-col items-center gap-0.5 self-stretch border border-sidebar-border bg-sidebar shadow-md', panelVisible ? 'rounded-r-lg' : 'ml-0.5 rounded-lg']">
      <button
        class="flex items-center justify-center rounded-md p-2 transition-colors"
        :class="activePanel === 'explorer' ? 'bg-sidebar-active text-sidebar-text-active' : 'text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-active'"
        :title="t('git.page.expandExplorer')"
        @click="toggle('explorer')"
      >
        <i class="pi pi-folder text-sm" />
      </button>
      <button
        class="flex items-center justify-center rounded-md p-2 transition-colors"
        :class="activePanel === 'runner' ? 'bg-sidebar-active text-sidebar-text-active' : 'text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-active'"
        :title="t('git.runner.title')"
        @click="toggle('runner')"
      >
        <i class="pi pi-play text-sm" />
      </button>
    </div>
  </template>
</template>
