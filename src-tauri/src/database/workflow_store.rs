//! Data access cho bảng `workflows` / `workflow_steps` (PostgreSQL). Danh mục
//! model cho step picker do domain `agent_provider_model` cung cấp (model enabled).

use std::collections::HashMap;

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::workflow::{NodePos, Workflow, WorkflowStep, WorkflowStepType};
use crate::utils::pgsql_connect;

fn map_workflow(row: &tokio_postgres::Row) -> Workflow {
    let layout_json: serde_json::Value = row.get("layout");
    Workflow {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        icon: row.get("icon"),
        layout: serde_json::from_value::<HashMap<String, NodePos>>(layout_json).unwrap_or_default(),
        created_by: row.get("created_by"),
        step_count: row.get("step_count"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn map_step(row: &tokio_postgres::Row) -> WorkflowStep {
    let step_type: String = row.get("step_type");
    WorkflowStep {
        id: row.get("id"),
        workflow_id: row.get("workflow_id"),
        name: row.get("name"),
        step_type: WorkflowStepType::from_str(&step_type),
        skill_name: row.get("skill_name"),
        prompt_id: row.get("prompt_id"),
        runner_command: row.get("runner_command"),
        ai_account_id: row.get("ai_account_id"),
        description: row.get("description"),
        icon: row.get("icon"),
        step_order: row.get("step_order"),
        is_latest_step: row.get("is_latest_step"),
        model_id: row.get("model_id"),
        created_at: row.get("created_at"),
    }
}


pub async fn list_all(created_by: &str) -> AppResult<Vec<Workflow>> {
    let client = pgsql_connect::connect().await?;
    let rows = client
        .query("SELECT * FROM sp_workflow_select_list($1)", &[&created_by])
        .await
        .map_err(|e| AppError::new(format!("Failed to list workflows: {e}")))?;
    Ok(rows.iter().map(map_workflow).collect())
}

pub async fn insert(name: &str, description: &str, icon: &str, created_by: &str) -> AppResult<Workflow> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one(
            "SELECT * FROM sp_workflow_insert($1, $2, $3, $4)",
            &[&name, &description, &icon, &created_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert workflow: {e}")))?;
    Ok(map_workflow(&row))
}

pub async fn update(
    id: i32,
    name: &str,
    description: &str,
    icon: &str,
    created_by: &str,
) -> AppResult<Option<Workflow>> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_opt(
            "SELECT * FROM sp_workflow_update($1, $2, $3, $4, $5)",
            &[&id, &name, &description, &icon, &created_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update workflow: {e}")))?;
    Ok(row.as_ref().map(map_workflow))
}

pub async fn delete_by_id(id: i32, created_by: &str) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one("SELECT sp_workflow_delete($1, $2)", &[&id, &created_by])
        .await
        .map_err(|e| AppError::new(format!("Failed to delete workflow: {e}")))?;
    let deleted: i32 = row.get(0);
    Ok(deleted > 0)
}

pub async fn update_layout(id: i32, layout: &HashMap<String, NodePos>, created_by: &str) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;
    let layout_json = serde_json::to_value(layout)
        .map_err(|e| AppError::new(format!("Invalid layout: {e}")))?;
    let row = client
        .query_one(
            "SELECT sp_workflow_update_layout($1, $2, $3)",
            &[&id, &layout_json, &created_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to save workflow layout: {e}")))?;
    let updated: i32 = row.get(0);
    Ok(updated > 0)
}

pub async fn duplicate(id: i32, created_by: &str) -> AppResult<Option<Workflow>> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_opt("SELECT * FROM sp_workflow_duplicate($1, $2)", &[&id, &created_by])
        .await
        .map_err(|e| AppError::new(format!("Failed to duplicate workflow: {e}")))?;
    Ok(row.as_ref().map(map_workflow))
}

pub async fn list_steps(workflow_id: i32) -> AppResult<Vec<WorkflowStep>> {
    let client = pgsql_connect::connect().await?;
    let rows = client
        .query("SELECT * FROM sp_workflow_step_select($1)", &[&workflow_id])
        .await
        .map_err(|e| AppError::new(format!("Failed to list workflow steps: {e}")))?;
    Ok(rows.iter().map(map_step).collect())
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_step(
    workflow_id: i32,
    name: &str,
    step_type: &WorkflowStepType,
    skill_name: &str,
    prompt_id: Option<i32>,
    runner_command: &str,
    ai_account_id: Option<i32>,
    description: &str,
    icon: &str,
    step_order: i32,
    is_latest_step: bool,
    model_id: Option<i32>,
) -> AppResult<WorkflowStep> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one(
            "SELECT * FROM sp_workflow_step_insert($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[
                &workflow_id,
                &name,
                &step_type.as_str(),
                &skill_name,
                &prompt_id,
                &runner_command,
                &ai_account_id,
                &description,
                &icon,
                &step_order,
                &is_latest_step,
                &model_id,
            ],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert workflow step: {e}")))?;
    Ok(map_step(&row))
}

#[allow(clippy::too_many_arguments)]
pub async fn update_step(
    id: i32,
    name: &str,
    step_type: &WorkflowStepType,
    skill_name: &str,
    prompt_id: Option<i32>,
    runner_command: &str,
    ai_account_id: Option<i32>,
    description: &str,
    icon: &str,
    step_order: i32,
    is_latest_step: bool,
    model_id: Option<i32>,
) -> AppResult<Option<WorkflowStep>> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_opt(
            "SELECT * FROM sp_workflow_step_update($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[
                &id,
                &name,
                &step_type.as_str(),
                &skill_name,
                &prompt_id,
                &runner_command,
                &ai_account_id,
                &description,
                &icon,
                &step_order,
                &is_latest_step,
                &model_id,
            ],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update workflow step: {e}")))?;
    Ok(row.as_ref().map(map_step))
}

pub async fn delete_step(id: i32) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one("SELECT sp_workflow_step_delete($1)", &[&id])
        .await
        .map_err(|e| AppError::new(format!("Failed to delete workflow step: {e}")))?;
    let deleted: i32 = row.get(0);
    Ok(deleted > 0)
}

pub async fn reorder_steps(workflow_id: i32, step_ids: &[i32]) -> AppResult<()> {
    let client = pgsql_connect::connect().await?;
    client
        .execute(
            "SELECT sp_workflow_step_reorder($1, $2)",
            &[&workflow_id, &step_ids],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to reorder workflow steps: {e}")))?;
    Ok(())
}
