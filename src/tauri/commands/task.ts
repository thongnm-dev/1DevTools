import { safeInvoke } from "./_base";
import type {
  CreateTaskRequest,
  CreateWfProcRequest,
  CreateWfProcStepRequest,
  Task,
  TaskWfProc,
  TaskWfProcStep,
  UpdateTaskRequest,
} from "@/models/task";

export function taskCreate(username: string, request: CreateTaskRequest) {
  return safeInvoke<Task>("task_create", { username, request });
}

export function taskList(keyword?: string, isComplete?: boolean | null) {
  return safeInvoke<Task[]>("task_list", {
    keyword: keyword || null,
    isComplete: isComplete ?? null,
  });
}

export function taskUpdate(id: number, username: string, request: UpdateTaskRequest) {
  return safeInvoke<Task>("task_update", { id, username, request });
}

export function taskWfProcCreate(username: string, request: CreateWfProcRequest) {
  return safeInvoke<TaskWfProc>("task_wf_proc_create", { username, request });
}

export function taskWfProcList(taskId: number) {
  return safeInvoke<TaskWfProc[]>("task_wf_proc_list", { taskId });
}

export function taskWfProcUpdate(id: number, latestStepId: number, username: string) {
  return safeInvoke<TaskWfProc>("task_wf_proc_update", { id, latestStepId, username });
}

export function taskWfProcDelete(id: number) {
  return safeInvoke<void>("task_wf_proc_delete", { id });
}

export function taskWfProcStepCreate(username: string, request: CreateWfProcStepRequest) {
  return safeInvoke<TaskWfProcStep>("task_wf_proc_step_create", { username, request });
}

export function taskWfProcStepList(wfProcId: number) {
  return safeInvoke<TaskWfProcStep[]>("task_wf_proc_step_list", { wfProcId });
}

export function taskWfProcStepUpdate(id: number, status: string, username: string) {
  return safeInvoke<TaskWfProcStep>("task_wf_proc_step_update", { id, status, username });
}
