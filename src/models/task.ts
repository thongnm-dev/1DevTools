/** Types cho tính năng AI Tasks / AI Cowork — khớp với DTO ở `src-tauri/src/models/task.rs`. */

export type TaskCategory = "screen" | "batch" | "part" | "other";

export const TASK_CATEGORY_OPTIONS: { label: string; value: TaskCategory }[] = [
  { label: "Screen", value: "screen" },
  { label: "Batch", value: "batch" },
  { label: "Part", value: "part" },
  { label: "Other", value: "other" },
];

export const TASK_CATEGORY_META: Record<TaskCategory, { badgeClass: string }> = {
  screen: { badgeClass: "bg-sky-100 text-sky-700" },
  batch: { badgeClass: "bg-violet-100 text-violet-700" },
  part: { badgeClass: "bg-amber-100 text-amber-700" },
  other: { badgeClass: "bg-canvas text-muted" },
};

export type WfProcStepStatus = "pending" | "in_progress" | "completed" | "skipped";

export const STEP_STATUS_META: Record<WfProcStepStatus, { badgeClass: string }> = {
  pending: { badgeClass: "bg-canvas text-muted" },
  in_progress: { badgeClass: "bg-sky-100 text-sky-700" },
  completed: { badgeClass: "bg-emerald-100 text-emerald-700" },
  skipped: { badgeClass: "bg-canvas text-muted" },
};

export interface Task {
  id: number;
  task_cd: string;
  task_name: string;
  category_id: string;
  is_complete: boolean;
  completed_at: string;
  created_at: string;
  created_by: string;
  updated_at: string;
  updated_by: string;
  /** Tên workflow đang áp dụng cho step in_progress gần nhất — chỉ có ở kết quả `taskList`. */
  current_wf_name: string;
  /** Tên step in_progress gần nhất — chỉ có ở kết quả `taskList`. */
  current_step_name: string;
  /** Trạng thái step in_progress gần nhất — chỉ có ở kết quả `taskList`. */
  current_step_status: string;
}

export interface CreateTaskRequest {
  task_cd: string;
  task_name: string;
  category_id: string;
}

export interface UpdateTaskRequest {
  task_cd: string;
  task_name: string;
  category_id: string;
  is_complete: boolean;
}

export interface TaskWfProc {
  id: number;
  task_id: number;
  wf_id: number;
  latest_step_id: number | null;
  created_at: string;
  created_by: string;
  updated_at: string;
  updated_by: string;
}

export interface CreateWfProcRequest {
  task_id: number;
  wf_id: number;
}

export interface TaskWfProcStep {
  id: number;
  wf_proc_id: number;
  wf_step_id: number;
  status: string;
  created_at: string;
  created_by: string;
  updated_at: string;
  updated_by: string;
}

export interface CreateWfProcStepRequest {
  wf_proc_id: number;
  wf_step_id: number;
  status: string;
}
