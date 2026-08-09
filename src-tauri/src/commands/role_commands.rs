//! Tauri command handlers cho module quản lý role (governance).

use crate::app::error::AppErrorPayload;
use crate::models::role::{CreateRoleRequest, RoleSummary, UpdateRoleRequest};
use crate::services::role_service;

/// Liệt kê tất cả role kèm thông tin tóm tắt (số user đang gán…).
#[tauri::command]
pub async fn list_role_details() -> Result<Vec<RoleSummary>, AppErrorPayload> {
    role_service::list_roles()
        .await
        .map_err(crate::app::error::log_err)
}

/// Tạo role mới. Service kiểm tra tên không rỗng và không trùng.
#[tauri::command]
pub async fn create_role(request: CreateRoleRequest) -> Result<RoleSummary, AppErrorPayload> {
    role_service::create_role(request)
        .await
        .map_err(crate::app::error::log_err)
}

/// Cập nhật tên/mô tả của role `role_id`.
#[tauri::command]
pub async fn update_role(
    role_id: i32,
    request: UpdateRoleRequest,
) -> Result<RoleSummary, AppErrorPayload> {
    role_service::update_role(role_id, request)
        .await
        .map_err(crate::app::error::log_err)
}

/// Xoá role `role_id`. Service từ chối nếu role còn được gán cho user.
#[tauri::command]
pub async fn delete_role(role_id: i32) -> Result<(), AppErrorPayload> {
    role_service::delete_role(role_id)
        .await
        .map_err(crate::app::error::log_err)
}
