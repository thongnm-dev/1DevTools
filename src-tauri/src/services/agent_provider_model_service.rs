//! Business logic cho AI Agent Provider Model — chuẩn hoá dữ liệu, kiểm tra
//! provider tồn tại + tên/mã hợp lệ rồi uỷ quyền xuống store.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::{agent_provider_model_store, agent_provider_store};
use crate::models::agent_provider_model::{AgentProviderModel, AgentProviderModelRequest};

/// Liệt kê toàn bộ model (kèm tên provider), mới cập nhật gần nhất lên đầu.
pub async fn list_models() -> AppResult<Vec<AgentProviderModel>> {
    agent_provider_model_store::list_all().await
}

/// Chỉ các model đang bật — dùng cho danh mục chọn model của workflow step.
pub async fn list_enabled_models() -> AppResult<Vec<AgentProviderModel>> {
    agent_provider_model_store::list_enabled().await
}

async fn ensure_provider_exists(provider_id: i32) -> AppResult<()> {
    agent_provider_store::find_by_id(provider_id)
        .await?
        .ok_or_else(|| AppError::new(format!("Provider '{provider_id}' not found.")))?;
    Ok(())
}

/// Đăng ký model mới: kiểm tra provider tồn tại, tên không rỗng và mã chưa trùng.
pub async fn create_model(
    request: AgentProviderModelRequest,
) -> AppResult<AgentProviderModel> {
    ensure_provider_exists(request.provider_id).await?;

    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Model name is required."));
    }

    let code = request.code.trim().to_string();
    if !code.is_empty()
        && agent_provider_model_store::code_exists(request.provider_id, &code, None).await?
    {
        return Err(AppError::new(format!(
            "Model code '{code}' already exists for this provider."
        )));
    }

    agent_provider_model_store::insert(
        request.provider_id,
        &name,
        &code,
        request.version.trim(),
        request.description.trim(),
        request.enabled,
    )
    .await
}

/// Cập nhật model: kiểm tra tồn tại, provider hợp lệ và mã mới không trùng.
pub async fn update_model(
    id: i32,
    request: AgentProviderModelRequest,
) -> AppResult<AgentProviderModel> {
    agent_provider_model_store::find_by_id(id)
        .await?
        .ok_or_else(|| AppError::new(format!("Provider model '{id}' not found.")))?;

    ensure_provider_exists(request.provider_id).await?;

    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Model name is required."));
    }

    let code = request.code.trim().to_string();
    if !code.is_empty()
        && agent_provider_model_store::code_exists(request.provider_id, &code, Some(id)).await?
    {
        return Err(AppError::new(format!(
            "Model code '{code}' already exists for this provider."
        )));
    }

    agent_provider_model_store::update(
        id,
        request.provider_id,
        &name,
        &code,
        request.version.trim(),
        request.description.trim(),
        request.enabled,
    )
    .await
}

/// Bật/tắt cho phép sử dụng model trong hệ thống.
pub async fn set_enabled(id: i32, enabled: bool) -> AppResult<AgentProviderModel> {
    agent_provider_model_store::set_enabled(id, enabled).await
}

/// Xoá model khỏi registry.
pub async fn delete_model(id: i32) -> AppResult<()> {
    if !agent_provider_model_store::delete_by_id(id).await? {
        return Err(AppError::new(format!("Provider model '{id}' not found.")));
    }
    Ok(())
}
