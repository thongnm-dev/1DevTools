//! Business logic cho registry AI Agent Provider — chuẩn hoá dữ liệu, kiểm tra
//! tên/mã hợp lệ rồi uỷ quyền xuống `agent_provider_store`.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::agent_provider_store;
use crate::models::agent_provider::{AgentProvider, AgentProviderRequest};

/// Chuẩn hoá danh sách model: trim và bỏ phần tử rỗng.
fn normalize_models(models: &[String]) -> Vec<String> {
    models
        .iter()
        .map(|m| m.trim().to_string())
        .filter(|m| !m.is_empty())
        .collect()
}

/// Liệt kê toàn bộ provider, mới cập nhật gần nhất lên đầu.
pub async fn list_providers() -> AppResult<Vec<AgentProvider>> {
    agent_provider_store::list_all().await
}

/// Đăng ký provider mới: kiểm tra tên không rỗng và mã (nếu có) chưa trùng.
pub async fn create_provider(request: AgentProviderRequest) -> AppResult<AgentProvider> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Provider name is required."));
    }

    let code = request.code.trim().to_string();
    if !code.is_empty() && agent_provider_store::code_exists(&code, None).await? {
        return Err(AppError::new(format!("Provider code '{code}' already exists.")));
    }

    let models = normalize_models(&request.models);

    agent_provider_store::insert(
        &name,
        &code,
        request.provider_type.as_code(),
        request.description.trim(),
        request.icon.trim(),
        request.command.trim(),
        request.website.trim(),
        &models,
        request.enabled,
    )
    .await
}

/// Cập nhật provider: kiểm tra tồn tại và mã mới không trùng provider khác.
pub async fn update_provider(
    id: i32,
    request: AgentProviderRequest,
) -> AppResult<AgentProvider> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Provider name is required."));
    }

    agent_provider_store::find_by_id(id)
        .await?
        .ok_or_else(|| AppError::new(format!("Agent provider '{id}' not found.")))?;

    let code = request.code.trim().to_string();
    if !code.is_empty() && agent_provider_store::code_exists(&code, Some(id)).await? {
        return Err(AppError::new(format!("Provider code '{code}' already exists.")));
    }

    let models = normalize_models(&request.models);

    agent_provider_store::update(
        id,
        &name,
        &code,
        request.provider_type.as_code(),
        request.description.trim(),
        request.icon.trim(),
        request.command.trim(),
        request.website.trim(),
        &models,
        request.enabled,
    )
    .await
}

/// Bật/tắt cho phép sử dụng provider trong hệ thống.
pub async fn set_enabled(id: i32, enabled: bool) -> AppResult<AgentProvider> {
    agent_provider_store::set_enabled(id, enabled).await
}

/// Xoá provider khỏi registry.
pub async fn delete_provider(id: i32) -> AppResult<()> {
    if !agent_provider_store::delete_by_id(id).await? {
        return Err(AppError::new(format!("Agent provider '{id}' not found.")));
    }
    Ok(())
}
