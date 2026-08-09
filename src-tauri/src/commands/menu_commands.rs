//! Tauri command handlers cho module quản lý menu.

use crate::app::error::AppErrorPayload;
use crate::models::menu_entity::{MenuEntity, SaveAllMenuConfigsRequest, SaveMenuConfigRequest};
use crate::services::menu_service;

/// Liệt kê toàn bộ cấu hình menu đang lưu để dựng sidebar/điều hướng.
#[tauri::command]
pub async fn list_menu_configs() -> Result<Vec<MenuEntity>, AppErrorPayload> {
    menu_service::list_menu_configs()
        .await
        .map_err(crate::app::error::log_err)
}

/// Thêm mới hoặc cập nhật (upsert) một mục menu theo `key`.
#[tauri::command]
pub async fn save_menu_config(request: SaveMenuConfigRequest) -> Result<(), AppErrorPayload> {
    menu_service::save_menu_config(request)
        .await
        .map_err(crate::app::error::log_err)
}

/// Lưu toàn bộ danh sách menu trong một lần (dùng khi kéo-thả sắp xếp lại),
/// rồi trả về danh sách sau khi lưu.
#[tauri::command]
pub async fn save_all_menu_configs(
    request: SaveAllMenuConfigsRequest,
) -> Result<Vec<MenuEntity>, AppErrorPayload> {
    menu_service::save_all_menu_configs(request)
        .await
        .map_err(crate::app::error::log_err)
}
