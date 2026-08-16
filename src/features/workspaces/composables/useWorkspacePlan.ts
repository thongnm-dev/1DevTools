import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useAuthStore } from "@/app/stores/auth";
import { friendlyError } from "@/tauri/commands/_base";
import {
  taskWfProcCreate,
  taskWfProcDelete,
  taskWfProcList,
  taskWfProcStepCreate,
  taskWfProcStepList,
  taskWfProcStepUpdate,
} from "@/tauri/commands/task";
import { workflowList, workflowStepList } from "@/tauri/commands/workflow";
import { promptList } from "@/tauri/commands/prompt";
import { aiUsageSetActive } from "@/tauri/commands/ai-usage";
import { useWorkspaceTerminal } from "./useWorkspaceTerminal";
import { useWorkspaceTaskLinks } from "./useWorkspaceTaskLinks";
import { useToast } from "@/shared/composables/useToast";
import type { Task, TaskWfProc } from "@/models/task";
import type { Workflow } from "@/models/workflow";
import type { Workspace } from "@/models/workspace";

export interface PlanStep {
  /** TaskWfProcStep.id — 0 nếu chưa có record */
  id: number;
  wfStepId: number;
  name: string;
  order: number;
  stepType: string;
  icon: string;
  description: string;
  status: string;
  updatedAt: string;
  /** Execution fields — từ WorkflowStep */
  skillName: string;
  runnerCommand: string;
  promptId: number | null;
  aiAccountId: number | null;
  modelId: number | null;
}

export interface Plan {
  proc: TaskWfProc;
  task: Task;
  workflow: Workflow;
  steps: PlanStep[];
}

export interface TaskPlanEntry {
  proc: TaskWfProc;
  task: Task;
  steps: PlanStep[];
}

export interface WorkflowPlanGroup {
  workflow: Workflow;
  taskEntries: TaskPlanEntry[];
}

export function useWorkspacePlan(workspace: Workspace) {
  const toast = useToast();
  const { t } = useI18n();
  const auth = useAuthStore();
  const term = useWorkspaceTerminal();

  function username() {
    return auth.user?.username ?? "";
  }

  const taskLinks = useWorkspaceTaskLinks(workspace);
  const plans = ref<Plan[]>([]);
  const workflows = ref<Workflow[]>([]);
  const loading = ref(false);
  const error = ref("");

  /** Set proc.id đang chạy — dùng để disable nút run và hiện loading */
  const runningProcIds = ref(new Set<number>());
  /** Set workflow.id đang chạy "Run All" */
  const runningWfIds = ref(new Set<number>());

  const groupedPlans = computed<WorkflowPlanGroup[]>(() => {
    const map = new Map<number, WorkflowPlanGroup>();
    for (const plan of plans.value) {
      if (!map.has(plan.workflow.id)) {
        map.set(plan.workflow.id, { workflow: plan.workflow, taskEntries: [] });
      }
      map.get(plan.workflow.id)!.taskEntries.push({
        proc: plan.proc,
        task: plan.task,
        steps: plan.steps,
      });
    }
    return [...map.values()];
  });

  async function loadPlans() {
    loading.value = true;
    error.value = "";
    try {
      const wfs = await workflowList(username());
      workflows.value = wfs;

      const perTask = await Promise.all(
        taskLinks.linkedTasks.value.map(async (task) => {
          const procs = await taskWfProcList(task.id);
          const perProc = await Promise.all(
            procs.map(async (proc): Promise<Plan | null> => {
              const wf = wfs.find((w) => w.id === proc.wf_id);
              if (!wf) return null;
              const [wfSteps, procSteps] = await Promise.all([
                workflowStepList(proc.wf_id),
                taskWfProcStepList(proc.id),
              ]);
              const steps: PlanStep[] = wfSteps
                .sort((a, b) => a.step_order - b.step_order)
                .map((s) => {
                  const ps = procSteps.find((p) => p.wf_step_id === s.id);
                  return {
                    id: ps?.id ?? 0,
                    wfStepId: s.id,
                    name: s.name,
                    order: s.step_order,
                    stepType: s.step_type,
                    icon: s.icon || "pi pi-circle",
                    description: s.description,
                    status: ps?.status ?? "pending",
                    updatedAt: ps?.updated_at ?? "",
                    skillName: s.skill_name,
                    runnerCommand: s.runner_command,
                    promptId: s.prompt_id,
                    aiAccountId: s.ai_account_id,
                    modelId: s.model_id,
                  };
                });
              return { proc, task, workflow: wf, steps };
            }),
          );
          return perProc.filter((p): p is Plan => p !== null);
        }),
      );

      plans.value = perTask.flat();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      loading.value = false;
    }
  }

  watch(taskLinks.linkedTasks, () => void loadPlans(), { immediate: true });

  // ─── Plan CRUD ────────────────────────────────────────────────────────────

  async function _createProcForTask(workflowId: number, taskId: number) {
    const proc = await taskWfProcCreate(username(), { task_id: taskId, wf_id: workflowId });
    const wfSteps = await workflowStepList(workflowId);
    await Promise.all(
      wfSteps.map((step) =>
        taskWfProcStepCreate(username(), {
          wf_proc_id: proc.id,
          wf_step_id: step.id,
          status: "pending",
        }),
      ),
    );
  }

  async function createPlan(workflowId: number, taskIds: number[]) {
    try {
      await Promise.all(taskIds.map((taskId) => _createProcForTask(workflowId, taskId)));
      await loadPlans();
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function updatePlanTasks(group: WorkflowPlanGroup, newTaskIds: number[]) {
    const currentTaskIds = group.taskEntries.map((e) => e.task.id);
    const toAdd = newTaskIds.filter((id) => !currentTaskIds.includes(id));
    const toRemove = group.taskEntries.filter((e) => !newTaskIds.includes(e.task.id));
    try {
      await Promise.all([
        ...toAdd.map((taskId) => _createProcForTask(group.workflow.id, taskId)),
        ...toRemove.map((entry) => taskWfProcDelete(entry.proc.id)),
      ]);
      await loadPlans();
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function deletePlan(procId: number) {
    plans.value = plans.value.filter((p) => p.proc.id !== procId);
    try {
      await taskWfProcDelete(procId);
    } catch (e) {
      toast.error(friendlyError(e));
      await loadPlans();
    }
  }

  async function updateStepStatus(procId: number, stepId: number, status: string) {
    const plan = plans.value.find((p) => p.proc.id === procId);
    if (plan) {
      const step = plan.steps.find((s) => s.id === stepId);
      if (step) step.status = status;
    }
    try {
      await taskWfProcStepUpdate(stepId, status, username());
    } catch (e) {
      toast.error(friendlyError(e));
      await loadPlans();
    }
  }

  // ─── Execution ────────────────────────────────────────────────────────────

  /**
   * Bước tiếp theo cần chạy: ưu tiên step đang `in_progress` (resume),
   * rồi mới tới `pending`. Trả về null nếu tất cả đã done/skipped.
   */
  function findNextStep(entry: TaskPlanEntry): PlanStep | null {
    return (
      entry.steps.find((s) => s.status === "in_progress") ??
      entry.steps.find((s) => s.status === "pending") ??
      null
    );
  }

  function isEntryDone(entry: TaskPlanEntry): boolean {
    return entry.steps.every((s) => s.status === "completed" || s.status === "skipped");
  }

  async function _executeStep(entry: TaskPlanEntry, step: PlanStep) {
    // Mark in_progress (optimistic + DB)
    if (step.status !== "in_progress" && step.id > 0) {
      await updateStepStatus(entry.proc.id, step.id, "in_progress");
    }

    // Switch AI account nếu step yêu cầu
    if (step.aiAccountId !== null) {
      try {
        await aiUsageSetActive(step.aiAccountId);
      } catch {
        // non-critical, tiếp tục
      }
    }

    const tabTitle = `${step.name} · ${entry.task.task_cd}`;

    switch (step.stepType) {
      case "runner":
      case "terminal":
        term.addTab(
          workspace.id,
          tabTitle,
          workspace.project_path,
          step.runnerCommand.trim() || undefined,
        );
        break;

      case "skill": {
        if (!step.skillName.trim()) {
          toast.error(t("workspaces.plan.run.skillNotSet", { name: step.name }));
          return;
        }
        const prompt = `/${step.skillName.trim()} [${entry.task.category_id.toUpperCase()}] ${entry.task.task_cd}`;
        await navigator.clipboard.writeText(prompt);
        toast.success(t("workspaces.plan.run.skillCopied", { prompt }));
        term.addTab(workspace.id, tabTitle, workspace.project_path);
        break;
      }

      case "prompt": {
        if (!step.promptId) {
          toast.error(t("workspaces.plan.run.promptNotSet", { name: step.name }));
          return;
        }
        try {
          const all = await promptList();
          const p = all.find((pr) => pr.id === step.promptId);
          if (!p) {
            toast.error(t("workspaces.plan.run.promptNotFound", { name: step.name }));
            return;
          }
          await navigator.clipboard.writeText(p.body);
          toast.success(t("workspaces.plan.run.promptCopied", { title: p.title }));
          term.addTab(workspace.id, tabTitle, workspace.project_path);
        } catch (e) {
          toast.error(friendlyError(e));
        }
        break;
      }

      case "custom":
        toast.info(t("workspaces.plan.run.manualStep", { name: step.name }));
        break;
    }
  }

  /** Chạy bước tiếp theo (in_progress → pending) của 1 task trong plan */
  async function runPlan(entry: TaskPlanEntry) {
    if (runningProcIds.value.has(entry.proc.id)) return;
    const nextStep = findNextStep(entry);
    if (!nextStep) {
      toast.info(t("workspaces.plan.run.allDone", { code: entry.task.task_cd }));
      return;
    }
    runningProcIds.value = new Set(runningProcIds.value).add(entry.proc.id);
    try {
      await _executeStep(entry, nextStep);
    } catch (e) {
      toast.error(friendlyError(e));
    } finally {
      const s = new Set(runningProcIds.value);
      s.delete(entry.proc.id);
      runningProcIds.value = s;
    }
  }

  /** Chạy bước tiếp theo cho tất cả tasks trong 1 workflow group */
  async function runAllPlans(group: WorkflowPlanGroup) {
    if (runningWfIds.value.has(group.workflow.id)) return;
    const pending = group.taskEntries.filter((e) => !isEntryDone(e));
    if (!pending.length) {
      toast.info(t("workspaces.plan.run.groupAllDone", { name: group.workflow.name }));
      return;
    }
    runningWfIds.value = new Set(runningWfIds.value).add(group.workflow.id);
    try {
      await Promise.all(pending.map((e) => runPlan(e)));
    } finally {
      const s = new Set(runningWfIds.value);
      s.delete(group.workflow.id);
      runningWfIds.value = s;
    }
  }

  return {
    linkedTasks: taskLinks.linkedTasks,
    plans,
    groupedPlans,
    workflows,
    loading,
    error,
    runningProcIds,
    runningWfIds,
    createPlan,
    updatePlanTasks,
    deletePlan,
    updateStepStatus,
    runPlan,
    runAllPlans,
    findNextStep,
    isEntryDone,
    refresh: loadPlans,
  };
}

export type WorkspacePlanApi = ReturnType<typeof useWorkspacePlan>;
