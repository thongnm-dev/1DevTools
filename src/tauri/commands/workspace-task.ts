import { safeInvoke } from "./_base";
import type { WorkspaceTaskLink } from "@/models/workspace";

export function workspaceTaskList(workspaceId: number) {
  return safeInvoke<WorkspaceTaskLink[]>("workspace_task_list", { workspaceId });
}

export function workspaceTaskAdd(workspaceId: number, taskId: number) {
  return safeInvoke<WorkspaceTaskLink>("workspace_task_add", { workspaceId, taskId });
}

export function workspaceTaskRemove(workspaceId: number, taskId: number) {
  return safeInvoke<void>("workspace_task_remove", { workspaceId, taskId });
}
