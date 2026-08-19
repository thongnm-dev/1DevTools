//! Tauri IPC commands cho Master Data (CRUD danh mục dùng chung).
//! Thin layer — uỷ quyền toàn bộ nghiệp vụ cho `master_data_service`.

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::master_data::{MasterData, MasterDataRequest};
use crate::services::master_data_service;

/// Danh sách toàn bộ danh mục.
#[tauri::command]
pub async fn master_data_list() -> Result<Vec<MasterData>, AppErrorPayload> {
    master_data_service::list_items().await.map_err(log_err)
}

/// Thêm danh mục mới.
#[tauri::command]
pub async fn master_data_create(
    request: MasterDataRequest,
) -> Result<MasterData, AppErrorPayload> {
    master_data_service::create_item(request).await.map_err(log_err)
}

/// Cập nhật danh mục `id`.
#[tauri::command]
pub async fn master_data_update(
    id: i32,
    request: MasterDataRequest,
) -> Result<MasterData, AppErrorPayload> {
    master_data_service::update_item(id, request).await.map_err(log_err)
}

/// Xoá danh mục `id`.
#[tauri::command]
pub async fn master_data_delete(id: i32) -> Result<(), AppErrorPayload> {
    master_data_service::delete_item(id).await.map_err(log_err)
}
