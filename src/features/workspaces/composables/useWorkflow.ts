import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { friendlyError } from "@/tauri/commands/_base";
import { workflowCreate, workflowDelete, workflowDuplicate, workflowList, workflowSaveLayout, workflowUpdate } from "@/tauri/commands/workflow";
import { aiUsageListAccounts } from "@/tauri/commands/ai-usage";
import type { AiAccount } from "@/models/ai-usage";
import type { NodePos, Workflow, WorkflowStep, WorkflowStepType } from "@/models/workflow";
import { DEFAULT_WORKFLOW_ICON, STEP_TYPE_META } from "@/models/workflow";
import { useToast } from "@/shared/composables/useToast";

export function useWorkflow() {
  const { t } = useI18n();
  const toast = useToast();

  const workflows = ref<Workflow[]>([]);
  const aiAccounts = ref<AiAccount[]>([]);
  const activeId = ref<number | null>(null);
  const selectedStepId = ref<string | null>(null);
  const isLoading = ref(false);
  const error = ref("");

  const activeWorkflow = computed(() => workflows.value.find((w) => w.id === activeId.value) ?? null);
  const activeSteps = computed(() => activeWorkflow.value?.steps ?? []);
  const selectedStep = computed(() => activeSteps.value.find((s) => s.id === selectedStepId.value) ?? null);

  async function loadWorkflows() {
    isLoading.value = true;
    error.value = "";
    try {
      workflows.value = await workflowList();
      if (activeId.value === null && workflows.value.length > 0) {
        activeId.value = workflows.value[0].id;
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

  void loadWorkflows();
  void loadAiAccounts();

  function selectWorkflow(id: number) {
    activeId.value = id;
    selectedStepId.value = null;
  }

  async function createWorkflow(name: string, description: string, icon?: string): Promise<Workflow | null> {
    error.value = "";
    try {
      const wf = await workflowCreate({ name, description, icon: icon ?? DEFAULT_WORKFLOW_ICON });
      workflows.value.unshift(wf);
      activeId.value = wf.id;
      selectedStepId.value = null;
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
      const updated = await workflowUpdate(id, {
        name: patch.name ?? wf.name,
        description: patch.description ?? wf.description,
        icon: patch.icon ?? wf.icon,
        steps: wf.steps,
      });
      const idx = workflows.value.findIndex((w) => w.id === id);
      if (idx !== -1) workflows.value[idx] = updated;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  async function deleteWorkflow(id: number) {
    error.value = "";
    try {
      await workflowDelete(id);
      workflows.value = workflows.value.filter((w) => w.id !== id);
      if (activeId.value === id) {
        activeId.value = workflows.value[0]?.id ?? null;
        selectedStepId.value = null;
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
      const wf = await workflowDuplicate(id);
      workflows.value.unshift(wf);
      activeId.value = wf.id;
      selectedStepId.value = null;
      toast.success(t("workflow.toast.duplicated"));
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  /** Ghi lại `steps` hiện tại của workflow đang active lên backend. */
  async function persistSteps() {
    const wf = activeWorkflow.value;
    if (!wf) return;
    try {
      const updated = await workflowUpdate(wf.id, { name: wf.name, description: wf.description, icon: wf.icon, steps: wf.steps });
      const idx = workflows.value.findIndex((w) => w.id === wf.id);
      if (idx !== -1) workflows.value[idx] = updated;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  async function addStep(afterStepId: string | null, step?: Partial<WorkflowStep>): Promise<WorkflowStep | null> {
    const wf = activeWorkflow.value;
    if (!wf) return null;

    const type: WorkflowStepType = step?.step_type ?? "custom";
    const meta = STEP_TYPE_META[type];

    const newStep: WorkflowStep = {
      id: crypto.randomUUID(),
      name: step?.name ?? "New Step",
      step_type: type,
      icon: step?.icon ?? meta.icon,
      description: step?.description ?? "",
      is_latest_step: step?.is_latest_step ?? false,
      skill_id: step?.skill_id ?? null,
      prompt_id: step?.prompt_id ?? null,
      runner_command: step?.runner_command ?? null,
      ai_account_id: step?.ai_account_id ?? null,
    };

    if (afterStepId !== null) {
      const idx = wf.steps.findIndex((s) => s.id === afterStepId);
      wf.steps.splice(idx !== -1 ? idx + 1 : wf.steps.length, 0, newStep);
    } else {
      wf.steps.push(newStep);
    }

    selectedStepId.value = newStep.id;
    await persistSteps();
    return newStep;
  }

  async function updateStep(stepId: string, patch: Partial<Omit<WorkflowStep, "id">>) {
    const wf = activeWorkflow.value;
    if (!wf) return;
    const step = wf.steps.find((s) => s.id === stepId);
    if (!step) return;
    Object.assign(step, patch);
    await persistSteps();
  }

  async function deleteStep(stepId: string) {
    const wf = activeWorkflow.value;
    if (!wf) return;
    wf.steps = wf.steps.filter((s) => s.id !== stepId);
    if (selectedStepId.value === stepId) selectedStepId.value = null;
    await persistSteps();
  }

  async function saveLayout(workflowId: number, positions: Record<string, NodePos>) {
    try {
      await workflowSaveLayout(workflowId, positions);
      const wf = workflows.value.find((w) => w.id === workflowId);
      if (wf) wf.layout = positions;
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  function selectStep(stepId: string | null) {
    selectedStepId.value = stepId;
  }

  return {
    workflows,
    aiAccounts,
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
