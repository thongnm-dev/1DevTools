//! Tauri command handlers cho module AI Usage.

use crate::app::error::AppErrorPayload;
use crate::models::ai_usage::{
    AddAiAccountRequest, AiAccount, AiUsageSettings, CapturedLogin, DetectedLogin,
    ReportUsageSignalRequest, UpdateAiAccountRequest,
};
use crate::services::ai_acc_service;
use crate::services::ai_usage_service;
use crate::services::claude_service;
use crate::services::claude_terminal;

/// Thêm thủ công một account AI bằng token do người dùng nhập (không qua dò/capture).
#[tauri::command]
pub async fn ai_usage_add_account(request: AddAiAccountRequest) -> Result<AiAccount, AppErrorPayload> {
    ai_acc_service::add_account(request).map_err(crate::app::error::log_err)
}

/// Dò các login Claude đã tồn tại trên máy (chưa thêm gì, chỉ trả danh sách).
#[tauri::command]
pub async fn ai_usage_detect_local() -> Result<Vec<DetectedLogin>, AppErrorPayload> {
    claude_service::detect_local().map_err(crate::app::error::log_err)
}

/// Dò login local rồi tự thêm những login chưa có; trả về danh sách account mới.
#[tauri::command]
pub async fn ai_usage_import_detected() -> Result<Vec<AiAccount>, AppErrorPayload> {
    claude_service::import_detected().map_err(crate::app::error::log_err)
}

/// Xem trước login Claude đang active trên máy (để capture) — không kèm token.
#[tauri::command]
pub async fn ai_usage_capture_preview() -> Result<Option<CapturedLogin>, AppErrorPayload> {
    claude_service::capture_preview().map_err(crate::app::error::log_err)
}

/// Capture login Claude đang active → lưu token vào profile riêng + thêm account.
#[tauri::command]
pub async fn ai_usage_capture_add(name: Option<String>) -> Result<AiAccount, AppErrorPayload> {
    claude_service::capture_add(name).map_err(crate::app::error::log_err)
}

/// Xem trước login Claude tại một `CLAUDE_CONFIG_DIR` (thêm account thứ 2).
#[tauri::command]
pub async fn ai_usage_config_dir_preview(
    config_dir: String,
) -> Result<Option<CapturedLogin>, AppErrorPayload> {
    claude_service::config_dir_preview(config_dir).map_err(crate::app::error::log_err)
}

/// Đăng ký account subscription thứ 2 từ một `CLAUDE_CONFIG_DIR` đã login sẵn.
#[tauri::command]
pub async fn ai_usage_add_config_dir(
    config_dir: String,
    name: Option<String>,
) -> Result<AiAccount, AppErrorPayload> {
    claude_service::add_config_dir(config_dir, name).map_err(crate::app::error::log_err)
}

/// Liệt kê tất cả account AI đã lưu (kèm thông tin usage gần nhất).
#[tauri::command]
pub async fn ai_usage_list_accounts() -> Result<Vec<AiAccount>, AppErrorPayload> {
    ai_acc_service::list_accounts().map_err(crate::app::error::log_err)
}

/// Cập nhật thông tin account (đổi tên, ghi chú…).
#[tauri::command]
pub async fn ai_usage_update_account(
    request: UpdateAiAccountRequest,
) -> Result<AiAccount, AppErrorPayload> {
    ai_acc_service::update_account(request).map_err(crate::app::error::log_err)
}

/// Xoá account AI theo ID (kèm token/profile liên quan).
#[tauri::command]
pub async fn ai_usage_delete_account(id: i64) -> Result<(), AppErrorPayload> {
    ai_acc_service::delete_account(id).map_err(crate::app::error::log_err)
}

/// Đặt account `id` làm account đang active (dùng cho các thao tác Claude sau đó).
#[tauri::command]
pub async fn ai_usage_set_active(id: i64) -> Result<(), AppErrorPayload> {
    ai_acc_service::set_active(id).map_err(crate::app::error::log_err)
}

/// Lấy access token của account `id` (giải mã từ profile) để gọi API.
#[tauri::command]
pub async fn ai_usage_get_token(id: i64) -> Result<String, AppErrorPayload> {
    ai_acc_service::get_token(id).map_err(crate::app::error::log_err)
}

/// Nhận tín hiệu usage do frontend/CLI báo về để cập nhật số liệu sử dụng.
#[tauri::command]
pub async fn ai_usage_report_signal(request: ReportUsageSignalRequest) -> Result<(), AppErrorPayload> {
    ai_acc_service::report_signal(request).map_err(crate::app::error::log_err)
}

/// Probe usage một account theo ID, trả về account sau khi cập nhật.
#[tauri::command]
pub async fn ai_usage_refresh_account(id: i64) -> Result<AiAccount, AppErrorPayload> {
    ai_usage_service::poll_account(id)
        .await
        .map_err(crate::app::error::log_err)
}

/// Ép probe usage ngay lập tức (toàn bộ account), trả về danh sách sau khi cập nhật.
#[tauri::command]
pub async fn ai_usage_refresh(app: tauri::AppHandle) -> Result<Vec<AiAccount>, AppErrorPayload> {
    ai_usage_service::poll_once(&app)
        .await
        .map_err(crate::app::error::log_err)?;
    ai_acc_service::list_accounts().map_err(crate::app::error::log_err)
}

/// Đọc cấu hình module AI Usage (chu kỳ tự probe, ngưỡng cảnh báo…).
#[tauri::command]
pub async fn ai_usage_get_settings() -> Result<AiUsageSettings, AppErrorPayload> {
    ai_acc_service::get_settings().map_err(crate::app::error::log_err)
}

/// Lưu cấu hình module AI Usage.
#[tauri::command]
pub async fn ai_usage_save_settings(settings: AiUsageSettings) -> Result<(), AppErrorPayload> {
    ai_acc_service::save_settings(settings).map_err(crate::app::error::log_err)
}

/// Mở terminal với `CLAUDE_CONFIG_DIR` trong working directory chỉ định.
#[tauri::command]
pub async fn ai_usage_open_terminal(
    config_dir: String,
    work_dir: String,
    prompt: Option<String>,
    model: Option<String>,
) -> Result<(), AppErrorPayload> {
    claude_terminal::open_terminal(&config_dir, &work_dir, prompt.as_deref(), model.as_deref())
        .map_err(crate::app::error::log_err)
}

/// Mở terminal mới chạy `claude /login` với `CLAUDE_CONFIG_DIR` tuỳ chỉnh.
#[tauri::command]
pub async fn ai_usage_open_login(config_dir: String, work_dir: String) -> Result<(), AppErrorPayload> {
    claude_terminal::open_login_terminal(&config_dir, &work_dir).map_err(crate::app::error::log_err)
}
