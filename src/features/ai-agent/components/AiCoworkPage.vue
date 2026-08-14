<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import Select from "primevue/select";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import Checkbox from "primevue/checkbox";
import TaskPickerDialog from "@/features/task/components/TaskPickerDialog.vue";
import { useCowork } from "../composables/useCowork";
import { useAiUsage } from "../composables/useAiUsage";
import { STEP_TYPE_META } from "@/models/workflow";
import { TASK_CATEGORY_META } from "@/models/task";
import type { Task } from "@/models/task";

const { t } = useI18n();
const ctrl = useCowork();
const aiUsage = useAiUsage();

const activeAccount = computed(() => aiUsage.accounts.value.find((a) => a.is_active) ?? null);

const workflowOptions = computed(() => ctrl.workflows.value.map((w) => ({ label: w.name, value: w.id })));

async function browseWorkDir() {
  const selected = await open({ directory: true, title: t("aiCowork.chooseWorkDir") });
  if (typeof selected === "string") ctrl.workDir.value = selected;
}

function stepBadgeClass(type: string): string {
  return STEP_TYPE_META[type as keyof typeof STEP_TYPE_META]?.badgeClass ?? "bg-canvas text-muted";
}

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

async function openTerminalForStep(stepId: number) {
  const step = ctrl.steps.value.find((s) => s.id === stepId);
  if (!step || !activeAccount.value) return;
  await ctrl.openStepTerminal(step, activeAccount.value.config_dir);
}

// --- Task picker dialog ---
const showPickerDialog = ref(false);
const excludeIds = computed(() => ctrl.selectedTasks.value.map((t) => t.id));

function openPicker() {
  showPickerDialog.value = true;
}

function pickTask(task: Task) {
  ctrl.addTask(task);
}

onMounted(() => {
  aiUsage.start();
});
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">
    <p v-if="ctrl.error.value" class="banner-danger">{{ ctrl.error.value }}</p>

    <!-- Header: working dir + active AI account -->
    <section class="flex flex-wrap items-end gap-4 rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <label class="block min-w-0 flex-1">
        <span class="text-xs font-bold text-muted">{{ t("aiCowork.workDir") }}</span>
        <InputGroup class="mt-1 h-9">
          <InputText readonly :placeholder="t('aiCowork.workDirPlaceholder')" :model-value="ctrl.workDir.value" />
          <Button icon="pi pi-folder-open" severity="secondary" outlined @click="browseWorkDir" />
        </InputGroup>
      </label>
      <div class="min-w-0">
        <span class="text-xs font-bold text-muted">{{ t("aiCowork.activeAccount") }}</span>
        <p class="mt-1 flex h-9 items-center gap-2 rounded-md border border-divider bg-canvas px-3 text-sm">
          <i class="pi pi-microchip-ai text-brand" />
          <span v-if="activeAccount" class="font-semibold text-ink">{{ activeAccount.name }}</span>
          <span v-else class="text-muted">{{ t("aiCowork.noActiveAccount") }}</span>
        </p>
      </div>
    </section>

    <div class="grid min-h-0 flex-1 grid-cols-1 gap-4 overflow-hidden lg:grid-cols-2">
      <!-- Tasks column -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <div class="flex items-center justify-between gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title">{{ t("aiCowork.tasks.title") }}</h3>
          <Button icon="pi pi-plus" size="small" :label="t('aiCowork.tasks.add')" @click="openPicker" />
        </div>
        <div class="flex min-h-0 flex-1 flex-col gap-1.5 overflow-auto p-3">
          <p v-if="ctrl.selectedTasks.value.length === 0" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("aiCowork.tasks.empty") }}
          </p>
          <div
            v-for="task in ctrl.selectedTasks.value"
            :key="task.id"
            class="flex items-center gap-2 rounded-md border border-divider bg-canvas px-3 py-2"
          >
            <Checkbox
              binary
              :model-value="ctrl.confirmedTaskIds.value.includes(task.id)"
              @update:model-value="ctrl.toggleConfirm(task.id)"
            />
            <span class="font-mono text-xs text-ink">{{ task.task_cd }}</span>
            <span :class="['rounded-full px-2 py-0.5 text-[10px] font-bold', categoryBadgeClass(task.category_id)]">
              {{ task.category_id }}
            </span>
            <span class="min-w-0 flex-1 truncate text-xs text-secondary">{{ task.task_name }}</span>
            <Button icon="pi pi-times" text rounded size="small" severity="danger" @click="ctrl.removeTask(task.id)" />
          </div>
        </div>
      </section>

      <!-- Workflow column -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <div class="flex items-center gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title shrink-0">{{ t("aiCowork.workflow.title") }}</h3>
          <Select
            v-model="ctrl.selectedWorkflowId.value"
            class="min-w-0 flex-1"
            :options="workflowOptions"
            option-label="label"
            option-value="value"
            :placeholder="t('aiCowork.workflow.placeholder')"
          />
          <Button
            size="small"
            :label="t('aiCowork.workflow.apply')"
            :loading="ctrl.isApplying.value"
            :disabled="ctrl.selectedWorkflowId.value === null"
            @click="ctrl.applyWorkflow()"
          />
        </div>
        <div class="flex min-h-0 flex-1 flex-col gap-1.5 overflow-auto p-3">
          <p v-if="ctrl.appliedWorkflowId.value === null" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("aiCowork.workflow.notApplied") }}
          </p>
          <div
            v-for="(step, index) in ctrl.steps.value"
            :key="step.id"
            class="flex items-center gap-2 rounded-md border px-3 py-2"
            :class="step.id === ctrl.currentStepId.value ? 'border-brand bg-brand/5' : 'border-divider bg-canvas'"
          >
            <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-brand/10 text-xs font-bold text-brand">
              {{ index + 1 }}
            </span>
            <i :class="[step.icon, 'text-muted']" />
            <span class="min-w-0 flex-1 truncate text-sm text-ink">{{ step.name }}</span>
            <span :class="['shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold', stepBadgeClass(step.step_type)]">
              {{ step.step_type }}
            </span>
            <i v-if="step.id === ctrl.currentStepId.value" class="pi pi-map-marker shrink-0 text-brand" :title="t('aiCowork.workflow.currentStep')" />
            <Button
              v-if="step.step_type === 'skill'"
              icon="pi pi-desktop"
              text
              rounded
              size="small"
              :title="t('aiCowork.workflow.openTerminal')"
              @click="openTerminalForStep(step.id)"
            />
          </div>
        </div>
      </section>
    </div>

    <!-- Task picker dialog -->
    <TaskPickerDialog v-model:visible="showPickerDialog" :exclude-ids="excludeIds" @pick="pickTask" />
  </section>
</template>
