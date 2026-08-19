import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { friendlyError } from "@/tauri/commands/_base";
import {
  workflowCreate,
  workflowDelete,
  workflowDuplicate,
  workflowList,
  workflowSaveLayout,
  workflowStepCreate,
  workflowStepDelete,
  workflowStepList,
  workflowStepReorder,
  workflowStepUpdate,
  workflowUpdate,
} from "@/tauri/commands/workflow";
import { agentProviderModelListEnabled } from "@/tauri/commands/agent-provider-model";
import { aiUsageListAccounts } from "@/tauri/commands/ai-usage";
import { useAuthStore } from "@/app/stores/auth";
import type { AiAccount } from "@/models/ai-usage";
import type { AgentProviderModel } from "@/models/agent-provider-model";
import type { NodePos, StepRequest, Workflow, WorkflowStep, WorkflowStepType } from "@/models/workflow";
import { DEFAULT_WORKFLOW_ICON, STEP_TYPE_META } from "@/models/workflow";
import { useToast } from "@/shared/composables/useToast";

export function useWorkflow() {
  const { t } = useI18n();
  const toast = useToast();

  function username(): string {
    return useAuthStore().user?.username ?? "";
  }

  const workflows = ref<Workflow[]>([]);
  const aiAccounts = ref<AiAccount[]>([]);
  const models = ref<AgentProviderModel[]>([]);
  const activeId = ref<number | null>(null);
  const activeSteps = ref<WorkflowStep[]>([]);
  const selectedStepId = ref<number | null>(null);
  const isLoading = ref(false);
  const error = ref("");

  const activeWorkflow = computed(() => workflows.value.find((w) => w.id === activeId.value) ?? null);
  const selectedStep = computed(() => activeSteps.value.find((s) => s.id === selectedStepId.value) ?? null);

  async function loadSteps(workflowId: number) {
    try {
      activeSteps.value = await workflowStepList(workflowId);
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  async function loadWorkflows() {
    isLoading.value = true;
    error.value = "";
    try {
      workflows.value = await workflowList(username());
      if (activeId.value === null && workflows.value.length > 0) {
        activeId.value = workflows.value[0].id;
      }
      if (activeId.value !== null) {
        await loadSteps(activeId.value);
      }
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      isLoading.value = false;
    }
  }

  async function loadAiAccounts() {
    try {
      aiAccounts.value = await aiUsageListAccounts();
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  async function loadModels() {
    try {
      models.value = await agentProviderModelListEnabled();
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  void loadWorkflows();
  void loadAiAccounts();
  void loadModels();

  async function selectWorkflow(id: number) {
    activeId.value = id;
    selectedStepId.value = null;
    await loadSteps(id);
  }

  async function createWorkflow(name: string, description: string, icon?: string): Promise<Workflow | null> {
    error.value = "";
    try {
      const wf = await workflowCreate(username(), { name, description, icon: icon ?? DEFAULT_WORKFLOW_ICON });
      workflows.value.unshift(wf);
      activeId.value = wf.id;
      selectedStepId.value = null;
      activeSteps.value = [];
      toast.success(t("workflow.toast.created"));
      return wf;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
      return null;
    }
  }

  async function updateWorkflow(id: number, patch: { name?: string; description?: string; icon?: string }) {
    const wf = workflows.value.find((w) => w.id === id);
    if (!wf) return;
    error.value = "";
    try {
      const updated = await workflowUpdate(id, username(), {
        name: patch.name ?? wf.name,
        description: patch.description ?? wf.description,
        icon: patch.icon ?? wf.icon,
      });
      const idx = workflows.value.findIndex((w) => w.id === id);
      if (idx !== -1) workflows.value[idx] = { ...updated, step_count: wf.step_count };
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  async function deleteWorkflow(id: number) {
    error.value = "";
    try {
      await workflowDelete(id, username());
      workflows.value = workflows.value.filter((w) => w.id !== id);
      if (activeId.value === id) {
        activeId.value = workflows.value[0]?.id ?? null;
        selectedStepId.value = null;
        activeSteps.value = [];
        if (activeId.value !== null) await loadSteps(activeId.value);
      }
      toast.success(t("workflow.toast.deleted"));
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  async function duplicateWorkflow(id: number) {
    error.value = "";
    try {
      const wf = await workflowDuplicate(id, username());
      workflows.value.unshift(wf);
      activeId.value = wf.id;
      selectedStepId.value = null;
      await loadSteps(wf.id);
      toast.success(t("workflow.toast.duplicated"));
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  function toStepRequest(step: Partial<WorkflowStep> & { step_type: WorkflowStepType }): StepRequest {
    return {
      name: step.name ?? "New Step",
      step_type: step.step_type,
      skill_name: step.skill_name ?? "",
      prompt_id: step.prompt_id ?? null,
      runner_command: step.runner_command ?? "",
      ai_account_id: step.ai_account_id ?? null,
      description: step.description ?? "",
      icon: step.icon ?? STEP_TYPE_META[step.step_type].icon,
      step_order: step.step_order ?? activeSteps.value.length,
      is_latest_step: step.is_latest_step ?? false,
      model_id: step.model_id ?? null,
    };
  }

  async function addStep(
    afterStepId: number | null,
    step?: Partial<WorkflowStep>,
  ): Promise<WorkflowStep | null> {
    const wf = activeWorkflow.value;
    if (!wf) return null;

    const type: WorkflowStepType = step?.step_type ?? "custom";
    const idx = afterStepId !== null ? activeSteps.value.findIndex((s) => s.id === afterStepId) : -1;
    const stepOrder = idx !== -1 ? idx + 1 : activeSteps.value.length;

    error.value = "";
    try {
      const created = await workflowStepCreate(wf.id, toStepRequest({ ...step, step_type: type, step_order: stepOrder }));
      if (idx !== -1) {
        activeSteps.value.splice(idx + 1, 0, created);
      } else {
        activeSteps.value.push(created);
      }
      selectedStepId.value = created.id;
      wf.step_count += 1;
      if (idx !== -1) await reorderAndSync();
      return created;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
      return null;
    }
  }

  async function updateStep(stepId: number, patch: Partial<Omit<WorkflowStep, "id" | "workflow_id" | "created_at">>) {
    const step = activeSteps.value.find((s) => s.id === stepId);
    if (!step) return;
    error.value = "";
    try {
      const merged = { ...step, ...patch };
      const updated = await workflowStepUpdate(stepId, toStepRequest(merged));
      const idx = activeSteps.value.findIndex((s) => s.id === stepId);
      if (idx !== -1) activeSteps.value[idx] = updated;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  async function deleteStep(stepId: number) {
    const wf = activeWorkflow.value;
    error.value = "";
    try {
      await workflowStepDelete(stepId);
      activeSteps.value = activeSteps.value.filter((s) => s.id !== stepId);
      if (selectedStepId.value === stepId) selectedStepId.value = null;
      if (wf) wf.step_count = Math.max(0, wf.step_count - 1);
      await reorderAndSync();
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  /** Đồng bộ `step_order` (0-based theo thứ tự hiện có trong `activeSteps`) lên backend. */
  async function reorderAndSync() {
    const wf = activeWorkflow.value;
    if (!wf || activeSteps.value.length === 0) return;
    try {
      await workflowStepReorder(wf.id, activeSteps.value.map((s) => s.id));
      activeSteps.value.forEach((s, i) => (s.step_order = i));
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  async function saveLayout(workflowId: number, positions: Record<string, NodePos>) {
    try {
      await workflowSaveLayout(workflowId, username(), positions);
      const wf = workflows.value.find((w) => w.id === workflowId);
      if (wf) wf.layout = positions;
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  function selectStep(stepId: number | null) {
    selectedStepId.value = stepId;
  }

  return {
    workflows,
    aiAccounts,
    models,
    activeId,
    selectedStepId,
    activeWorkflow,
    activeSteps,
    selectedStep,
    isLoading,
    error,
    selectWorkflow,
    createWorkflow,
    updateWorkflow,
    deleteWorkflow,
    duplicateWorkflow,
    addStep,
    updateStep,
    deleteStep,
    saveLayout,
    selectStep,
  };
}

export type WorkflowApi = ReturnType<typeof useWorkflow>;
