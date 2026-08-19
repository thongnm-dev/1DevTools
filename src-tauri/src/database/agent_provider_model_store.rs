//! Data access cho bảng `agent_provider_models` (PostgreSQL) — mọi truy vấn qua
//! stored procedure `sp_agent_provider_model_*`.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::agent_provider_model::AgentProviderModel;
use crate::utils::pgsql_connect;

fn map_row(row: &tokio_postgres::Row) -> AgentProviderModel {
    AgentProviderModel {
        id: row.get("id"),
        provider_id: row.get("provider_id"),
        provider_name: row.get("provider_name"),
        name: row.get("name"),
        code: row.get("code"),
        version: row.get("version"),
        description: row.get("description"),
        enabled: row.get("enabled"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn list_all() -> AppResult<Vec<AgentProviderModel>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query("SELECT * FROM sp_agent_provider_model_select_list()", &[])
        .await
        .map_err(|e| AppError::new(format!("Failed to list provider models: {e}")))?;

    Ok(rows.iter().map(map_row).collect())
}

pub async fn find_by_id(id: i32) -> AppResult<Option<AgentProviderModel>> {
    Ok(list_all().await?.into_iter().find(|m| m.id == id))
}

/// Chỉ các model đang bật (enabled) — dùng cho workflow step "Model" picker.
pub async fn list_enabled() -> AppResult<Vec<AgentProviderModel>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query("SELECT * FROM sp_agent_provider_model_select_enabled()", &[])
        .await
        .map_err(|e| AppError::new(format!("Failed to list enabled provider models: {e}")))?;

    Ok(rows.iter().map(map_row).collect())
}

pub async fn insert(
    provider_id: i32,
    name: &str,
    code: &str,
    version: &str,
    description: &str,
    enabled: bool,
) -> AppResult<AgentProviderModel> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one(
            "SELECT * FROM sp_agent_provider_model_insert($1, $2, $3, $4, $5, $6)",
            &[
                &provider_id,
                &name,
                &code,
                &version,
                &description,
                &enabled,
            ],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert provider model: {e}")))?;

    Ok(map_row(&row))
}

pub async fn update(
    id: i32,
    provider_id: i32,
    name: &str,
    code: &str,
    version: &str,
    description: &str,
    enabled: bool,
) -> AppResult<AgentProviderModel> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_opt(
            "SELECT * FROM sp_agent_provider_model_update($1, $2, $3, $4, $5, $6, $7)",
            &[
                &id,
                &provider_id,
                &name,
                &code,
                &version,
                &description,
                &enabled,
            ],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update provider model: {e}")))?
        .ok_or_else(|| AppError::new(format!("Provider model '{id}' not found.")))?;

    Ok(map_row(&row))
}

pub async fn set_enabled(id: i32, enabled: bool) -> AppResult<AgentProviderModel> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_opt(
            "SELECT * FROM sp_agent_provider_model_set_enabled($1, $2)",
            &[&id, &enabled],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to toggle provider model: {e}")))?
        .ok_or_else(|| AppError::new(format!("Provider model '{id}' not found.")))?;

    Ok(map_row(&row))
}

pub async fn delete_by_id(id: i32) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one("SELECT sp_agent_provider_model_delete($1)", &[&id])
        .await
        .map_err(|e| AppError::new(format!("Failed to delete provider model: {e}")))?;

    let deleted: i32 = row.get(0);
    Ok(deleted > 0)
}

pub async fn code_exists(
    provider_id: i32,
    code: &str,
    exclude_id: Option<i32>,
) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one(
            "SELECT sp_agent_provider_model_code_exists($1, $2, $3)",
            &[&provider_id, &code, &exclude_id],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to check model code: {e}")))?;

    Ok(row.get(0))
}
