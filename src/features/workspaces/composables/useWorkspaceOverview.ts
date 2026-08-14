import { computed, ref, watch } from "vue";
import { useAuthStore } from "@/app/stores/auth";
import { friendlyError } from "@/tauri/commands/_base";
import { taskWfProcList, taskWfProcStepList } from "@/tauri/commands/task";
import { workflowList, workflowStepList } from "@/tauri/commands/workflow";
import { gitLog, gitStatus } from "@/tauri/commands/git";
import { useWorkspaceTaskLinks } from "./useWorkspaceTaskLinks";
import { useWorkspaceTerminal } from "./useWorkspaceTerminal";
import type { Task } from "@/models/task";
import type { Workspace } from "@/models/workspace";
import type { GitCommit, GitFileChange, GitRepo } from "@/models/git";

export interface OverviewPlanStep {
  id: number;
  name: string;
  order: number;
  status: string;
  updatedAt: string;
}

/** "Plan" thực thi của 1 task = các step của workflow đang áp dụng (task_wf_proc gần nhất). */
export interface OverviewPlan {
  task: Task;
  workflowName: string;
  steps: OverviewPlanStep[];
}

export interface OverviewFile extends GitFileChange {
  staged: boolean;
}

export interface ActivityEntry {
  at: string;
  text: string;
}

function username(): string {
  return useAuthStore().user?.username ?? "";
}

/**
 * Tổng hợp dữ liệu cho màn Overview của 1 workspace: task đã thêm, session
 * terminal đang mở, plan (step workflow) đang chạy cho từng task, feed hoạt
 * động gần đây (step task + commit git), và file đang thay đổi (git status).
 * Toàn bộ đều đọc lại dữ liệu đã có sẵn — không có backend mới.
 */
export function useWorkspaceOverview(workspace: Workspace) {
  const taskLinks = useWorkspaceTaskLinks(workspace);
  const wsTerm = useWorkspaceTerminal();

  const sessions = computed(() => wsTerm.tabsFor(workspace.id));

  const plans = ref<OverviewPlan[]>([]);
  const plansLoading = ref(false);

  async function loadPlans() {
    plansLoading.value = true;
    try {
      const workflows = await workflowList(username());
      const results = await Promise.all(
        taskLinks.linkedTasks.value.map(async (task): Promise<OverviewPlan | null> => {
          const procs = await taskWfProcList(task.id);
          const latest = procs.at(-1);
          if (!latest) return null;
          const workflow = workflows.find((w) => w.id === latest.wf_id);
          const [wfSteps, procSteps] = await Promise.all([
            workflowStepList(latest.wf_id),
            taskWfProcStepList(latest.id),
          ]);
          const steps: OverviewPlanStep[] = wfSteps
            .sort((a, b) => a.step_order - b.step_order)
            .map((s) => {
              const procStep = procSteps.find((p) => p.wf_step_id === s.id);
              return {
                id: s.id,
                name: s.name,
                order: s.step_order,
                status: procStep?.status ?? "pending",
                updatedAt: procStep?.updated_at ?? "",
              };
            });
          return { task, workflowName: workflow?.name ?? "—", steps };
        }),
      );
      plans.value = results.filter((p): p is OverviewPlan => p !== null);
    } catch {
      plans.value = [];
    } finally {
      plansLoading.value = false;
    }
  }

  watch(taskLinks.linkedTasks, () => void loadPlans(), { immediate: true });

  const files = ref<OverviewFile[]>([]);
  const commits = ref<GitCommit[]>([]);
  const gitError = ref("");

  async function loadGitData(repo: GitRepo) {
    gitError.value = "";
    try {
      const [status, log] = await Promise.all([gitStatus(repo.path), gitLog(repo.path, 10)]);
      files.value = [
        ...status.staged.map((f) => ({ ...f, staged: true })),
        ...status.unstaged.map((f) => ({ ...f, staged: false })),
      ];
      commits.value = log;
    } catch (e) {
      gitError.value = friendlyError(e);
    }
  }

  const activity = computed<ActivityEntry[]>(() => {
    const taskEntries: ActivityEntry[] = plans.value.flatMap((plan) =>
      plan.steps
        .filter((s) => s.status !== "pending" && s.updatedAt)
        .map((s) => ({
          at: s.updatedAt,
          text: `${plan.task.task_cd} · ${s.name} → ${s.status}`,
        })),
    );
    const commitEntries: ActivityEntry[] = commits.value.map((c) => ({
      at: c.date,
      text: `${c.short_hash} ${c.subject} (${c.author_name})`,
    }));
    return [...taskEntries, ...commitEntries]
      .sort((a, b) => new Date(b.at).getTime() - new Date(a.at).getTime())
      .slice(0, 15);
  });

  return {
    linkedTasks: taskLinks.linkedTasks,
    tasksError: taskLinks.error,
    sessions,
    plans,
    plansLoading,
    files,
    commits,
    gitError,
    activity,
    loadGitData,
  };
}

export type WorkspaceOverviewApi = ReturnType<typeof useWorkspaceOverview>;
