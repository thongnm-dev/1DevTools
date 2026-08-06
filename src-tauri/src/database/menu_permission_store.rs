//! Data access cho bảng `role_menu_permissions` và `user_menu_permissions` (PostgreSQL).

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::menu_permission::EffectiveMenuPermission;
use crate::utils::pgsql_connect;

pub async fn list_effective(user_id: i32) -> AppResult<Vec<EffectiveMenuPermission>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query(
            "SELECT * FROM sp_menu_permission_effective_select($1)",
            &[&user_id],
        )
        .await
        .map_err(|e| AppError::new(format!("Failed to resolve menu permissions: {e}")))?;

    Ok(rows
        .iter()
        .map(|row| EffectiveMenuPermission {
            menu_key: row.get("menu_key"),
            is_allowed: row.get("is_allowed"),
            role_allowed: row.get("role_allowed"),
            source: row.get("source"),
        })
        .collect())
}
