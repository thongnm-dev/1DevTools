//! Tauri command handlers cho module phân quyền menu.

use crate::app::error::AppErrorPayload;
use crate::models::menu_permission::{
    EffectiveMenuPermission, SaveRoleMenuPermissionsRequest, SaveUserMenuPermissionsRequest,
    UserMenuPermission,
};
use crate::services::menu_permission_service;

/// Lấy danh sách key menu mà `role_id` được phép truy cập (phân quyền theo role).
#[tauri::command]
pub async fn list_role_menu_permissions(role_id: i32) -> Result<Vec<String>, AppErrorPayload> {
    menu_permission_service::list_role_menu_permissions(role_id)
        .await
        .map_err(crate::app::error::log_err)
}

/// Ghi đè toàn bộ quyền menu của một role, trả về danh sách quyền sau khi lưu.
#[tauri::command]
pub async fn save_role_menu_permissions(
    request: SaveRoleMenuPermissionsRequest,
) -> Result<Vec<String>, AppErrorPayload> {
    menu_permission_service::save_role_menu_permissions(request)
        .await
        .map_err(crate::app::error::log_err)
}

/// Lấy các quyền menu ghi đè riêng ở cấp user (allow/deny cho từng menu).
#[tauri::command]
pub async fn list_user_menu_permissions(
    user_id: i32,
) -> Result<Vec<UserMenuPermission>, AppErrorPayload> {
    menu_permission_service::list_user_menu_permissions(user_id)
        .await
        .map_err(crate::app::error::log_err)
}

/// Ghi đè các quyền menu riêng ở cấp user, trả về danh sách sau khi lưu.
#[tauri::command]
pub async fn save_user_menu_permissions(
    request: SaveUserMenuPermissionsRequest,
) -> Result<Vec<UserMenuPermission>, AppErrorPayload> {
    menu_permission_service::save_user_menu_permissions(request)
        .await
        .map_err(crate::app::error::log_err)
}

/// Quyền menu hiệu lực cuối cùng của user = quyền theo role hợp nhất với các
/// ghi đè cấp user. Đây là danh sách UI dùng để ẩn/hiện menu thực tế.
#[tauri::command]
pub async fn list_effective_menu_permissions(
    user_id: i32,
) -> Result<Vec<EffectiveMenuPermission>, AppErrorPayload> {
    menu_permission_service::list_effective_menu_permissions(user_id)
        .await
        .map_err(crate::app::error::log_err)
}
