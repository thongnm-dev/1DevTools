//! Tauri IPC commands cho tính năng AI Tasks / AI Cowork.

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::task::{
    CreateTaskRequest, CreateWfProcRequest, CreateWfProcStepRequest, Task, TaskWfProc, TaskWfProcStep,
    UpdateTaskRequest,
};
use crate::services::task_service;

// ── tasks ────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn task_create(username: String, request: CreateTaskRequest) -> Result<Task, AppErrorPayload> {
    task_service::create_task(&username, request).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_list(
    keyword: Option<String>,
    is_complete: Option<bool>,
) -> Result<Vec<Task>, AppErrorPayload> {
    task_service::list_tasks(keyword, is_complete).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_update(
    id: i32,
    username: String,
    request: UpdateTaskRequest,
) -> Result<Task, AppErrorPayload> {
    task_service::update_task(id, &username, request).await.map_err(log_err)
}

// ── task_wf_proc ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn task_wf_proc_create(
    username: String,
    request: CreateWfProcRequest,
) -> Result<TaskWfProc, AppErrorPayload> {
    task_service::create_wf_proc(&username, request).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_wf_proc_list(task_id: i32) -> Result<Vec<TaskWfProc>, AppErrorPayload> {
    task_service::list_wf_procs(task_id).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_wf_proc_update(
    id: i32,
    latest_step_id: i32,
    username: String,
) -> Result<TaskWfProc, AppErrorPayload> {
    task_service::update_wf_proc(id, latest_step_id, &username).await.map_err(log_err)
}

// ── task_wf_proc_step ────────────────────────────────────────────────────

#[tauri::command]
pub async fn task_wf_proc_delete(id: i32) -> Result<(), AppErrorPayload> {
    task_service::delete_wf_proc(id).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_wf_proc_step_create(
    username: String,
    request: CreateWfProcStepRequest,
) -> Result<TaskWfProcStep, AppErrorPayload> {
    task_service::create_wf_proc_step(&username, request).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_wf_proc_step_list(wf_proc_id: i32) -> Result<Vec<TaskWfProcStep>, AppErrorPayload> {
    task_service::list_wf_proc_steps(wf_proc_id).await.map_err(log_err)
}

#[tauri::command]
pub async fn task_wf_proc_step_update(
    id: i32,
    status: String,
    username: String,
) -> Result<TaskWfProcStep, AppErrorPayload> {
    task_service::update_wf_proc_step(id, status, &username).await.map_err(log_err)
}
