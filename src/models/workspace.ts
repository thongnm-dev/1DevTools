/** Types cho registry Workspace — khớp với DTO ở `src-tauri/src/models/workspace.rs`. */

export interface Workspace {
  id: number;
  name: string;
  project_path: string;
  icon: string;
  last_opened_at: string;
}

export const DEFAULT_WORKSPACE_ICON = "pi pi-folder";

export interface CreateWorkspaceRequest {
  name: string;
  project_path: string;
  icon: string;
}

export interface UpdateWorkspaceRequest {
  name: string;
  icon: string;
}
