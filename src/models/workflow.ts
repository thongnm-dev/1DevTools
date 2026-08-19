/** Types cho màn hình Workflow — khớp với DTO ở `src-tauri/src/models/workflow.rs`. */

export type WorkflowStepType = "skill" | "prompt" | "runner" | "terminal" | "custom";

export interface WorkflowStep {
  id: number;
  workflow_id: number;
  name: string;
  step_type: WorkflowStepType;
  icon: string;
  description: string;
  step_order: number;
  is_latest_step: boolean;
  /** Slug skill dùng khi `step_type = "skill"` (vd `create-plan`) — nhập tự do. */
  skill_name: string;
  prompt_id: number | null;
  runner_command: string;
  ai_account_id: number | null;
  /** Model AI dùng khi AI Cowork chạy step này (chỉ có ý nghĩa với `step_type = "skill"`). */
  model_id: number | null;
  created_at: string;
}

export interface NodePos {
  x: number;
  y: number;
}

export interface Workflow {
  id: number;
  name: string;
  description: string;
  icon: string;
  layout: Record<string, NodePos>;
  created_by: string;
  step_count: number;
  created_at: string;
  updated_at: string;
}

export const DEFAULT_WORKFLOW_ICON = "pi pi-sitemap";

export interface CreateWorkflowRequest {
  name: string;
  description: string;
  icon: string;
}

export interface UpdateWorkflowRequest {
  name: string;
  description: string;
  icon: string;
}

export interface StepRequest {
  name: string;
  step_type: WorkflowStepType;
  skill_name: string;
  prompt_id: number | null;
  runner_command: string;
  ai_account_id: number | null;
  description: string;
  icon: string;
  step_order: number;
  is_latest_step: boolean;
  model_id: number | null;
}

export const STEP_TYPE_META: Record<WorkflowStepType, { icon: string; badgeClass: string }> = {
  skill: { icon: "pi pi-book", badgeClass: "bg-sky-100 text-sky-700" },
  prompt: { icon: "pi pi-comment", badgeClass: "bg-fuchsia-100 text-fuchsia-700" },
  runner: { icon: "pi pi-play", badgeClass: "bg-violet-100 text-violet-700" },
  terminal: { icon: "pi pi-desktop", badgeClass: "bg-amber-100 text-amber-700" },
  custom: { icon: "pi pi-cog", badgeClass: "bg-canvas text-muted" },
};
