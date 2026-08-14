import {
  taskWfProcCreate,
  taskWfProcList,
  taskWfProcStepCreate,
  taskWfProcStepList,
  taskWfProcStepUpdate,
  taskWfProcUpdate,
} from "@/tauri/commands/task";
import type { WorkflowStep } from "@/models/workflow";

/**
 * Ghi nhận tiến trình workflow cho 1 task khi 1 step của nó được chạy:
 *  - Tạo `task_wf_proc` cho (task, workflow) nếu chưa có.
 *  - Cập nhật `latest_step_id` = step đang chạy.
 *  - Tạo/cập nhật `task_wf_proc_step` cho (proc, step) thành `in_progress`.
 *
 * Dùng chung bởi AI Cowork (mỗi lần mở terminal cho 1 step) và Workspace Tasks
 * (mỗi lần bấm Run cho step cuối cùng đã chạy tới).
 */
export function useTaskWorkflowProgress() {
  async function recordStepProgress(taskId: number, wfId: number, step: WorkflowStep, username: string) {
    const procs = await taskWfProcList(taskId);
    let proc = procs.find((p) => p.wf_id === wfId);
    if (!proc) {
      proc = await taskWfProcCreate(username, { task_id: taskId, wf_id: wfId });
    }
    if (proc.latest_step_id !== step.id) {
      proc = await taskWfProcUpdate(proc.id, step.id, username);
    }
    const procSteps = await taskWfProcStepList(proc.id);
    const existing = procSteps.find((s) => s.wf_step_id === step.id);
    if (!existing) {
      await taskWfProcStepCreate(username, { wf_proc_id: proc.id, wf_step_id: step.id, status: "in_progress" });
    } else if (existing.status !== "in_progress") {
      await taskWfProcStepUpdate(existing.id, "in_progress", username);
    }
  }

  return { recordStepProgress };
}
