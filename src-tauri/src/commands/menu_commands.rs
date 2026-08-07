//! Tauri command handlers cho module quản lý menu.

use crate::app::error::AppErrorPayload;
use crate::models::menu_entity::MenuEntity;
use crate::services::menu_service;

#[tauri::command]
pub async fn list_menu_configs() -> Result<Vec<MenuEntity>, AppErrorPayload> {
    menu_service::list_menu_configs()
        .await
        .map_err(crate::app::error::log_err)
}
