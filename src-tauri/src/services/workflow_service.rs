//! Business logic cho màn hình Workflow (CRUD workflow + step + layout canvas).

use std::collections::HashMap;

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::workflow_store;
use crate::models::workflow::{
    CreateWorkflowRequest, NodePos, StepRequest, UpdateWorkflowRequest, Workflow, WorkflowStep,
};

pub async fn list_workflows(created_by: &str) -> AppResult<Vec<Workflow>> {
    workflow_store::list_all(created_by).await
}

pub async fn create_workflow(created_by: &str, request: CreateWorkflowRequest) -> AppResult<Workflow> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Workflow name is required."));
    }
    workflow_store::insert(&name, request.description.trim(), request.icon.trim(), created_by).await
}

pub async fn update_workflow(
    id: i32,
    created_by: &str,
    request: UpdateWorkflowRequest,
) -> AppResult<Workflow> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Workflow name is required."));
    }
    workflow_store::update(id, &name, request.description.trim(), request.icon.trim(), created_by)
        .await?
        .ok_or_else(|| AppError::new(format!("Workflow '{id}' not found.")))
}

pub async fn delete_workflow(id: i32, created_by: &str) -> AppResult<()> {
    if !workflow_store::delete_by_id(id, created_by).await? {
        return Err(AppError::new(format!("Workflow '{id}' not found.")));
    }
    Ok(())
}

pub async fn duplicate_workflow(id: i32, created_by: &str) -> AppResult<Workflow> {
    workflow_store::duplicate(id, created_by)
        .await?
        .ok_or_else(|| AppError::new(format!("Workflow '{id}' not found.")))
}

pub async fn save_layout(id: i32, created_by: &str, layout: HashMap<String, NodePos>) -> AppResult<()> {
    workflow_store::update_layout(id, &layout, created_by).await?;
    Ok(())
}

pub async fn list_steps(workflow_id: i32) -> AppResult<Vec<WorkflowStep>> {
    workflow_store::list_steps(workflow_id).await
}

pub async fn create_step(workflow_id: i32, request: StepRequest) -> AppResult<WorkflowStep> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Step name is required."));
    }
    workflow_store::insert_step(
        workflow_id,
        &name,
        &request.step_type,
        request.skill_name.trim(),
        request.prompt_id,
        request.runner_command.trim(),
        request.ai_account_id,
        request.description.trim(),
        request.icon.trim(),
        request.step_order,
        request.is_latest_step,
        request.model_id,
    )
    .await
}

pub async fn update_step(id: i32, request: StepRequest) -> AppResult<WorkflowStep> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Step name is required."));
    }
    workflow_store::update_step(
        id,
        &name,
        &request.step_type,
        request.skill_name.trim(),
        request.prompt_id,
        request.runner_command.trim(),
        request.ai_account_id,
        request.description.trim(),
        request.icon.trim(),
        request.step_order,
        request.is_latest_step,
        request.model_id,
    )
    .await?
    .ok_or_else(|| AppError::new(format!("Step '{id}' not found.")))
}

pub async fn delete_step(id: i32) -> AppResult<()> {
    if !workflow_store::delete_step(id).await? {
        return Err(AppError::new(format!("Step '{id}' not found.")));
    }
    Ok(())
}

pub async fn reorder_steps(workflow_id: i32, step_ids: Vec<i32>) -> AppResult<()> {
    workflow_store::reorder_steps(workflow_id, &step_ids).await
}
