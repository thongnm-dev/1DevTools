//! Tauri command handlers cho xác thực người dùng: đăng nhập và đặt lại mật khẩu.

use crate::app::error::AppErrorPayload;
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::services::auth_service;

/// Đăng nhập bằng username/password, trả về thông tin phiên (user + quyền menu).
#[tauri::command]
pub async fn login(request: LoginRequest) -> Result<LoginResponse, AppErrorPayload> {
    auth_service::login(request)
        .await
        .map_err(crate::app::error::log_err)
}

/// Bước 1 quy trình quên mật khẩu: gửi mã xác nhận tới email của `username`.
/// Trả về thông báo/địa chỉ email đã gửi để UI hiển thị.
#[tauri::command]
pub async fn request_password_reset(username: String) -> Result<String, AppErrorPayload> {
    auth_service::request_password_reset(&username)
        .await
        .map_err(crate::app::error::log_err)
}

/// Bước 2 quy trình quên mật khẩu: kiểm tra `code` xác nhận và cấp mật khẩu mới.
#[tauri::command]
pub async fn verify_password_reset(username: String, code: String) -> Result<String, AppErrorPayload> {
    auth_service::verify_password_reset(&username, &code)
        .await
        .map_err(crate::app::error::log_err)
}
