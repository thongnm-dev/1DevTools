<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Listbox from "primevue/listbox";
import Select from "primevue/select";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useWorkflow } from "../composables/useWorkflow";
import { usePrompt } from "@/features/prompt/composables/usePrompt";
import type { NodePos, Workflow, WorkflowStepType } from "@/models/workflow";
import { DEFAULT_WORKFLOW_ICON, STEP_TYPE_META } from "@/models/workflow";
import { agentProviderModelLabel } from "@/models/agent-provider-model";

const { t } = useI18n();
const ctrl = useWorkflow();
const promptCtrl = usePrompt();

// --- Sidebar resize (pattern from AiWorkflowPage.vue) ---
const SIDEBAR_MIN = 200;
const SIDEBAR_MAX = 400;
const sidebarWidth = ref(280);
const sidebarCollapsed = ref(false);
let cleanupDrag: (() => void) | null = null;

function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value;
}

function startDrag(event: MouseEvent) {
  event.preventDefault();
  const startX = event.clientX;
  const startWidth = sidebarWidth.value;
  function onMove(ev: MouseEvent) {
    sidebarWidth.value = Math.min(SIDEBAR_MAX, Math.max(SIDEBAR_MIN, startWidth + (ev.clientX - startX)));
  }
  function onUp() {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onUp);
    document.body.style.userSelect = "";
    cleanupDrag = null;
  }
  document.body.style.userSelect = "none";
  document.addEventListener("mousemove", onMove);
  document.addEventListener("mouseup", onUp);
  cleanupDrag = onUp;
}

onBeforeUnmount(() => cleanupDrag?.());

// --- Workflow dialog ---
const showWorkflowDialog = ref(false);
const editingWorkflowId = ref<number | null>(null);
const wfName = ref("");
const wfDescription = ref("");
const wfIcon = ref(DEFAULT_WORKFLOW_ICON);
const showWorkflowIconPicker = ref(false);

function openCreateWorkflowDialog() {
  editingWorkflowId.value = null;
  wfName.value = "";
  wfDescription.value = "";
  wfIcon.value = DEFAULT_WORKFLOW_ICON;
  showWorkflowDialog.value = true;
}

function openEditWorkflowDialog() {
  const wf = ctrl.activeWorkflow.value;
  if (!wf) return;
  editingWorkflowId.value = wf.id;
  wfName.value = wf.name;
  wfDescription.value = wf.description;
  wfIcon.value = wf.icon;
  showWorkflowDialog.value = true;
}

async function saveWorkflow() {
  const name = wfName.value.trim();
  if (!name) return;
  if (editingWorkflowId.value) {
    await ctrl.updateWorkflow(editingWorkflowId.value, { name, description: wfDescription.value.trim(), icon: wfIcon.value });
  } else {
    await ctrl.createWorkflow(name, wfDescription.value.trim(), wfIcon.value);
  }
  showWorkflowDialog.value = false;
}

// --- Step dialog ---
const showStepDialog = ref(false);
const editingStepId = ref<number | null>(null);
const afterStepId = ref<number | null>(null);
const stepName = ref("");
const stepType = ref<WorkflowStepType>("custom");
const stepDescription = ref("");
const stepIcon = ref(STEP_TYPE_META.custom.icon);
const stepIsLatest = ref(false);
const stepAiAccountId = ref<number | null>(null);
const stepSkillName = ref("");
const stepModelId = ref<number | null>(null);
const stepPromptId = ref<number | null>(null);
const stepRunnerCommand = ref("");

const promptOptions = computed(() => promptCtrl.prompts.value.map((p) => ({ label: p.title, value: p.id })));

const agentOptions = computed(() =>
  ctrl.aiAccounts.value.map((a) => ({ label: `${a.name} (${a.provider})`, value: a.id })),
);

const modelOptions = computed(() =>
  ctrl.models.value.map((m) => ({ label: agentProviderModelLabel(m), value: m.id })),
);

const stepTypeOptions = computed(() =>
  (Object.keys(STEP_TYPE_META) as WorkflowStepType[]).map((value) => ({
    label: t(`workflow.stepType.${value}`),
    value,
    icon: STEP_TYPE_META[value].icon,
  })),
);

function openAddStepDialog(after: number | null) {
  editingStepId.value = null;
  afterStepId.value = after;
  stepName.value = "";
  stepType.value = "custom";
  stepDescription.value = "";
  stepIcon.value = STEP_TYPE_META.custom.icon;
  stepIsLatest.value = false;
  stepAiAccountId.value = null;
  stepSkillName.value = "";
  stepModelId.value = null;
  stepPromptId.value = null;
  stepRunnerCommand.value = "";
  showStepDialog.value = true;
}

function openEditStepDialog(stepId: number) {
  const step = ctrl.activeSteps.value.find((s) => s.id === stepId);
  if (!step) return;
  editingStepId.value = step.id;
  afterStepId.value = null;
  stepName.value = step.name;
  stepType.value = step.step_type;
  stepDescription.value = step.description;
  stepIcon.value = step.icon;
  stepIsLatest.value = step.is_latest_step;
  stepAiAccountId.value = step.ai_account_id;
  stepSkillName.value = step.skill_name;
  stepModelId.value = step.model_id;
  stepPromptId.value = step.prompt_id;
  stepRunnerCommand.value = step.runner_command ?? "";
  showStepDialog.value = true;
}

function onStepTypeChange() {
  stepIcon.value = STEP_TYPE_META[stepType.value].icon;
}

async function saveStep() {
  const name = stepName.value.trim();
  if (!name) return;
  const isRunnerLike = stepType.value === "runner" || stepType.value === "terminal";
  const data = {
    name,
    step_type: stepType.value,
    icon: stepIcon.value,
    description: stepDescription.value.trim(),
    is_latest_step: stepIsLatest.value,
    ai_account_id: stepAiAccountId.value,
    skill_name: stepType.value === "skill" ? stepSkillName.value.trim() : "",
    model_id: stepType.value === "skill" ? stepModelId.value : null,
    prompt_id: stepType.value === "prompt" ? stepPromptId.value : null,
    runner_command: isRunnerLike ? stepRunnerCommand.value.trim() : "",
  };
  if (editingStepId.value) {
    await ctrl.updateStep(editingStepId.value, data);
  } else {
    await ctrl.addStep(afterStepId.value, data);
  }
  showStepDialog.value = false;
}

// --- Delete confirmation ---
const showDeleteDialog = ref(false);
const deleteTarget = ref<{ type: "workflow" | "step"; id: number; name: string } | null>(null);

function confirmDeleteWorkflow(wf?: Workflow) {
  const target = wf ?? ctrl.activeWorkflow.value;
  if (!target) return;
  deleteTarget.value = { type: "workflow", id: target.id, name: target.name };
  showDeleteDialog.value = true;
}

function confirmDeleteStep(stepId: number) {
  const step = ctrl.activeSteps.value.find((s) => s.id === stepId);
  if (!step) return;
  deleteTarget.value = { type: "step", id: step.id, name: step.name };
  showDeleteDialog.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  if (deleteTarget.value.type === "workflow") {
    await ctrl.deleteWorkflow(deleteTarget.value.id);
  } else {
    await ctrl.deleteStep(deleteTarget.value.id);
  }
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}

// --- SVG Arrow computation ---
const nodesContainer = ref<HTMLElement | null>(null);
const arrowPaths = ref<{ id: string; d: string }[]>([]);
let resizeObserver: ResizeObserver | null = null;

function updateArrows() {
  const container = nodesContainer.value;
  const steps = ctrl.activeSteps.value;
  if (!container || steps.length < 2) {
    arrowPaths.value = [];
    return;
  }

  const containerRect = container.getBoundingClientRect();
  const paths: { id: string; d: string }[] = [];

  for (let i = 0; i < steps.length - 1; i++) {
    const fromEl = container.querySelector(`[data-step-id="${steps[i].id}"]`) as HTMLElement | null;
    const toEl = container.querySelector(`[data-step-id="${steps[i + 1].id}"]`) as HTMLElement | null;
    if (!fromEl || !toEl) continue;

    const fromRect = fromEl.getBoundingClientRect();
    const toRect = toEl.getBoundingClientRect();

    const x1 = fromRect.right - containerRect.left;
    const y1 = fromRect.top + fromRect.height / 2 - containerRect.top;
    const x2 = toRect.left - containerRect.left;
    const y2 = toRect.top + toRect.height / 2 - containerRect.top;

    const sameRow = Math.abs(y1 - y2) < 30;
    let d: string;
    if (sameRow) {
      const cx = (x1 + x2) / 2;
      d = `M ${x1} ${y1} C ${cx} ${y1}, ${cx} ${y2}, ${x2} ${y2}`;
    } else {
      d = `M ${x1} ${y1} C ${x1 + 50} ${y1}, ${x2 - 50} ${y2}, ${x2} ${y2}`;
    }
    paths.push({ id: `arrow-${steps[i].id}-${steps[i + 1].id}`, d });
  }

  arrowPaths.value = paths;
}

watch(
  () => ctrl.activeId.value,
  async (id) => {
    if (id !== null) {
      await nextTick();
      loadPositions(id);
    }
    await nextTick();
    updateArrows();
  },
);

watch(
  () => ctrl.activeSteps.value.length,
  async () => {
    ctrl.activeSteps.value.forEach((s) => ensureStepPosition(s.id));
    await nextTick();
    updateArrows();
  },
);

onMounted(() => {
  if (ctrl.activeId.value !== null) {
    loadPositions(ctrl.activeId.value);
  }
  void nextTick(() => updateArrows());
  if (nodesContainer.value) {
    resizeObserver = new ResizeObserver(() => updateArrows());
    resizeObserver.observe(nodesContainer.value);
  }
});

onBeforeUnmount(() => resizeObserver?.disconnect());

// --- Helpers ---
function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString([], { month: "short", day: "numeric" });
}

function stepTypeBadgeClass(type: WorkflowStepType): string {
  return STEP_TYPE_META[type]?.badgeClass ?? "bg-canvas text-muted";
}

function stepTypeLabel(type: WorkflowStepType): string {
  return t(`workflow.stepType.${type}`);
}

function agentLabel(id: number | null): string {
  if (id === null) return "";
  const account = ctrl.aiAccounts.value.find((a) => a.id === id);
  return account ? account.name : "";
}

const showStepIconPicker = ref(false);

// --- Free-form canvas node positioning ---
const NODE_W = 208;
const NODE_H = 140;
const H_GAP = 120;
const V_GAP = 100;
const NODES_PER_ROW = 3;

const nodePositions = ref<Record<string, NodePos>>({});
const draggingNodeId = ref<number | null>(null);
let nodeDragStartX = 0;
let nodeDragStartY = 0;
let nodeDragOrigX = 0;
let nodeDragOrigY = 0;

function loadPositions(workflowId: number) {
  const wf = ctrl.workflows.value.find((w) => w.id === workflowId);
  if (wf && Object.keys(wf.layout).length > 0) {
    nodePositions.value = wf.layout;
  } else {
    autoLayout();
  }
}

function savePositions() {
  const wf = ctrl.activeWorkflow.value;
  if (!wf) return;
  ctrl.saveLayout(wf.id, nodePositions.value);
}

function autoLayout() {
  const pos: Record<string, NodePos> = {};
  ctrl.activeSteps.value.forEach((step, i) => {
    const col = i % NODES_PER_ROW;
    const row = Math.floor(i / NODES_PER_ROW);
    pos[step.id] = {
      x: col * (NODE_W + H_GAP),
      y: row * (NODE_H + V_GAP),
    };
  });
  nodePositions.value = pos;
  savePositions();
  void nextTick(() => updateArrows());
}

function ensureStepPosition(stepId: number) {
  if (nodePositions.value[stepId]) return;
  const steps = ctrl.activeSteps.value;
  const idx = steps.findIndex((s) => s.id === stepId);
  if (idx <= 0) {
    nodePositions.value[stepId] = { x: 0, y: 0 };
  } else {
    const prev = nodePositions.value[steps[idx - 1].id];
    if (prev) {
      nodePositions.value[stepId] = { x: prev.x + NODE_W + H_GAP, y: prev.y };
    } else {
      const col = idx % NODES_PER_ROW;
      const row = Math.floor(idx / NODES_PER_ROW);
      nodePositions.value[stepId] = { x: col * (NODE_W + H_GAP), y: row * (NODE_H + V_GAP) };
    }
  }
  savePositions();
}

const canvasSize = computed(() => {
  let maxX = 0;
  let maxY = 0;
  for (const pos of Object.values(nodePositions.value)) {
    maxX = Math.max(maxX, pos.x + NODE_W + 60);
    maxY = Math.max(maxY, pos.y + NODE_H + 60);
  }
  return { width: Math.max(maxX, 600), height: Math.max(maxY, 300) };
});

function startNodeDrag(stepId: number, event: MouseEvent) {
  if ((event.target as HTMLElement).closest("button")) return;
  event.preventDefault();
  draggingNodeId.value = stepId;
  nodeDragStartX = event.clientX;
  nodeDragStartY = event.clientY;
  const pos = nodePositions.value[stepId] ?? { x: 0, y: 0 };
  nodeDragOrigX = pos.x;
  nodeDragOrigY = pos.y;
  document.addEventListener("mousemove", onNodeDrag);
  document.addEventListener("mouseup", stopNodeDrag);
  document.body.style.userSelect = "none";
}

function onNodeDrag(event: MouseEvent) {
  if (draggingNodeId.value === null) return;
  const dx = event.clientX - nodeDragStartX;
  const dy = event.clientY - nodeDragStartY;
  nodePositions.value = {
    ...nodePositions.value,
    [draggingNodeId.value]: {
      x: Math.max(0, nodeDragOrigX + dx),
      y: Math.max(0, nodeDragOrigY + dy),
    },
  };
  updateArrows();
}

function stopNodeDrag() {
  draggingNodeId.value = null;
  document.removeEventListener("mousemove", onNodeDrag);
  document.removeEventListener("mouseup", stopNodeDrag);
  document.body.style.userSelect = "";
  savePositions();
  void nextTick(() => updateArrows());
}

const selectPt = {
  root: { class: "!bg-panel !border-divider" },
  label: { class: "!flex !items-center !text-xs !py-1.5 !text-ink" },
  option: { class: "!text-xs" },
};

const listboxPt = {
  root: { class: "!flex !min-h-0 !flex-1 !flex-col !border-0 !bg-transparent !shadow-none" },
  header: { class: "!border-0 !bg-transparent !p-0 !pb-2" },
  pcFilter: { class: "embedded-input w-full !bg-canvas !text-xs" },
  listContainer: { class: "!flex-1 !p-0" },
  list: { class: "!gap-1 !p-0" },
  option: { class: "group !items-center !rounded-md !px-2 !py-2" },
  emptyMessage: { class: "!px-2 !py-4 !text-center !text-xs !text-muted" },
};
</script>

<template>
  <div class="flex flex-1 gap-3 overflow-hidden">
    <!-- Left sidebar: workflow list (shrinks to an icon rail instead of hiding, so the
         expand toggle never floats over the detail panel) -->
    <aside
      class="flex shrink-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm transition-[width] duration-150"
      :style="{ width: (sidebarCollapsed ? 52 : sidebarWidth) + 'px' }"
    >
      <div class="flex items-center gap-2 border-b border-divider p-3" :class="{ 'justify-center': sidebarCollapsed }">
        <Button
          :icon="sidebarCollapsed ? 'pi pi-angle-double-right' : 'pi pi-angle-double-left'"
          text
          rounded
          size="small"
          :title="sidebarCollapsed ? t('workflow.title') : t('common.close')"
          @click="toggleSidebar"
        />
        <template v-if="!sidebarCollapsed">
          <span class="text-sm font-semibold text-ink">{{ t("workflow.title") }}</span>
          <Button icon="pi pi-plus" size="small" class="ml-auto" :title="t('workflow.newWorkflow')" @click="openCreateWorkflowDialog" />
        </template>
      </div>

      <div class="flex min-h-0 flex-1 flex-col overflow-auto p-2" :class="{ 'items-center': sidebarCollapsed }">
        <template v-if="sidebarCollapsed">
          <Button
            v-for="wf in ctrl.workflows.value"
            :key="wf.id"
            :icon="wf.icon"
            text
            rounded
            size="small"
            class="mb-1"
            :class="wf.id === ctrl.activeId.value ? '!bg-brand/10 !text-brand' : ''"
            :title="wf.name"
            @click="ctrl.selectWorkflow(wf.id)"
          />
          <Button
            icon="pi pi-plus"
            text
            rounded
            size="small"
            severity="secondary"
            :title="t('workflow.newWorkflow')"
            @click="openCreateWorkflowDialog"
          />
        </template>
        <div v-else-if="ctrl.isLoading.value" class="px-2 py-4 text-center text-xs text-muted">{{ t("common.loading") }}</div>
        <Listbox
          v-else
          :model-value="ctrl.activeId.value"
          :options="ctrl.workflows.value"
          option-label="name"
          option-value="id"
          filter
          :filter-placeholder="t('workflow.searchPlaceholder')"
          :filter-fields="['name', 'description']"
          :pt="listboxPt"
          @update:model-value="(id: number | null) => id !== null && ctrl.selectWorkflow(id)"
        >
          <template #option="{ option }">
            <i :class="[option.icon, 'shrink-0 text-xs']" />
            <span class="min-w-0 flex-1">
              <span class="block truncate text-xs font-bold">{{ option.name }}</span>
              <span class="block truncate text-[10px] text-muted">
                {{ formatDate(option.updated_at) }} · {{ t("workflow.stepCount", { count: option.step_count }) }}
              </span>
            </span>
            <Button
              icon="pi pi-trash"
              text
              rounded
              size="small"
              severity="danger"
              class="!h-6 !w-6 shrink-0 opacity-0 group-hover:opacity-100"
              :title="t('common.delete')"
              @click.stop="confirmDeleteWorkflow(option)"
            />
          </template>
          <template #emptyfilter>{{ t("workflow.empty") }}</template>
          <template #empty>{{ t("workflow.empty") }}</template>
        </Listbox>
      </div>
    </aside>

    <!-- Drag handle -->
    <div
      v-if="!sidebarCollapsed"
      class="group flex w-1.5 shrink-0 cursor-col-resize items-center justify-center"
      title="Drag to resize"
      @mousedown="startDrag"
    >
      <div class="h-10 w-1 rounded-full bg-divider transition-colors group-hover:bg-brand" />
    </div>

    <!-- Main area -->
    <div class="flex min-h-0 min-w-0 flex-1 flex-col gap-4 overflow-hidden">
      <!-- Empty state -->
      <div v-if="!ctrl.activeWorkflow.value" class="flex flex-1 items-center justify-center rounded-lg border border-dashed border-divider bg-panel/50 p-12">
        <div class="text-center">
          <i class="pi pi-sitemap text-4xl text-muted/60" />
          <p class="mt-2 text-sm text-muted">{{ t("workflow.emptyState") }}</p>
          <Button icon="pi pi-plus" :label="t('workflow.createWorkflow')" class="mt-4" @click="openCreateWorkflowDialog" />
        </div>
      </div>

      <template v-else>
        <!-- Header panel -->
        <div class="shrink-0 rounded-lg border border-divider bg-panel p-6 shadow-sm">
          <div class="flex flex-wrap items-center gap-3">
            <i :class="[ctrl.activeWorkflow.value.icon, 'text-2xl text-muted']" />
            <div class="min-w-0">
              <h2 class="page-title">{{ ctrl.activeWorkflow.value.name }}</h2>
              <p class="text-sm text-muted">{{ ctrl.activeWorkflow.value.description || t("workflow.noDescription") }}</p>
            </div>
            <div class="ml-auto flex shrink-0 items-center gap-2">
              <Button icon="pi pi-objects-column" :label="t('workflow.autoLayout')" severity="secondary" size="small" @click="autoLayout" />
              <Button icon="pi pi-pencil" :label="t('common.edit')" severity="secondary" size="small" @click="openEditWorkflowDialog" />
              <Button icon="pi pi-copy" :label="t('workflow.duplicate')" severity="secondary" size="small" @click="ctrl.duplicateWorkflow(ctrl.activeId.value!)" />
              <Button icon="pi pi-trash" :label="t('common.delete')" severity="danger" text size="small" @click="confirmDeleteWorkflow()" />
            </div>
          </div>
        </div>

        <!-- Diagram area -->
        <div class="relative flex-1 overflow-auto rounded-lg border border-divider bg-panel shadow-sm">
          <div
            ref="nodesContainer"
            class="relative"
            :style="{ minWidth: canvasSize.width + 'px', minHeight: canvasSize.height + 'px' }"
          >
            <!-- SVG arrow layer -->
            <svg class="pointer-events-none absolute inset-0 overflow-visible" :style="{ width: canvasSize.width + 'px', height: canvasSize.height + 'px' }" style="z-index: 0">
              <defs>
                <marker id="wf-arrowhead" markerWidth="10" markerHeight="7" refX="10" refY="3.5" orient="auto">
                  <polygon points="0 0, 10 3.5, 0 7" style="fill: rgb(var(--color-text-muted))" />
                </marker>
              </defs>
              <path
                v-for="arrow in arrowPaths"
                :key="arrow.id"
                :d="arrow.d"
                fill="none"
                stroke-width="2"
                stroke-linecap="round"
                marker-end="url(#wf-arrowhead)"
                style="stroke: rgb(var(--color-text-muted))"
              />
            </svg>

            <!-- Nodes (absolute positioned) -->
            <template v-for="(step, index) in ctrl.activeSteps.value" :key="step.id">
              <div
                :data-step-id="step.id"
                class="absolute z-10 w-52 rounded-lg border bg-panel p-4 shadow-card transition-shadow hover:shadow-float"
                :class="[
                  step.id === ctrl.selectedStepId.value ? 'border-brand ring-2 ring-brand/30' : 'border-divider',
                  draggingNodeId === step.id ? 'shadow-float cursor-grabbing opacity-80' : 'cursor-grab',
                ]"
                :style="{
                  left: (nodePositions[step.id]?.x ?? 0) + 'px',
                  top: (nodePositions[step.id]?.y ?? 0) + 'px',
                }"
                @mousedown="startNodeDrag(step.id, $event)"
                @click="ctrl.selectStep(step.id)"
                @dblclick="openEditStepDialog(step.id)"
              >
                <div class="mb-2 flex items-center gap-2">
                  <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-brand/10 text-xs font-bold text-brand">
                    {{ index + 1 }}
                  </span>
                  <i :class="[step.icon, 'text-muted']" />
                  <h4 class="section-title min-w-0 flex-1 break-words">{{ step.name }}</h4>
                  <button
                    class="shrink-0 text-muted opacity-0 transition-opacity hover:text-red-500"
                    :class="{ 'opacity-100': step.id === ctrl.selectedStepId.value }"
                    :title="t('workflow.deleteStep')"
                    @click.stop="confirmDeleteStep(step.id)"
                  >
                    <i class="pi pi-times text-[10px]" />
                  </button>
                </div>
                <div class="mt-1 flex flex-wrap items-center gap-1">
                  <span :class="['inline-block rounded-full px-2 py-0.5 text-[11px] font-bold', stepTypeBadgeClass(step.step_type)]">
                    {{ stepTypeLabel(step.step_type) }}
                  </span>
                  <span v-if="step.is_latest_step" class="badge-success" :title="t('workflow.step.isLatestHint')">
                    <i class="pi pi-flag-fill text-[9px]" />{{ t("workflow.latest") }}
                  </span>
                  <span
                    v-if="step.ai_account_id !== null"
                    class="inline-flex items-center gap-1 rounded-full bg-canvas px-2 py-0.5 text-[11px] font-bold text-secondary"
                    :title="t('workflow.step.agent')"
                  >
                    <i class="pi pi-microchip-ai text-[9px]" />{{ agentLabel(step.ai_account_id) }}
                  </span>
                </div>
                <p class="mt-2 line-clamp-2 text-xs text-muted">{{ step.description }}</p>
              </div>
            </template>

            <!-- Add step button (fixed bottom-right of canvas) -->
            <button
              class="absolute z-10 flex h-10 items-center gap-2 rounded-lg border border-dashed border-divider px-4 text-sm text-muted transition-colors hover:border-brand hover:bg-brand/5 hover:text-brand"
              :style="{ left: '16px', top: (canvasSize.height - 20) + 'px' }"
              @click="openAddStepDialog(ctrl.activeSteps.value.length > 0 ? ctrl.activeSteps.value[ctrl.activeSteps.value.length - 1].id : null)"
            >
              <i class="pi pi-plus-circle" />
              <span>{{ t("workflow.addStep") }}</span>
            </button>
          </div>
        </div>

        <!-- Selected step detail panel -->
        <div v-if="ctrl.selectedStep.value" class="shrink-0 rounded-lg border border-divider bg-panel p-4 shadow-sm">
          <div class="flex items-center gap-3">
            <i :class="[ctrl.selectedStep.value.icon, 'text-xl text-brand']" />
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-semibold text-ink">{{ ctrl.selectedStep.value.name }}</p>
              <p class="truncate text-xs text-muted">{{ ctrl.selectedStep.value.description || t("workflow.noDescription") }}</p>
            </div>
            <span :class="['rounded-full px-2 py-0.5 text-[11px] font-bold', stepTypeBadgeClass(ctrl.selectedStep.value.step_type)]">
              {{ stepTypeLabel(ctrl.selectedStep.value.step_type) }}
            </span>
            <Button icon="pi pi-pencil" severity="secondary" text rounded size="small" :title="t('workflow.editStep')" @click="openEditStepDialog(ctrl.selectedStep.value.id)" />
            <Button icon="pi pi-trash" severity="danger" text rounded size="small" :title="t('workflow.deleteStep')" @click="confirmDeleteStep(ctrl.selectedStep.value.id)" />
            <Button icon="pi pi-times" severity="secondary" text rounded size="small" :title="t('common.close')" @click="ctrl.selectStep(null)" />
          </div>
        </div>
      </template>
    </div>

    <!-- Add/Edit Workflow Dialog -->
    <Dialog
      :visible="showWorkflowDialog"
      class="w-full max-w-md rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showWorkflowDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ editingWorkflowId ? t("workflow.dialog.editTitle") : t("workflow.dialog.newTitle") }}</h3>
      </template>

      <div class="space-y-4">
        <div class="flex items-end gap-3">
          <label class="block min-w-0 flex-1">
            <span class="text-xs font-bold text-muted">{{ t("workflow.dialog.name") }} <span class="text-red-500">*</span></span>
            <InputText v-model="wfName" class="mt-1 w-full" :placeholder="t('workflow.dialog.namePlaceholder')" autofocus />
          </label>
          <div class="block">
            <span class="text-xs font-bold text-muted">{{ t("workflow.step.icon") }}</span>
            <div class="mt-1 flex items-center gap-2">
              <div class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3">
                <i :class="[wfIcon, 'text-brand']" />
                <InputText
                  v-model="wfIcon"
                  class="embedded-input w-24 border-0 !bg-transparent !p-0 !text-sm"
                  placeholder="pi pi-sitemap"
                />
              </div>
              <Button icon="pi pi-th-large" severity="secondary" outlined :title="t('workflow.step.browseIcons')" @click="showWorkflowIconPicker = true" />
            </div>
          </div>
        </div>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.dialog.description") }}</span>
          <InputText v-model="wfDescription" class="mt-1 w-full" :placeholder="t('workflow.dialog.descriptionPlaceholder')" />
        </label>
      </div>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="editingWorkflowId ? t('common.save') : t('common.create')"
          :confirm-icon="editingWorkflowId ? 'pi pi-check' : 'pi pi-plus'"
          :confirm-disabled="!wfName.trim()"
          @cancel="showWorkflowDialog = false"
          @confirm="saveWorkflow"
        />
      </template>
    </Dialog>

    <!-- Add/Edit Step Dialog -->
    <Dialog
      :visible="showStepDialog"
      class="w-full max-w-md rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showStepDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ editingStepId ? t("workflow.step.editTitle") : t("workflow.step.newTitle") }}</h3>
      </template>

      <div class="space-y-4">
        <div class="flex items-end gap-3">
          <label class="block min-w-0 flex-1">
            <span class="text-xs font-bold text-muted">{{ t("workflow.step.name") }} <span class="text-red-500">*</span></span>
            <InputText v-model="stepName" class="mt-1 w-full" :placeholder="t('workflow.step.namePlaceholder')" autofocus />
          </label>
          <div class="block">
            <span class="text-xs font-bold text-muted">{{ t("workflow.step.icon") }}</span>
            <div class="mt-1 flex items-center gap-2">
              <div class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3">
                <i :class="[stepIcon, 'text-brand']" />
                <InputText
                  v-model="stepIcon"
                  class="embedded-input w-24 border-0 !bg-transparent !p-0 !text-sm"
                  placeholder="pi pi-cog"
                />
              </div>
              <Button icon="pi pi-th-large" severity="secondary" outlined :title="t('workflow.step.browseIcons')" @click="showStepIconPicker = true" />
            </div>
          </div>
        </div>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.type") }}</span>
          <Select
            v-model="stepType"
            :options="stepTypeOptions"
            optionLabel="label"
            optionValue="value"
            class="mt-1 w-full"
            :pt="selectPt"
            @change="onStepTypeChange"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.agent") }}</span>
          <Select
            v-model="stepAiAccountId"
            :options="agentOptions"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('workflow.step.agentPlaceholder')"
            show-clear
            class="mt-1 w-full"
            :pt="selectPt"
          />
          <span class="text-xs text-muted">{{ t("workflow.step.agentHint") }}</span>
        </label>
        <label v-if="stepType === 'skill'" class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.skill") }}</span>
          <InputText v-model="stepSkillName" class="mt-1 w-full" :placeholder="t('workflow.step.skillPlaceholder')" />
          <span class="text-xs text-muted">{{ t("workflow.step.skillHint") }}</span>
        </label>
        <label v-if="stepType === 'skill'" class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.model") }}</span>
          <Select
            v-model="stepModelId"
            :options="modelOptions"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('workflow.step.modelPlaceholder')"
            show-clear
            class="mt-1 w-full"
            :pt="selectPt"
          />
        </label>
        <label v-if="stepType === 'prompt'" class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.prompt") }}</span>
          <Select
            v-model="stepPromptId"
            :options="promptOptions"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('workflow.step.promptPlaceholder')"
            show-clear
            class="mt-1 w-full"
            :pt="selectPt"
          />
          <span v-if="promptOptions.length === 0" class="text-xs text-muted">{{ t("workflow.step.promptHint") }}</span>
        </label>
        <label v-if="stepType === 'runner' || stepType === 'terminal'" class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.command") }}</span>
          <InputText v-model="stepRunnerCommand" class="mt-1 w-full" :placeholder="t('workflow.step.commandPlaceholder')" />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("workflow.step.description") }}</span>
          <InputText v-model="stepDescription" class="mt-1 w-full" :placeholder="t('workflow.step.descriptionPlaceholder')" />
        </label>
        <label class="flex items-start gap-2">
          <Checkbox v-model="stepIsLatest" binary input-id="step-is-latest" class="mt-0.5" />
          <span class="min-w-0">
            <span class="text-sm text-ink">{{ t("workflow.step.isLatest") }}</span>
            <span class="block text-xs text-muted">{{ t("workflow.step.isLatestHint") }}</span>
          </span>
        </label>
      </div>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="editingStepId ? t('common.save') : t('workflow.step.add')"
          :confirm-icon="editingStepId ? 'pi pi-check' : 'pi pi-plus'"
          :confirm-disabled="!stepName.trim()"
          @cancel="showStepDialog = false"
          @confirm="saveStep"
        />
      </template>
    </Dialog>

    <!-- Step Icon Picker Dialog -->
    <IconPickerDialog
      :visible="showStepIconPicker"
      :selected="stepIcon"
      @update:visible="showStepIconPicker = $event"
      @select="(icon: string) => (stepIcon = 'pi ' + icon)"
    />

    <!-- Workflow Icon Picker Dialog -->
    <IconPickerDialog
      :visible="showWorkflowIconPicker"
      :selected="wfIcon"
      @update:visible="showWorkflowIconPicker = $event"
      @select="(icon: string) => (wfIcon = 'pi ' + icon)"
    />

    <!-- Delete Confirmation Dialog -->
    <Dialog
      :visible="showDeleteDialog"
      class="w-full max-w-sm rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showDeleteDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ t("workflow.deleteConfirm.title") }}</h3>
      </template>

      <p class="text-sm text-ink">{{ t("workflow.deleteConfirm.message", { name: deleteTarget?.name ?? "" }) }}</p>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="t('common.delete')"
          confirm-icon="pi pi-trash"
          confirm-severity="danger"
          @cancel="showDeleteDialog = false"
          @confirm="executeDelete"
        />
      </template>
    </Dialog>
  </div>
</template>
