//! Tauri IPC commands cho màn hình Workflow (CRUD workflow + step + layout canvas).

use std::collections::HashMap;

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::workflow::{
    CreateWorkflowRequest, NodePos, StepRequest, UpdateWorkflowRequest, Workflow, WorkflowStep,
};
use crate::services::workflow_service;

/// Danh sách toàn bộ workflow của user, mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub async fn workflow_list(username: String) -> Result<Vec<Workflow>, AppErrorPayload> {
    workflow_service::list_workflows(&username).await.map_err(log_err)
}

/// Tạo workflow mới (chưa có step).
#[tauri::command]
pub async fn workflow_create(
    username: String,
    request: CreateWorkflowRequest,
) -> Result<Workflow, AppErrorPayload> {
    workflow_service::create_workflow(&username, request).await.map_err(log_err)
}

/// Cập nhật tên/mô tả/icon workflow.
#[tauri::command]
pub async fn workflow_update(
    id: i32,
    username: String,
    request: UpdateWorkflowRequest,
) -> Result<Workflow, AppErrorPayload> {
    workflow_service::update_workflow(id, &username, request).await.map_err(log_err)
}

/// Xoá workflow (cascade xoá toàn bộ step).
#[tauri::command]
pub async fn workflow_delete(id: i32, username: String) -> Result<(), AppErrorPayload> {
    workflow_service::delete_workflow(id, &username).await.map_err(log_err)
}

/// Nhân bản workflow (kèm steps, không kèm layout — auto layout lại ở frontend).
#[tauri::command]
pub async fn workflow_duplicate(id: i32, username: String) -> Result<Workflow, AppErrorPayload> {
    workflow_service::duplicate_workflow(id, &username).await.map_err(log_err)
}

/// Ghi lại vị trí node trên canvas (không đụng tới step).
#[tauri::command]
pub async fn workflow_save_layout(
    id: i32,
    username: String,
    layout: HashMap<String, NodePos>,
) -> Result<(), AppErrorPayload> {
    workflow_service::save_layout(id, &username, layout).await.map_err(log_err)
}

/// Danh sách step của 1 workflow, theo `step_order`.
#[tauri::command]
pub async fn workflow_step_list(workflow_id: i32) -> Result<Vec<WorkflowStep>, AppErrorPayload> {
    workflow_service::list_steps(workflow_id).await.map_err(log_err)
}

/// Thêm 1 step mới vào workflow.
#[tauri::command]
pub async fn workflow_step_create(
    workflow_id: i32,
    request: StepRequest,
) -> Result<WorkflowStep, AppErrorPayload> {
    workflow_service::create_step(workflow_id, request).await.map_err(log_err)
}

/// Cập nhật 1 step.
#[tauri::command]
pub async fn workflow_step_update(
    id: i32,
    request: StepRequest,
) -> Result<WorkflowStep, AppErrorPayload> {
    workflow_service::update_step(id, request).await.map_err(log_err)
}

/// Xoá 1 step.
#[tauri::command]
pub async fn workflow_step_delete(id: i32) -> Result<(), AppErrorPayload> {
    workflow_service::delete_step(id).await.map_err(log_err)
}

/// Sắp lại thứ tự step theo danh sách id truyền vào.
#[tauri::command]
pub async fn workflow_step_reorder(
    workflow_id: i32,
    step_ids: Vec<i32>,
) -> Result<(), AppErrorPayload> {
    workflow_service::reorder_steps(workflow_id, step_ids).await.map_err(log_err)
}
