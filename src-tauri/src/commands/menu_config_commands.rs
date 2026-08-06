//! Tauri command handlers cho module quản lý menu.

use crate::models::menu_config::MenuConfig;
use crate::services::menu_config_service;

#[tauri::command]
pub async fn list_menu_configs() -> Result<Vec<MenuConfig>, String> {
    menu_config_service::list_menu_configs()
        .await
        .map_err(crate::app::error::log_err)
}
