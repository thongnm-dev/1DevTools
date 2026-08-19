//! Data access cho bảng `agent_providers` (PostgreSQL) — mọi truy vấn qua
//! stored procedure `sp_agent_provider_*`.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::agent_provider::{AgentProvider, AgentProviderType};
use crate::utils::pgsql_connect;

fn map_row(row: &tokio_postgres::Row) -> AgentProvider {
    let provider_type: String = row.get("provider_type");
    AgentProvider {
        id: row.get("id"),
        name: row.get("name"),
        code: row.get("code"),
        provider_type: AgentProviderType::from_code(&provider_type),
        description: row.get("description"),
        icon: row.get("icon"),
        command: row.get("command"),
        website: row.get("website"),
        models: row.get("models"),
        enabled: row.get("enabled"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn list_all() -> AppResult<Vec<AgentProvider>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query("SELECT * FROM sp_agent_provider_select_list()", &[])
        .await
        .map_err(|e| AppError::new(format!("Failed to list agent providers: {e}")))?;

    Ok(rows.iter().map(map_row).collect())
}

pub async fn find_by_id(id: i32) -> AppResult<Option<AgentProvider>> {
    Ok(list_all().await?.into_iter().find(|p| p.id == id))
}

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    name: &str,
    code: &str,
    provider_type: &str,
    description: &str,
    icon: &str,
    command: &str,
    website: &str,
    models: &[String],
    enabled: bool,
) -> AppResult<AgentProvider> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one(
            "SELECT * FROM sp_agent_provider_insert($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &name,
                &code,
                &provider_type,
                &description,
                &icon,
                &command,
                &website,
                &models,
                &enabled,
            ],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert agent provider: {e}")))?;

    Ok(map_row(&row))
}

#[allow(clippy::too_many_arguments)]
pub async fn update(
    id: i32,
    name: &str,
    code: &str,
    provider_type: &str,
    description: &str,
    icon: &str,
    command: &str,
    website: &str,
    models: &[String],
    enabled: bool,
) -> AppResult<AgentProvider> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_opt(
            "SELECT * FROM sp_agent_provider_update($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            &[
                &id,
                &name,
                &code,
                &provider_type,
                &description,
                &icon,
                &command,
                &website,
                &models,
                &enabled,
            ],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update agent provider: {e}")))?
        .ok_or_else(|| AppError::new(format!("Agent provider '{id}' not found.")))?;

    Ok(map_row(&row))
}

pub async fn set_enabled(id: i32, enabled: bool) -> AppResult<AgentProvider> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_opt(
            "SELECT * FROM sp_agent_provider_set_enabled($1, $2)",
            &[&id, &enabled],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to toggle agent provider: {e}")))?
        .ok_or_else(|| AppError::new(format!("Agent provider '{id}' not found.")))?;

    Ok(map_row(&row))
}

pub async fn delete_by_id(id: i32) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one("SELECT sp_agent_provider_delete($1)", &[&id])
        .await
        .map_err(|e| AppError::new(format!("Failed to delete agent provider: {e}")))?;

    let deleted: i32 = row.get(0);
    Ok(deleted > 0)
}

pub async fn code_exists(code: &str, exclude_id: Option<i32>) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one(
            "SELECT sp_agent_provider_code_exists($1, $2)",
            &[&code, &exclude_id],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to check provider code: {e}")))?;

    Ok(row.get(0))
}
