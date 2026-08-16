<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Select from "primevue/select";
import Checkbox from "primevue/checkbox";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { TASK_CATEGORY_META } from "@/models/task";
import type { Task } from "@/models/task";
import type { Workflow } from "@/models/workflow";
import type { WorkflowPlanGroup } from "../composables/useWorkspacePlan";

const props = defineProps<{
  linkedTasks: Task[];
  workflows: Workflow[];
  editingGroup: WorkflowPlanGroup | null;
}>();

const visible = defineModel<boolean>("visible", { default: false });

const emit = defineEmits<{
  /** Create mode: workflowId + taskIds[] */
  create: [workflowId: number, taskIds: number[]];
  /** Edit mode: group + newTaskIds[] */
  edit: [group: WorkflowPlanGroup, newTaskIds: number[]];
}>();

const { t } = useI18n();

const isEditMode = computed(() => props.editingGroup !== null);

const selectedWorkflowId = ref<number | null>(null);
const selectedTaskIds = ref<number[]>([]);
const busy = ref(false);

const workflowOptions = computed(() =>
  props.workflows.map((wf) => ({
    label: wf.name,
    value: wf.id,
  })),
);

watch(visible, (v) => {
  if (!v) return;
  busy.value = false;
  if (props.editingGroup) {
    selectedWorkflowId.value = props.editingGroup.workflow.id;
    selectedTaskIds.value = props.editingGroup.taskEntries.map((e) => e.task.id);
  } else {
    selectedWorkflowId.value = null;
    selectedTaskIds.value = [];
  }
});

function toggleTask(taskId: number) {
  const idx = selectedTaskIds.value.indexOf(taskId);
  if (idx === -1) {
    selectedTaskIds.value = [...selectedTaskIds.value, taskId];
  } else {
    selectedTaskIds.value = selectedTaskIds.value.filter((id) => id !== taskId);
  }
}

function isSelected(taskId: number) {
  return selectedTaskIds.value.includes(taskId);
}

const canConfirm = computed(() => {
  if (selectedTaskIds.value.length === 0) return false;
  if (!isEditMode.value && !selectedWorkflowId.value) return false;
  return true;
});

async function confirm() {
  if (!canConfirm.value) return;
  busy.value = true;
  if (isEditMode.value && props.editingGroup) {
    emit("edit", props.editingGroup, selectedTaskIds.value);
  } else if (selectedWorkflowId.value) {
    emit("create", selectedWorkflowId.value, selectedTaskIds.value);
  }
  visible.value = false;
  busy.value = false;
}

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

const selectPt = {
  root: { class: "!bg-panel !border-divider" },
  label: { class: "!text-xs !py-1.5" },
  option: { class: "!text-xs" },
};
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <h3 class="section-title">
        {{ isEditMode ? t("workspaces.plan.dialog.editTitle") : t("workspaces.plan.dialog.title") }}
      </h3>
    </template>

    <div class="space-y-4">
      <!-- Workflow selector (create mode) or read-only display (edit mode) -->
      <div>
        <span class="text-xs font-bold text-muted">
          {{ t("workspaces.plan.dialog.selectWorkflow") }}
          <span v-if="!isEditMode" class="text-red-500">*</span>
        </span>

        <div v-if="isEditMode && editingGroup" class="mt-1 flex items-center gap-2 rounded-md border border-divider bg-canvas px-3 py-2">
          <i :class="[editingGroup.workflow.icon || 'pi pi-sitemap', 'text-sm text-brand']" />
          <span class="text-sm font-medium text-ink">{{ editingGroup.workflow.name }}</span>
          <span v-if="editingGroup.workflow.description" class="truncate text-xs text-muted">
            — {{ editingGroup.workflow.description }}
          </span>
        </div>

        <Select
          v-else
          v-model="selectedWorkflowId"
          :options="workflowOptions"
          option-label="label"
          option-value="value"
          :placeholder="t('workspaces.plan.dialog.workflowPlaceholder')"
          class="mt-1 w-full"
          :pt="selectPt"
        />
      </div>

      <!-- Task checklist -->
      <div>
        <div class="mb-1.5 flex items-center justify-between">
          <span class="text-xs font-bold text-muted">
            {{ t("workspaces.plan.dialog.selectTask") }} <span class="text-red-500">*</span>
          </span>
          <span class="text-[10px] text-muted">
            {{ t("workspaces.plan.dialog.selectedCount", { count: selectedTaskIds.length }) }}
          </span>
        </div>

        <div class="max-h-64 overflow-y-auto rounded-md border border-divider">
          <p v-if="!linkedTasks.length" class="px-3 py-6 text-center text-xs text-muted">
            {{ t("workspaces.plan.noLinkedTasks") }}
          </p>

          <label
            v-for="task in linkedTasks"
            :key="task.id"
            class="flex cursor-pointer items-center gap-2.5 border-b border-divider/60 px-3 py-2 last:border-0 hover:bg-canvas"
          >
            <Checkbox
              :model-value="isSelected(task.id)"
              :binary="true"
              @update:model-value="toggleTask(task.id)"
            />
            <span
              :class="[
                'shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold',
                categoryBadgeClass(task.category_id),
              ]"
            >
              {{ task.category_id || "—" }}
            </span>
            <span class="shrink-0 font-mono text-xs text-ink">{{ task.task_cd }}</span>
            <span class="min-w-0 flex-1 truncate text-xs text-secondary">{{ task.task_name }}</span>
          </label>
        </div>
      </div>
    </div>

    <template #footer>
      <DialogFooter
        :confirm-label="isEditMode ? t('common.save') : t('workspaces.plan.dialog.create')"
        :confirm-icon="isEditMode ? 'pi pi-check' : 'pi pi-plus'"
        :confirm-disabled="!canConfirm"
        :busy="busy"
        @cancel="visible = false"
        @confirm="confirm"
      />
    </template>
  </Dialog>
</template>
