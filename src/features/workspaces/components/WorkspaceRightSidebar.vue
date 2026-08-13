<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { WorkspaceMainPanel } from "@/models/workspace";

const { t } = useI18n();

defineProps<{ activePanel: WorkspaceMainPanel }>();
const emit = defineEmits<{ select: [panel: WorkspaceMainPanel] }>();

function itemClass(panel: WorkspaceMainPanel, active: WorkspaceMainPanel) {
  return panel === active
    ? "bg-sidebar-active text-sidebar-text-active"
    : "text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-active";
}
</script>

<template>
  <div class="ml-0.5 flex shrink-0 flex-col items-center gap-0.5 self-stretch rounded-lg border border-sidebar-border bg-sidebar p-0.5 shadow-md">
    <button
      class="flex items-center justify-center rounded-md p-2 transition-colors"
      :class="itemClass('git', activePanel)"
      :title="t('workspaces.action.git')"
      @click="emit('select', 'git')"
    >
      <i class="pi pi-github text-sm" />
    </button>

    <div class="my-0.5 h-px w-6 bg-sidebar-border" />

    <button
      class="flex items-center justify-center rounded-md p-2 transition-colors"
      :class="itemClass('terminal', activePanel)"
      :title="t('workspaces.sidebar.terminal')"
      @click="emit('select', 'terminal')"
    >
      <i class="pi pi-desktop text-sm" />
    </button>
    <button
      class="flex items-center justify-center rounded-md p-2 transition-colors"
      :class="itemClass('agents', activePanel)"
      :title="t('workspaces.sidebar.agents')"
      @click="emit('select', 'agents')"
    >
      <i class="pi pi-microchip-ai text-sm" />
    </button>
    <button
      class="flex items-center justify-center rounded-md p-2 transition-colors"
      :class="itemClass('ide', activePanel)"
      :title="t('workspaces.sidebar.ide')"
      @click="emit('select', 'ide')"
    >
      <i class="pi pi-code text-sm" />
    </button>
  </div>
</template>
