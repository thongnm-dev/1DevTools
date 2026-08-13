//! Tauri IPC commands cho màn hình Workflow (CRUD + layout canvas).

use std::collections::HashMap;

use crate::app::error::{log_err, AppError, AppErrorPayload};
use crate::database::workflow_store;
use crate::models::workflow::{CreateWorkflowRequest, NodePos, UpdateWorkflowRequest, Workflow};

/// Danh sách toàn bộ workflow, mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub fn workflow_list() -> Result<Vec<Workflow>, AppErrorPayload> {
    let data = workflow_store::load().map_err(log_err)?;
    let mut workflows = data.workflows;
    workflows.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(workflows)
}

/// Tạo workflow mới (chưa có step).
#[tauri::command]
pub fn workflow_create(request: CreateWorkflowRequest) -> Result<Workflow, AppErrorPayload> {
    let mut data = workflow_store::load().map_err(log_err)?;
    data.next_id += 1;
    let now = chrono::Local::now().to_rfc3339();
    let workflow = Workflow {
        id: data.next_id,
        name: request.name,
        description: request.description,
        steps: Vec::new(),
        layout: HashMap::new(),
        created_at: now.clone(),
        updated_at: now,
    };
    data.workflows.push(workflow.clone());
    workflow_store::save(&data).map_err(log_err)?;
    Ok(workflow)
}

/// Cập nhật workflow — ghi đè `name`/`description`/toàn bộ `steps`.
#[tauri::command]
pub fn workflow_update(id: i64, request: UpdateWorkflowRequest) -> Result<Workflow, AppErrorPayload> {
    let mut data = workflow_store::load().map_err(log_err)?;
    let workflow = data
        .workflows
        .iter_mut()
        .find(|w| w.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Workflow #{id} không tồn tại"))))?;
    workflow.name = request.name;
    workflow.description = request.description;
    workflow.steps = request.steps;
    workflow.updated_at = chrono::Local::now().to_rfc3339();
    let result = workflow.clone();
    workflow_store::save(&data).map_err(log_err)?;
    Ok(result)
}

/// Xoá workflow.
#[tauri::command]
pub fn workflow_delete(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = workflow_store::load().map_err(log_err)?;
    data.workflows.retain(|w| w.id != id);
    workflow_store::save(&data).map_err(log_err)?;
    Ok(())
}

/// Nhân bản workflow (kèm steps, không kèm layout — auto layout lại ở frontend).
#[tauri::command]
pub fn workflow_duplicate(id: i64) -> Result<Workflow, AppErrorPayload> {
    let mut data = workflow_store::load().map_err(log_err)?;
    let source = data
        .workflows
        .iter()
        .find(|w| w.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Workflow #{id} không tồn tại"))))?
        .clone();

    data.next_id += 1;
    let now = chrono::Local::now().to_rfc3339();
    let copy = Workflow {
        id: data.next_id,
        name: format!("{} (copy)", source.name),
        description: source.description,
        steps: source.steps,
        layout: HashMap::new(),
        created_at: now.clone(),
        updated_at: now,
    };
    data.workflows.push(copy.clone());
    workflow_store::save(&data).map_err(log_err)?;
    Ok(copy)
}

/// Ghi lại vị trí node trên canvas (không đụng tới `steps`).
#[tauri::command]
pub fn workflow_save_layout(id: i64, layout: HashMap<String, NodePos>) -> Result<(), AppErrorPayload> {
    let mut data = workflow_store::load().map_err(log_err)?;
    if let Some(workflow) = data.workflows.iter_mut().find(|w| w.id == id) {
        workflow.layout = layout;
        workflow_store::save(&data).map_err(log_err)?;
    }
    Ok(())
}
