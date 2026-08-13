/** Types cho màn hình Workflow — khớp với DTO ở `src-tauri/src/models/workflow.rs`. */

export type WorkflowStepType = "skill" | "prompt" | "runner" | "terminal" | "custom";

export interface WorkflowStep {
  id: string;
  name: string;
  step_type: WorkflowStepType;
  icon: string;
  description: string;
  is_latest_step: boolean;
  skill_id: number | null;
  prompt_id: number | null;
  runner_command: string | null;
  ai_account_id: number | null;
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
  steps: WorkflowStep[];
  layout: Record<string, NodePos>;
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
  steps: WorkflowStep[];
}

export const STEP_TYPE_META: Record<WorkflowStepType, { icon: string; badgeClass: string }> = {
  skill: { icon: "pi pi-book", badgeClass: "bg-sky-100 text-sky-700" },
  prompt: { icon: "pi pi-comment", badgeClass: "bg-fuchsia-100 text-fuchsia-700" },
  runner: { icon: "pi pi-play", badgeClass: "bg-violet-100 text-violet-700" },
  terminal: { icon: "pi pi-desktop", badgeClass: "bg-amber-100 text-amber-700" },
  custom: { icon: "pi pi-cog", badgeClass: "bg-canvas text-muted" },
};
