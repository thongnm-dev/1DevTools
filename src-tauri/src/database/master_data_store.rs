//! Data access cho bảng `master_data` (PostgreSQL) — mọi truy vấn qua stored
//! procedure `sp_master_data_*`.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::master_data::MasterData;
use crate::utils::pgsql_connect;

fn map_row(row: &tokio_postgres::Row) -> MasterData {
    MasterData {
        id: row.get("id"),
        name: row.get("name"),
        icon: row.get("icon"),
        keygroup: row.get("keygroup"),
        display_order: row.get("display_order"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn list_all() -> AppResult<Vec<MasterData>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query("SELECT * FROM sp_master_data_select_list()", &[])
        .await
        .map_err(|e| AppError::new(format!("Failed to list master data: {e}")))?;

    Ok(rows.iter().map(map_row).collect())
}

pub async fn find_by_id(id: i32) -> AppResult<Option<MasterData>> {
    Ok(list_all().await?.into_iter().find(|m| m.id == id))
}

pub async fn insert(
    name: &str,
    icon: &str,
    keygroup: &str,
    display_order: i32,
    description: &str,
) -> AppResult<MasterData> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one(
            "SELECT * FROM sp_master_data_insert($1, $2, $3, $4, $5)",
            &[&name, &icon, &keygroup, &display_order, &description],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to insert master data: {e}")))?;

    Ok(map_row(&row))
}

pub async fn update(
    id: i32,
    name: &str,
    icon: &str,
    keygroup: &str,
    display_order: i32,
    description: &str,
) -> AppResult<MasterData> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_opt(
            "SELECT * FROM sp_master_data_update($1, $2, $3, $4, $5, $6)",
            &[&id, &name, &icon, &keygroup, &display_order, &description],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to update master data: {e}")))?
        .ok_or_else(|| AppError::new(format!("Master data '{id}' not found.")))?;

    Ok(map_row(&row))
}

pub async fn delete_by_id(id: i32) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one("SELECT sp_master_data_delete($1)", &[&id])
        .await
        .map_err(|e| AppError::new(format!("Failed to delete master data: {e}")))?;

    let deleted: i32 = row.get(0);
    Ok(deleted > 0)
}

pub async fn name_exists(name: &str, exclude_id: Option<i32>) -> AppResult<bool> {
    let client = pgsql_connect::connect().await?;

    let row = client
        .query_one(
            "SELECT sp_master_data_name_exists($1, $2)",
            &[&name, &exclude_id],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to check master data name: {e}")))?;

    Ok(row.get(0))
}
