<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Select from "primevue/select";
import WorkspacePlanDialog from "./WorkspacePlanDialog.vue";
import type { Workspace } from "@/models/workspace";
import { TASK_CATEGORY_META, STEP_STATUS_META } from "@/models/task";
import { STEP_TYPE_META } from "@/models/workflow";
import { useWorkspacePlan } from "../composables/useWorkspacePlan";
import type { WorkflowPlanGroup, TaskPlanEntry } from "../composables/useWorkspacePlan";

const props = defineProps<{ workspace: Workspace }>();
const { t } = useI18n();

const ctrl = useWorkspacePlan(props.workspace);

// === Dialog ===
const showDialog = ref(false);
const editingGroup = ref<WorkflowPlanGroup | null>(null);

function openCreate() {
  editingGroup.value = null;
  showDialog.value = true;
}

function openEdit(group: WorkflowPlanGroup) {
  editingGroup.value = group;
  showDialog.value = true;
}

async function onDialogCreate(workflowId: number, taskIds: number[]) {
  await ctrl.createPlan(workflowId, taskIds);
}

async function onDialogEdit(group: WorkflowPlanGroup, newTaskIds: number[]) {
  await ctrl.updatePlanTasks(group, newTaskIds);
}

// === Expand / collapse ===
const expandedProcs = ref(new Set<number>());

function toggleProc(procId: number) {
  if (expandedProcs.value.has(procId)) {
    expandedProcs.value.delete(procId);
  } else {
    expandedProcs.value.add(procId);
  }
}

// === Step status ===
const STATUS_OPTIONS = (["pending", "in_progress", "completed", "skipped"] as const).map((s) => ({
  label: t(`workspaces.plan.step.${s}`),
  value: s,
}));

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

function stepStatusBadgeClass(status: string): string {
  return STEP_STATUS_META[status as keyof typeof STEP_STATUS_META]?.badgeClass ?? "bg-canvas text-muted";
}

function progressPercent(steps: { status: string }[]): number {
  if (!steps.length) return 0;
  const done = steps.filter((s) => s.status === "completed" || s.status === "skipped").length;
  return Math.round((done / steps.length) * 100);
}

function doneCount(steps: { status: string }[]): number {
  return steps.filter((s) => s.status === "completed" || s.status === "skipped").length;
}

// === Run helpers ===
function isEntryRunning(entry: TaskPlanEntry): boolean {
  return ctrl.runningProcIds.value.has(entry.proc.id);
}

function isGroupRunning(group: WorkflowPlanGroup): boolean {
  return ctrl.runningWfIds.value.has(group.workflow.id);
}

const selectPt = {
  root: { class: "!bg-canvas !border-divider" },
  label: { class: "!flex !items-center !text-[10px] !py-0.5 !px-1.5 !text-ink" },
  option: { class: "!text-xs" },
  overlay: { class: "!z-50" },
};
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-panel text-ink">
    <!-- Header -->
    <div class="flex shrink-0 items-center justify-between gap-2 border-b border-divider px-4 py-3">
      <div class="flex items-center gap-2">
        <h3 class="section-title">{{ t("workspaces.plan.title") }}</h3>
        <span class="text-xs text-muted">{{ ctrl.groupedPlans.value.length }}</span>
      </div>
      <div class="flex items-center gap-1.5">
        <Button
          icon="pi pi-refresh"
          text
          rounded
          size="small"
          severity="secondary"
          :loading="ctrl.loading.value"
          :title="t('common.refresh')"
          @click="ctrl.refresh()"
        />
        <Button
          icon="pi pi-plus"
          size="small"
          :label="t('workspaces.plan.addPlan')"
          :disabled="!ctrl.linkedTasks.value.length || !ctrl.workflows.value.length"
          @click="openCreate"
        />
      </div>
    </div>

    <p v-if="ctrl.error.value" class="banner-danger mx-4 mt-3">{{ ctrl.error.value }}</p>

    <!-- Empty: no linked tasks -->
    <div
      v-if="!ctrl.loading.value && !ctrl.linkedTasks.value.length"
      class="flex flex-1 items-center justify-center p-8 text-center"
    >
      <div class="space-y-2">
        <i class="pi pi-link text-2xl text-muted" />
        <p class="text-xs text-muted">{{ t("workspaces.plan.noLinkedTasks") }}</p>
      </div>
    </div>

    <!-- Empty: has tasks but no plans -->
    <div
      v-else-if="!ctrl.loading.value && !ctrl.groupedPlans.value.length"
      class="flex flex-1 items-center justify-center p-8 text-center"
    >
      <div class="space-y-2">
        <i class="pi pi-sitemap text-2xl text-muted" />
        <p class="text-xs text-muted">{{ t("workspaces.plan.empty") }}</p>
      </div>
    </div>

    <!-- Loading -->
    <div v-else-if="ctrl.loading.value" class="flex flex-1 items-center justify-center">
      <i class="pi pi-spin pi-spinner text-muted" />
    </div>

    <!-- Workflow groups -->
    <div v-else class="min-h-0 flex-1 space-y-3 overflow-y-auto p-3">
      <div
        v-for="group in ctrl.groupedPlans.value"
        :key="group.workflow.id"
        class="overflow-hidden rounded-lg border border-divider bg-canvas"
      >
        <!-- Workflow header -->
        <div class="flex items-center gap-2 border-b border-divider bg-panel px-3 py-2.5">
          <i :class="[group.workflow.icon || 'pi pi-sitemap', 'shrink-0 text-sm text-brand']" />
          <div class="min-w-0 flex-1">
            <span class="block truncate text-sm font-semibold text-ink">{{ group.workflow.name }}</span>
            <span v-if="group.workflow.description" class="block truncate text-[11px] text-muted">
              {{ group.workflow.description }}
            </span>
          </div>
          <span class="shrink-0 rounded-full bg-brand/10 px-2 py-0.5 text-[10px] font-bold text-brand">
            {{ group.taskEntries.length }} {{ t("workspaces.plan.tasks") }}
          </span>
          <!-- Run All -->
          <Button
            icon="pi pi-forward"
            text
            rounded
            size="small"
            :loading="isGroupRunning(group)"
            :title="t('workspaces.plan.run.runAll')"
            @click.stop="ctrl.runAllPlans(group)"
          />
          <!-- Edit -->
          <Button
            icon="pi pi-pencil"
            text
            rounded
            size="small"
            severity="secondary"
            :title="t('workspaces.plan.edit')"
            @click.stop="openEdit(group)"
          />
        </div>

        <!-- Task list -->
        <div class="divide-y divide-divider">
          <div v-for="entry in group.taskEntries" :key="entry.proc.id">
            <!-- Task row -->
            <div class="flex items-center">
              <button
                class="flex min-w-0 flex-1 items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-panel/60"
                @click="toggleProc(entry.proc.id)"
              >
                <i
                  :class="[
                    'pi shrink-0 text-[10px] text-muted transition-transform',
                    expandedProcs.has(entry.proc.id) ? 'pi-chevron-down' : 'pi-chevron-right',
                  ]"
                />
                <span
                  :class="[
                    'shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold',
                    categoryBadgeClass(entry.task.category_id),
                  ]"
                >
                  {{ entry.task.category_id || "—" }}
                </span>
                <span class="shrink-0 font-mono text-xs text-ink">{{ entry.task.task_cd }}</span>
                <span class="min-w-0 flex-1 truncate text-xs text-secondary">{{ entry.task.task_name }}</span>
                <div class="flex shrink-0 items-center gap-2">
                  <div class="h-1.5 w-20 overflow-hidden rounded-full bg-divider">
                    <div
                      class="h-1.5 rounded-full bg-brand transition-all duration-300"
                      :style="{ width: progressPercent(entry.steps) + '%' }"
                    />
                  </div>
                  <span class="w-14 text-right text-[10px] text-muted">
                    {{ doneCount(entry.steps) }}/{{ entry.steps.length }} {{ t("workspaces.plan.steps") }}
                  </span>
                </div>
              </button>

              <!-- Run task -->
              <Button
                icon="pi pi-play"
                text
                rounded
                size="small"
                :loading="isEntryRunning(entry)"
                :disabled="ctrl.isEntryDone(entry)"
                :title="t('workspaces.plan.run.title')"
                class="shrink-0"
                @click.stop="ctrl.runPlan(entry)"
              />
              <!-- Delete task from plan -->
              <Button
                icon="pi pi-trash"
                text
                rounded
                size="small"
                severity="danger"
                class="mr-1 shrink-0"
                :title="t('workspaces.plan.delete')"
                @click.stop="ctrl.deletePlan(entry.proc.id)"
              />
            </div>

            <!-- Steps (expanded) -->
            <div v-if="expandedProcs.has(entry.proc.id)" class="border-t border-divider bg-panel/40">
              <div
                v-for="step in entry.steps"
                :key="step.id"
                class="flex items-center gap-2 border-b border-divider/50 px-4 py-1.5 last:border-0"
              >
                <span class="w-4 shrink-0 text-center text-[10px] text-muted">{{ step.order + 1 }}</span>
                <i
                  :class="[
                    STEP_TYPE_META[step.stepType as keyof typeof STEP_TYPE_META]?.icon ?? 'pi pi-circle',
                    'shrink-0 text-[11px] text-muted',
                  ]"
                />
                <div class="min-w-0 flex-1">
                  <span class="block truncate text-xs text-ink">{{ step.name }}</span>
                  <span v-if="step.description" class="block truncate text-[10px] text-muted">
                    {{ step.description }}
                  </span>
                </div>
                <Select
                  :model-value="step.status"
                  :options="STATUS_OPTIONS"
                  option-label="label"
                  option-value="value"
                  class="!h-auto shrink-0"
                  :pt="selectPt"
                  @update:model-value="ctrl.updateStepStatus(entry.proc.id, step.id, $event)"
                >
                  <template #value="{ value }">
                    <span
                      :class="[
                        'rounded-full px-1.5 py-0.5 text-[9px] font-bold',
                        stepStatusBadgeClass(value),
                      ]"
                    >
                      {{ value }}
                    </span>
                  </template>
                </Select>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <WorkspacePlanDialog
      v-model:visible="showDialog"
      :linked-tasks="ctrl.linkedTasks.value"
      :workflows="ctrl.workflows.value"
      :editing-group="editingGroup"
      @create="onDialogCreate"
      @edit="onDialogEdit"
    />
  </div>
</template>
