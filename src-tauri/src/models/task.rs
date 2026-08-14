//! Model cho tính năng AI Tasks / AI Cowork — theo dõi một task đi qua từng
//! step của 1 workflow (bảng `tasks` + `task_wf_proc` + `task_wf_proc_step`).

use serde::{Deserialize, Serialize};

/// Một hạng mục công việc (bảng `tasks`).
#[derive(Clone, Debug, Serialize)]
pub struct Task {
    pub id: i32,
    pub task_cd: String,
    pub task_name: String,
    pub category_id: String,
    pub is_complete: bool,
    pub completed_at: String,
    pub created_at: String,
    pub created_by: String,
    pub updated_at: String,
    pub updated_by: String,
    /// Tên workflow đang áp dụng cho step in_progress gần nhất — chỉ có ở kết quả `task_list`.
    #[serde(default)]
    pub current_wf_name: String,
    /// Tên step in_progress gần nhất — chỉ có ở kết quả `task_list`.
    #[serde(default)]
    pub current_step_name: String,
    /// Trạng thái step in_progress gần nhất — chỉ có ở kết quả `task_list`.
    #[serde(default)]
    pub current_step_status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub task_cd: String,
    #[serde(default)]
    pub task_name: String,
    #[serde(default)]
    pub category_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub task_cd: String,
    #[serde(default)]
    pub task_name: String,
    #[serde(default)]
    pub category_id: String,
    #[serde(default)]
    pub is_complete: bool,
}

/// Một lượt "chạy" workflow cho 1 task (bảng `task_wf_proc`).
#[derive(Clone, Debug, Serialize)]
pub struct TaskWfProc {
    pub id: i32,
    pub task_id: i32,
    pub wf_id: i32,
    pub latest_step_id: Option<i32>,
    pub created_at: String,
    pub created_by: String,
    pub updated_at: String,
    pub updated_by: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateWfProcRequest {
    pub task_id: i32,
    pub wf_id: i32,
}

/// Trạng thái của task tại 1 step cụ thể (bảng `task_wf_proc_step`).
#[derive(Clone, Debug, Serialize)]
pub struct TaskWfProcStep {
    pub id: i32,
    pub wf_proc_id: i32,
    pub wf_step_id: i32,
    pub status: String,
    pub created_at: String,
    pub created_by: String,
    pub updated_at: String,
    pub updated_by: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateWfProcStepRequest {
    pub wf_proc_id: i32,
    pub wf_step_id: i32,
    #[serde(default)]
    pub status: String,
}
