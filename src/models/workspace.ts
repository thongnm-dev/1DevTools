/** Types cho registry Workspace — khớp với DTO ở `src-tauri/src/models/workspace.rs`. */

export interface Workspace {
  id: number;
  name: string;
  project_path: string;
  icon: string;
  last_opened_at: string;
  /** Workflow tự động chạy khi file trong workspace này thay đổi — `null` = không. */
  auto_workflow_id: number | null;
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
  auto_workflow_id: number | null;
}

/** Panel đang hiển thị trong vùng nội dung chính của 1 workspace — chuyển qua icon rail (`WorkspaceRightSidebar.vue`).
 * "ide" gộp cả Explorer (cột 1) + vùng xem file (cột 2), giống bố cục VSCode. */
export type WorkspaceMainPanel = "git" | "terminal" | "agents" | "ide";
