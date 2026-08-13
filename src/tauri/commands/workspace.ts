import { safeInvoke } from "./_base";
import type { CreateWorkspaceRequest, UpdateWorkspaceRequest, Workspace } from "@/models/workspace";

export function workspaceList() {
  return safeInvoke<Workspace[]>("workspace_list");
}

export function workspaceCreate(request: CreateWorkspaceRequest) {
  return safeInvoke<Workspace>("workspace_create", { request });
}

export function workspaceUpdate(id: number, request: UpdateWorkspaceRequest) {
  return safeInvoke<Workspace>("workspace_update", { id, request });
}

export function workspaceRemove(id: number) {
  return safeInvoke<void>("workspace_remove", { id });
}

export function workspaceTouch(id: number) {
  return safeInvoke<void>("workspace_touch", { id });
}
