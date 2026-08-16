//! Business logic cho tính năng AI Tasks / AI Cowork.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::task_store;
use crate::models::task::{
    CreateTaskRequest, CreateWfProcRequest, CreateWfProcStepRequest, Task, TaskWfProc, TaskWfProcStep,
    UpdateTaskRequest,
};

const VALID_CATEGORIES: [&str; 4] = ["screen", "batch", "part", "other"];
const VALID_STEP_STATUSES: [&str; 4] = ["pending", "in_progress", "completed", "skipped"];

fn validate_category(raw: &str) -> AppResult<String> {
    let value = raw.trim().to_lowercase();
    if value.is_empty() {
        return Ok("other".to_string());
    }
    if !VALID_CATEGORIES.contains(&value.as_str()) {
        return Err(AppError::new(format!(
            "Invalid category '{value}'. Must be one of: {}.",
            VALID_CATEGORIES.join(", ")
        )));
    }
    Ok(value)
}

pub async fn create_task(created_by: &str, request: CreateTaskRequest) -> AppResult<Task> {
    let task_cd = request.task_cd.trim().to_string();
    if task_cd.is_empty() {
        return Err(AppError::new("Task code is required."));
    }
    let category_id = validate_category(&request.category_id)?;
    task_store::insert(&task_cd, request.task_name.trim(), &category_id, created_by).await
}

pub async fn list_tasks(keyword: Option<String>, is_complete: Option<bool>) -> AppResult<Vec<Task>> {
    task_store::select_list(keyword.as_deref(), is_complete).await
}

pub async fn update_task(id: i32, updated_by: &str, request: UpdateTaskRequest) -> AppResult<Task> {
    let task_cd = request.task_cd.trim().to_string();
    if task_cd.is_empty() {
        return Err(AppError::new("Task code is required."));
    }
    let category_id = validate_category(&request.category_id)?;
    task_store::update(id, &task_cd, request.task_name.trim(), &category_id, request.is_complete, updated_by)
        .await?
        .ok_or_else(|| AppError::new(format!("Task '{id}' not found.")))
}

pub async fn create_wf_proc(created_by: &str, request: CreateWfProcRequest) -> AppResult<TaskWfProc> {
    task_store::wf_proc_insert(request.task_id, request.wf_id, created_by).await
}

pub async fn list_wf_procs(task_id: i32) -> AppResult<Vec<TaskWfProc>> {
    task_store::wf_proc_select_by_task(task_id).await
}

pub async fn update_wf_proc(id: i32, latest_step_id: i32, updated_by: &str) -> AppResult<TaskWfProc> {
    task_store::wf_proc_update(id, latest_step_id, updated_by)
        .await?
        .ok_or_else(|| AppError::new(format!("Task workflow process '{id}' not found.")))
}

pub async fn delete_wf_proc(id: i32) -> AppResult<()> {
    task_store::wf_proc_delete(id).await?;
    Ok(())
}

pub async fn create_wf_proc_step(created_by: &str, request: CreateWfProcStepRequest) -> AppResult<TaskWfProcStep> {
    let status = if request.status.trim().is_empty() {
        "pending".to_string()
    } else {
        request.status.trim().to_lowercase()
    };
    if !VALID_STEP_STATUSES.contains(&status.as_str()) {
        return Err(AppError::new(format!(
            "Invalid status '{status}'. Must be one of: {}.",
            VALID_STEP_STATUSES.join(", ")
        )));
    }
    task_store::wf_proc_step_insert(request.wf_proc_id, request.wf_step_id, &status, created_by).await
}

pub async fn list_wf_proc_steps(wf_proc_id: i32) -> AppResult<Vec<TaskWfProcStep>> {
    task_store::wf_proc_step_select_by_proc(wf_proc_id).await
}

pub async fn update_wf_proc_step(id: i32, status: String, updated_by: &str) -> AppResult<TaskWfProcStep> {
    let status = status.trim().to_lowercase();
    if !VALID_STEP_STATUSES.contains(&status.as_str()) {
        return Err(AppError::new(format!(
            "Invalid status '{status}'. Must be one of: {}.",
            VALID_STEP_STATUSES.join(", ")
        )));
    }
    task_store::wf_proc_step_update(id, &status, updated_by)
        .await?
        .ok_or_else(|| AppError::new(format!("Task workflow step '{id}' not found.")))
}
