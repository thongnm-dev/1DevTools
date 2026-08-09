//! Tauri command handlers cho module phân quyền menu.

use crate::app::error::AppErrorPayload;
use crate::models::menu_permission::{
    EffectiveMenuPermission, SaveRoleMenuPermissionsRequest, SaveUserMenuPermissionsRequest,
    UserMenuPermission,
};
use crate::services::menu_permission_service;

#[tauri::command]
pub async fn list_role_menu_permissions(role_id: i32) -> Result<Vec<String>, AppErrorPayload> {
    menu_permission_service::list_role_menu_permissions(role_id)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn save_role_menu_permissions(
    request: SaveRoleMenuPermissionsRequest,
) -> Result<Vec<String>, AppErrorPayload> {
    menu_permission_service::save_role_menu_permissions(request)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn list_user_menu_permissions(
    user_id: i32,
) -> Result<Vec<UserMenuPermission>, AppErrorPayload> {
    menu_permission_service::list_user_menu_permissions(user_id)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn save_user_menu_permissions(
    request: SaveUserMenuPermissionsRequest,
) -> Result<Vec<UserMenuPermission>, AppErrorPayload> {
    menu_permission_service::save_user_menu_permissions(request)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn list_effective_menu_permissions(
    user_id: i32,
) -> Result<Vec<EffectiveMenuPermission>, AppErrorPayload> {
    menu_permission_service::list_effective_menu_permissions(user_id)
        .await
        .map_err(crate::app::error::log_err)
}
