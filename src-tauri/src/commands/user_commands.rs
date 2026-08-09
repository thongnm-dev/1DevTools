//! Tauri command handlers cho module quản lý người dùng (governance).

use crate::app::error::AppErrorPayload;
use crate::models::user::{
    ChangePasswordRequest, CreateUserRequest, UpdateUserRequest, UserDetail, UserSummary,
};
use crate::services::user_service;

#[tauri::command]
pub async fn create_user(request: CreateUserRequest) -> Result<UserDetail, AppErrorPayload> {
    user_service::create_user(request)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn update_user(
    user_id: i32,
    request: UpdateUserRequest,
) -> Result<UserDetail, AppErrorPayload> {
    user_service::update_user(user_id, request)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn get_user_detail(user_id: i32) -> Result<UserDetail, AppErrorPayload> {
    user_service::get_user_detail(user_id)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn list_users() -> Result<Vec<UserSummary>, AppErrorPayload> {
    user_service::list_users()
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn delete_user(user_id: i32) -> Result<(), AppErrorPayload> {
    user_service::delete_user(user_id)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn change_user_password(
    user_id: i32,
    request: ChangePasswordRequest,
) -> Result<(), AppErrorPayload> {
    user_service::change_password(user_id, request)
        .await
        .map_err(crate::app::error::log_err)
}

#[tauri::command]
pub async fn list_roles() -> Result<Vec<String>, AppErrorPayload> {
    user_service::list_roles()
        .await
        .map_err(crate::app::error::log_err)
}
