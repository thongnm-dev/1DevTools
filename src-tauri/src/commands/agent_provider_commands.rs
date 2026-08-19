//! Tauri IPC commands cho registry AI Agent Provider (CRUD + bật/tắt).
//! Thin layer — uỷ quyền toàn bộ nghiệp vụ cho `agent_provider_service`.

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::agent_provider::{AgentProvider, AgentProviderRequest};
use crate::services::agent_provider_service;

/// Danh sách toàn bộ provider, mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub async fn agent_provider_list() -> Result<Vec<AgentProvider>, AppErrorPayload> {
    agent_provider_service::list_providers()
        .await
        .map_err(log_err)
}

/// Đăng ký provider mới.
#[tauri::command]
pub async fn agent_provider_create(
    request: AgentProviderRequest,
) -> Result<AgentProvider, AppErrorPayload> {
    agent_provider_service::create_provider(request)
        .await
        .map_err(log_err)
}

/// Cập nhật provider `id`.
#[tauri::command]
pub async fn agent_provider_update(
    id: i32,
    request: AgentProviderRequest,
) -> Result<AgentProvider, AppErrorPayload> {
    agent_provider_service::update_provider(id, request)
        .await
        .map_err(log_err)
}

/// Bật/tắt cho phép sử dụng provider `id` trong hệ thống.
#[tauri::command]
pub async fn agent_provider_set_enabled(
    id: i32,
    enabled: bool,
) -> Result<AgentProvider, AppErrorPayload> {
    agent_provider_service::set_enabled(id, enabled)
        .await
        .map_err(log_err)
}

/// Xoá provider `id` khỏi registry.
#[tauri::command]
pub async fn agent_provider_delete(id: i32) -> Result<(), AppErrorPayload> {
    agent_provider_service::delete_provider(id)
        .await
        .map_err(log_err)
}
