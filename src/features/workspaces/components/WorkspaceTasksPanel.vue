<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import TaskPickerDialog from "@/features/task/components/TaskPickerDialog.vue";
import type { Workspace } from "@/models/workspace";
import { TASK_CATEGORY_META, STEP_STATUS_META } from "@/models/task";
import type { Task } from "@/models/task";
import { useWorkspaceTaskLinks } from "../composables/useWorkspaceTaskLinks";

const props = defineProps<{ workspace: Workspace }>();
const { t } = useI18n();

const linksCtrl = useWorkspaceTaskLinks(props.workspace);

const showPickerDialog = ref(false);
const excludeIds = computed(() => linksCtrl.linkedTasks.value.map((task) => task.id));

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

function stepStatusBadgeClass(status: string): string {
  return STEP_STATUS_META[status as keyof typeof STEP_STATUS_META]?.badgeClass ?? "bg-canvas text-muted";
}

function pickTask(task: Task) {
  void linksCtrl.addTask(task);
}
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-panel text-ink">
    <!-- Header -->
    <div class="flex shrink-0 items-center justify-between gap-2 border-b border-divider px-4 py-3">
      <div class="flex items-center gap-2">
        <h3 class="section-title">{{ t("workspaces.tasks.title") }}</h3>
        <span class="text-xs text-muted">{{ linksCtrl.linkedTasks.value.length }}</span>
      </div>
      <Button icon="pi pi-plus" size="small" :label="t('workspaces.tasks.addTask')" @click="showPickerDialog = true" />
    </div>

    <p v-if="linksCtrl.error.value" class="banner-danger mx-4 mt-3">{{ linksCtrl.error.value }}</p>

    <!-- Task list -->
    <div class="min-h-0 flex-1 overflow-y-auto p-3">
      <p v-if="!linksCtrl.linkedTasks.value.length" class="px-3 py-8 text-center text-xs text-muted">
        {{ t("workspaces.tasks.empty") }}
      </p>

      <div
        v-for="task in linksCtrl.linkedTasks.value"
        :key="task.id"
        class="mb-2 flex items-center gap-2 rounded-md border border-divider bg-canvas px-3 py-2"
      >
        <span :class="['shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold', categoryBadgeClass(task.category_id)]">
          {{ task.category_id || "—" }}
        </span>

        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2">
            <span class="font-mono text-xs text-ink">{{ task.task_cd }}</span>
            <span class="min-w-0 truncate text-xs text-secondary">{{ task.task_name }}</span>
          </div>
          <div v-if="task.current_wf_name" class="mt-0.5 flex items-center gap-1.5 text-[11px]">
            <span class="truncate text-muted">{{ task.current_wf_name }} › {{ task.current_step_name }}</span>
            <span
              :class="['shrink-0 rounded-full px-1.5 py-0.5 text-[9px] font-bold', stepStatusBadgeClass(task.current_step_status)]"
            >
              {{ task.current_step_status }}
            </span>
          </div>
        </div>

        <Button
          icon="pi pi-trash"
          text
          rounded
          size="small"
          severity="danger"
          :title="t('workspaces.tasks.unlink')"
          @click="linksCtrl.removeTask(task.id)"
        />
      </div>
    </div>

    <TaskPickerDialog v-model:visible="showPickerDialog" :exclude-ids="excludeIds" @pick="pickTask" />
  </div>
</template>
