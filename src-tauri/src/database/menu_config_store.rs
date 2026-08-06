//! Data access cho bảng `menu_configs` (PostgreSQL).

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::menu_config::MenuConfig;
use crate::utils::pgsql_connect;

pub async fn list_all() -> AppResult<Vec<MenuConfig>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query("SELECT * FROM sp_menu_config_select_list()", &[])
        .await
        .map_err(|e| AppError::new(format!("Failed to list menu configs: {e}")))?;

    let items = rows
        .iter()
        .map(|row| MenuConfig {
            key: row.get("key"),
            title: row.get("title"),
            path: row.get("path"),
            icon: row.get("icon"),
            group: row.get("menu_group"),
            visible: row.get("is_visible"),
            order: row.get("display_order"),
        })
        .collect();

    Ok(items)
}
