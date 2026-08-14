import { computed, ref, watch } from "vue";
import { useAuthStore } from "@/app/stores/auth";
import { friendlyError } from "@/tauri/commands/_base";
import { aiUsageOpenTerminal } from "@/tauri/commands/ai-usage";
import { aiModelList, workflowList, workflowStepList } from "@/tauri/commands/workflow";
import { taskWfProcList } from "@/tauri/commands/task";
import { useTaskWorkflowProgress } from "@/features/task/composables/useTaskWorkflowProgress";
import { aiModelFlag } from "@/models/workflow";
import type { AiModel, Workflow, WorkflowStep } from "@/models/workflow";
import type { Task } from "@/models/task";
import { useToast } from "@/shared/composables/useToast";

export function useCowork() {
  const toast = useToast();
  const taskProgress = useTaskWorkflowProgress();

  function username(): string {
    return useAuthStore().user?.username ?? "";
  }

  const workDir = ref("");

  const workflows = ref<Workflow[]>([]);
  const models = ref<AiModel[]>([]);
  const selectedWorkflowId = ref<number | null>(null);
  const appliedWorkflowId = ref<number | null>(null);
  const steps = ref<WorkflowStep[]>([]);
  const isApplying = ref(false);

  const selectedTasks = ref<Task[]>([]);
  const confirmedTaskIds = ref<number[]>([]);
  const currentStepId = ref<number | null>(null);

  const error = ref("");

  const confirmedTasks = computed(() =>
    selectedTasks.value.filter((t) => confirmedTaskIds.value.includes(t.id)),
  );

  async function loadWorkflows() {
    try {
      workflows.value = await workflowList(username());
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  async function loadModels() {
    try {
      models.value = await aiModelList();
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  void loadWorkflows();
  void loadModels();

  async function applyWorkflow() {
    if (selectedWorkflowId.value === null) return;
    isApplying.value = true;
    error.value = "";
    try {
      steps.value = (await workflowStepList(selectedWorkflowId.value)).sort((a, b) => a.step_order - b.step_order);
      appliedWorkflowId.value = selectedWorkflowId.value;
      await refreshCurrentStep();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      isApplying.value = false;
    }
  }

  function addTask(task: Task) {
    if (!selectedTasks.value.some((t) => t.id === task.id)) {
      selectedTasks.value.push(task);
    }
  }

  function removeTask(id: number) {
    selectedTasks.value = selectedTasks.value.filter((t) => t.id !== id);
    confirmedTaskIds.value = confirmedTaskIds.value.filter((i) => i !== id);
  }

  function toggleConfirm(id: number) {
    if (confirmedTaskIds.value.includes(id)) {
      confirmedTaskIds.value = confirmedTaskIds.value.filter((i) => i !== id);
    } else {
      confirmedTaskIds.value = [...confirmedTaskIds.value, id];
    }
  }

  /** Step task đang đứng — lấy theo task đầu tiên trong danh sách đã confirm. */
  async function refreshCurrentStep() {
    currentStepId.value = null;
    const firstId = confirmedTaskIds.value[0];
    if (!firstId || appliedWorkflowId.value === null) return;
    try {
      const procs = await taskWfProcList(firstId);
      const proc = procs.find((p) => p.wf_id === appliedWorkflowId.value);
      currentStepId.value = proc?.latest_step_id ?? steps.value[0]?.id ?? null;
    } catch {
      currentStepId.value = null;
    }
  }

  watch([confirmedTaskIds, appliedWorkflowId], () => void refreshCurrentStep());

  function resolveModelFlag(modelId: number | null): string | undefined {
    if (modelId === null) return undefined;
    const model = models.value.find((m) => m.id === modelId);
    return model ? aiModelFlag(model) : undefined;
  }

  /**
   * Ghi nhận tiến trình workflow cho các task đã confirm khi mở terminal cho 1 step:
   *  - Tạo `task_wf_proc` cho (task, workflow đang áp dụng) nếu chưa có.
   *  - Cập nhật `latest_step_id` = step đang mở.
   *  - Tạo/cập nhật `task_wf_proc_step` cho (proc, step) thành `in_progress`.
   */
  async function registerTaskWorkflowProgress(step: WorkflowStep) {
    const wfId = appliedWorkflowId.value;
    if (wfId === null) return;
    const user = username();
    for (const task of confirmedTasks.value) {
      await taskProgress.recordStepProgress(task.id, wfId, step, user);
    }
  }

  /** Ghép prompt truyền cho `claude`: `/<skill-name> [CATEGORY] <task_cd>`. */
  function buildTaskSkillPrompt(step: WorkflowStep, task: Task): string {
    return `/${step.skill_name} [${task.category_id.toUpperCase()}] ${task.task_cd}`;
  }

  async function openStepTerminal(step: WorkflowStep, configDir: string) {
    if (confirmedTasks.value.length === 0) {
      toast.error("Confirm at least one task first.");
      return;
    }
    if (!workDir.value.trim()) {
      toast.error("Choose a working directory first.");
      return;
    }
    if (!configDir.trim()) {
      toast.error("No active AI account.");
      return;
    }
    error.value = "";
    try {
      await registerTaskWorkflowProgress(step);
      await refreshCurrentStep();
      const modelFlag = resolveModelFlag(step.model_id);
      for (const task of confirmedTasks.value) {
        await aiUsageOpenTerminal(configDir, workDir.value.trim(), buildTaskSkillPrompt(step, task), modelFlag);
      }
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  return {
    workDir,
    workflows,
    models,
    selectedWorkflowId,
    appliedWorkflowId,
    steps,
    isApplying,
    selectedTasks,
    confirmedTaskIds,
    confirmedTasks,
    currentStepId,
    error,
    applyWorkflow,
    addTask,
    removeTask,
    toggleConfirm,
    openStepTerminal,
  };
}

export type CoworkApi = ReturnType<typeof useCowork>;
