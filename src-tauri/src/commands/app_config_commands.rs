//! Tauri command handlers cho cấu hình ứng dụng (config.ini).

use crate::app::error::AppErrorPayload;
use crate::models::app_config::{AppConfigData, SaveAppConfigRequest};
use crate::services::app_config_service;

#[tauri::command]
pub fn get_app_config() -> Result<AppConfigData, AppErrorPayload> {
    app_config_service::get_app_config().map_err(crate::app::error::log_err)
}

#[tauri::command]
pub fn save_app_config(request: SaveAppConfigRequest) -> Result<AppConfigData, AppErrorPayload> {
    app_config_service::save_app_config(request).map_err(crate::app::error::log_err)
}
