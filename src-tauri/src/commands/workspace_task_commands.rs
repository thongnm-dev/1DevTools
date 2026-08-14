//! Tauri IPC commands cho liên kết Workspace <-> Task (JSON cục bộ).

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::workspace_task::WorkspaceTask;
use crate::services::workspace_task_service;

#[tauri::command]
pub fn workspace_task_list(workspace_id: i64) -> Result<Vec<WorkspaceTask>, AppErrorPayload> {
    workspace_task_service::list_for_workspace(workspace_id).map_err(log_err)
}

#[tauri::command]
pub fn workspace_task_add(workspace_id: i64, task_id: i32) -> Result<WorkspaceTask, AppErrorPayload> {
    workspace_task_service::add(workspace_id, task_id).map_err(log_err)
}

#[tauri::command]
pub fn workspace_task_remove(workspace_id: i64, task_id: i32) -> Result<(), AppErrorPayload> {
    workspace_task_service::remove(workspace_id, task_id).map_err(log_err)
}
