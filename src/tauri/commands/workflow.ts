import { safeInvoke } from "./_base";
import type { CreateWorkflowRequest, NodePos, UpdateWorkflowRequest, Workflow } from "@/models/workflow";

export function workflowList() {
  return safeInvoke<Workflow[]>("workflow_list");
}

export function workflowCreate(request: CreateWorkflowRequest) {
  return safeInvoke<Workflow>("workflow_create", { request });
}

export function workflowUpdate(id: number, request: UpdateWorkflowRequest) {
  return safeInvoke<Workflow>("workflow_update", { id, request });
}

export function workflowDelete(id: number) {
  return safeInvoke<void>("workflow_delete", { id });
}

export function workflowDuplicate(id: number) {
  return safeInvoke<Workflow>("workflow_duplicate", { id });
}

export function workflowSaveLayout(id: number, layout: Record<string, NodePos>) {
  return safeInvoke<void>("workflow_save_layout", { id, layout });
}
