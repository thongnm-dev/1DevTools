//! Tauri command handlers cho module phân quyền menu.

use crate::app::error::AppErrorPayload;
use crate::models::menu_permission::EffectiveMenuPermission;
use crate::services::menu_permission_service;

#[tauri::command]
pub async fn list_effective_menu_permissions(
    user_id: i32,
) -> Result<Vec<EffectiveMenuPermission>, AppErrorPayload> {
    menu_permission_service::list_effective_menu_permissions(user_id)
        .await
        .map_err(crate::app::error::log_err)
}
