import { safeInvoke } from "./_base";
import type {
  CreateWorkflowRequest,
  NodePos,
  StepRequest,
  UpdateWorkflowRequest,
  Workflow,
  WorkflowStep,
} from "@/models/workflow";

export function workflowList(username: string) {
  return safeInvoke<Workflow[]>("workflow_list", { username });
}

export function workflowCreate(username: string, request: CreateWorkflowRequest) {
  return safeInvoke<Workflow>("workflow_create", { username, request });
}

export function workflowUpdate(id: number, username: string, request: UpdateWorkflowRequest) {
  return safeInvoke<Workflow>("workflow_update", { id, username, request });
}

export function workflowDelete(id: number, username: string) {
  return safeInvoke<void>("workflow_delete", { id, username });
}

export function workflowDuplicate(id: number, username: string) {
  return safeInvoke<Workflow>("workflow_duplicate", { id, username });
}

export function workflowSaveLayout(id: number, username: string, layout: Record<string, NodePos>) {
  return safeInvoke<void>("workflow_save_layout", { id, username, layout });
}

export function workflowStepList(workflowId: number) {
  return safeInvoke<WorkflowStep[]>("workflow_step_list", { workflowId });
}

export function workflowStepCreate(workflowId: number, request: StepRequest) {
  return safeInvoke<WorkflowStep>("workflow_step_create", { workflowId, request });
}

export function workflowStepUpdate(id: number, request: StepRequest) {
  return safeInvoke<WorkflowStep>("workflow_step_update", { id, request });
}

export function workflowStepDelete(id: number) {
  return safeInvoke<void>("workflow_step_delete", { id });
}

export function workflowStepReorder(workflowId: number, stepIds: number[]) {
  return safeInvoke<void>("workflow_step_reorder", { workflowId, stepIds });
}
