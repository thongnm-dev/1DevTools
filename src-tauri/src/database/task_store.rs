//! Data access cho bảng `tasks` / `task_wf_proc` / `task_wf_proc_step` (PostgreSQL).

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::task::{Task, TaskWfProc, TaskWfProcStep};
use crate::utils::pgsql_connect;

fn map_task(row: &tokio_postgres::Row) -> Task {
    Task {
        id: row.get("id"),
        task_cd: row.get("task_cd"),
        task_name: row.get("task_name"),
        category_id: row.get("category_id"),
        is_complete: row.get("is_complete"),
        completed_at: row.get("completed_at"),
        created_at: row.get("created_at"),
        created_by: row.get("created_by"),
        updated_at: row.get("updated_at"),
        updated_by: row.get("updated_by"),
        // Chỉ `sp_task_select_list` trả về 3 cột này — insert/update không có nên
        // dùng `try_get` để tái sử dụng chung 1 hàm map cho cả 3 SP.
        current_wf_name: row.try_get("current_wf_name").unwrap_or_default(),
        current_step_name: row.try_get("current_step_name").unwrap_or_default(),
        current_step_status: row.try_get("current_step_status").unwrap_or_default(),
    }
}

fn map_wf_proc(row: &tokio_postgres::Row) -> TaskWfProc {
    TaskWfProc {
        id: row.get("id"),
        task_id: row.get("task_id"),
        wf_id: row.get("wf_id"),
        latest_step_id: row.get("latest_step_id"),
        created_at: row.get("created_at"),
        created_by: row.get("created_by"),
        updated_at: row.get("updated_at"),
        updated_by: row.get("updated_by"),
    }
}

fn map_wf_proc_step(row: &tokio_postgres::Row) -> TaskWfProcStep {
    TaskWfProcStep {
        id: row.get("id"),
        wf_proc_id: row.get("wf_proc_id"),
        wf_step_id: row.get("wf_step_id"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        created_by: row.get("created_by"),
        updated_at: row.get("updated_at"),
        updated_by: row.get("updated_by"),
    }
}

pub async fn insert(task_cd: &str, task_name: &str, category_id: &str, created_by: &str) -> AppResult<Task> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one(
            "SELECT * FROM sp_task_insert($1, $2, $3, $4)",
            &[&task_cd, &task_name, &category_id, &created_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert task: {e}")))?;
    Ok(map_task(&row))
}

pub async fn select_list(keyword: Option<&str>, is_complete: Option<bool>) -> AppResult<Vec<Task>> {
    let client = pgsql_connect::connect().await?;
    let rows = client
        .query(
            "SELECT * FROM sp_task_select_list($1, $2)",
            &[&keyword, &is_complete],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to list tasks: {e}")))?;
    Ok(rows.iter().map(map_task).collect())
}

pub async fn update(
    id: i32,
    task_cd: &str,
    task_name: &str,
    category_id: &str,
    is_complete: bool,
    updated_by: &str,
) -> AppResult<Option<Task>> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_opt(
            "SELECT * FROM sp_task_update($1, $2, $3, $4, $5, $6)",
            &[&id, &task_cd, &task_name, &category_id, &is_complete, &updated_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update task: {e}")))?;
    Ok(row.as_ref().map(map_task))
}

pub async fn wf_proc_insert(task_id: i32, wf_id: i32, created_by: &str) -> AppResult<TaskWfProc> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one(
            "SELECT * FROM sp_task_wf_proc_insert($1, $2, $3)",
            &[&task_id, &wf_id, &created_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to start task workflow process: {e}")))?;
    Ok(map_wf_proc(&row))
}

pub async fn wf_proc_select_by_task(task_id: i32) -> AppResult<Vec<TaskWfProc>> {
    let client = pgsql_connect::connect().await?;
    let rows = client
        .query("SELECT * FROM sp_task_wf_proc_select_by_task($1)", &[&task_id])
        .await
        .map_err(|e| AppError::new(format!("Failed to list task workflow processes: {e}")))?;
    Ok(rows.iter().map(map_wf_proc).collect())
}

pub async fn wf_proc_update(id: i32, latest_step_id: i32, updated_by: &str) -> AppResult<Option<TaskWfProc>> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_opt(
            "SELECT * FROM sp_task_wf_proc_update($1, $2, $3)",
            &[&id, &latest_step_id, &updated_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update task workflow process: {e}")))?;
    Ok(row.as_ref().map(map_wf_proc))
}

pub async fn wf_proc_delete(id: i32) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one("SELECT sp_task_wf_proc_delete($1)", &[&id])
        .await
        .map_err(|e| AppError::new(format!("Failed to delete task workflow process: {e}")))?;
    let deleted: i32 = row.get(0);
    Ok(deleted > 0)
}

pub async fn wf_proc_step_insert(
    wf_proc_id: i32,
    wf_step_id: i32,
    status: &str,
    created_by: &str,
) -> AppResult<TaskWfProcStep> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_one(
            "SELECT * FROM sp_task_wf_proc_step_insert($1, $2, $3, $4)",
            &[&wf_proc_id, &wf_step_id, &status, &created_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert task workflow step status: {e}")))?;
    Ok(map_wf_proc_step(&row))
}

pub async fn wf_proc_step_select_by_proc(wf_proc_id: i32) -> AppResult<Vec<TaskWfProcStep>> {
    let client = pgsql_connect::connect().await?;
    let rows = client
        .query(
            "SELECT * FROM sp_task_wf_proc_step_select_by_proc($1)",
            &[&wf_proc_id],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to list task workflow step statuses: {e}")))?;
    Ok(rows.iter().map(map_wf_proc_step).collect())
}

pub async fn wf_proc_step_update(id: i32, status: &str, updated_by: &str) -> AppResult<Option<TaskWfProcStep>> {
    let client = pgsql_connect::connect().await?;
    let row = client
        .query_opt(
            "SELECT * FROM sp_task_wf_proc_step_update($1, $2, $3)",
            &[&id, &status, &updated_by],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update task workflow step status: {e}")))?;
    Ok(row.as_ref().map(map_wf_proc_step))
}
