//! Tauri IPC commands cho AI Agent Provider Model (CRUD + bật/tắt).
//! Thin layer — uỷ quyền toàn bộ nghiệp vụ cho `agent_provider_model_service`.

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::agent_provider_model::{AgentProviderModel, AgentProviderModelRequest};
use crate::services::agent_provider_model_service;

/// Danh sách toàn bộ model (kèm tên provider), mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub async fn agent_provider_model_list() -> Result<Vec<AgentProviderModel>, AppErrorPayload> {
    agent_provider_model_service::list_models()
        .await
        .map_err(log_err)
}

/// Danh sách model đang bật — dùng cho danh mục chọn model của workflow step.
#[tauri::command]
pub async fn agent_provider_model_list_enabled(
) -> Result<Vec<AgentProviderModel>, AppErrorPayload> {
    agent_provider_model_service::list_enabled_models()
        .await
        .map_err(log_err)
}

/// Đăng ký model mới.
#[tauri::command]
pub async fn agent_provider_model_create(
    request: AgentProviderModelRequest,
) -> Result<AgentProviderModel, AppErrorPayload> {
    agent_provider_model_service::create_model(request)
        .await
        .map_err(log_err)
}

/// Cập nhật model `id`.
#[tauri::command]
pub async fn agent_provider_model_update(
    id: i32,
    request: AgentProviderModelRequest,
) -> Result<AgentProviderModel, AppErrorPayload> {
    agent_provider_model_service::update_model(id, request)
        .await
        .map_err(log_err)
}

/// Bật/tắt cho phép sử dụng model `id` trong hệ thống.
#[tauri::command]
pub async fn agent_provider_model_set_enabled(
    id: i32,
    enabled: bool,
) -> Result<AgentProviderModel, AppErrorPayload> {
    agent_provider_model_service::set_enabled(id, enabled)
        .await
        .map_err(log_err)
}

/// Xoá model `id` khỏi registry.
#[tauri::command]
pub async fn agent_provider_model_delete(id: i32) -> Result<(), AppErrorPayload> {
    agent_provider_model_service::delete_model(id)
        .await
        .map_err(log_err)
}
