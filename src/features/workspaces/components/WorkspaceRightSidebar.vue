<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Workspace, WorkspaceMainPanel } from "@/models/workspace";

const { t } = useI18n();

const props = defineProps<{ activePanel: WorkspaceMainPanel; workspace: Workspace }>();
const emit = defineEmits<{
  select: [panel: WorkspaceMainPanel];
  edit: [];
  delete: [];
}>();

function itemClass(panel: WorkspaceMainPanel, active: WorkspaceMainPanel) {
  return panel === active
    ? "bg-sidebar-active text-sidebar-text-active"
    : "text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-active";
}

// === Cog menu ===
const cogOpen = ref(false);
const cogTriggerRef = ref<HTMLButtonElement | null>(null);
const cogMenuStyle = ref({ bottom: "0px", right: "0px" });

function toggleCog() {
  if (!cogOpen.value) {
    const rect = cogTriggerRef.value?.getBoundingClientRect();
    if (rect) {
      cogMenuStyle.value = {
        bottom: `${window.innerHeight - rect.top + 4}px`,
        right: `${window.innerWidth - rect.right}px`,
      };
    }
  }
  cogOpen.value = !cogOpen.value;
}

function cogAction(name: "edit" | "delete") {
  cogOpen.value = false;
  emit(name);
}

function onClickAway(e: MouseEvent) {
  if (!cogTriggerRef.value?.contains(e.target as Node)) {
    cogOpen.value = false;
  }
}

watch(cogOpen, (val) => {
  if (val) {
    requestAnimationFrame(() => document.addEventListener("click", onClickAway));
  } else {
    document.removeEventListener("click", onClickAway);
  }
});
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
    <button
      class="flex items-center justify-center rounded-md p-2 transition-colors"
      :class="itemClass('tasks', activePanel)"
      :title="t('workspaces.sidebar.tasks')"
      @click="emit('select', 'tasks')"
    >
      <i class="pi pi-list-check text-sm" />
    </button>

    <!-- Cog button pinned to bottom -->
    <div class="mt-auto">
      <div class="my-0.5 h-px w-6 bg-sidebar-border" />
      <button
        ref="cogTriggerRef"
        class="flex items-center justify-center rounded-md p-2 text-sidebar-text transition-colors hover:bg-sidebar-hover hover:text-sidebar-text-active"
        :class="cogOpen ? 'bg-sidebar-active text-sidebar-text-active' : ''"
        :title="t('workspaces.settings')"
        @click="toggleCog"
      >
        <i class="pi pi-cog text-sm" />
      </button>
    </div>

    <Teleport to="body">
      <div
        v-if="cogOpen"
        class="fixed z-50 w-48 rounded-lg border border-divider bg-panel p-1 shadow-float"
        :style="cogMenuStyle"
        @click.stop
      >
        <button
          class="ctx-menu-item"
          @click="cogAction('edit')"
        >
          <i class="pi pi-pencil text-xs" /> {{ t("workspaces.edit") }}
        </button>
        <div class="my-1 border-t border-divider" />
        <button
          class="ctx-menu-item-danger"
          @click="cogAction('delete')"
        >
          <i class="pi pi-trash text-xs" /> {{ t("workspaces.deleteWorkspace") }}
        </button>
      </div>
    </Teleport>
  </div>
</template>
