//! Tauri IPC commands cho thư viện Prompt (CRUD + đếm lượt dùng).

use crate::app::error::{log_err, AppError, AppErrorPayload};
use crate::database::prompt_store;
use crate::models::prompt::{Prompt, PromptRequest};

/// Danh sách toàn bộ prompt, mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub fn prompt_list() -> Result<Vec<Prompt>, AppErrorPayload> {
    let data = prompt_store::load().map_err(log_err)?;
    let mut prompts = data.prompts;
    prompts.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(prompts)
}

/// Tạo prompt mới.
#[tauri::command]
pub fn prompt_create(request: PromptRequest) -> Result<Prompt, AppErrorPayload> {
    let mut data = prompt_store::load().map_err(log_err)?;
    data.next_id += 1;
    let now = chrono::Local::now().to_rfc3339();
    let prompt = Prompt {
        id: data.next_id,
        title: request.title,
        body: request.body,
        tags: request.tags,
        category: request.category,
        usage_count: 0,
        created_at: now.clone(),
        updated_at: now,
    };
    data.prompts.push(prompt.clone());
    prompt_store::save(&data).map_err(log_err)?;
    Ok(prompt)
}

/// Cập nhật prompt.
#[tauri::command]
pub fn prompt_update(id: i64, request: PromptRequest) -> Result<Prompt, AppErrorPayload> {
    let mut data = prompt_store::load().map_err(log_err)?;
    let prompt = data
        .prompts
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Prompt #{id} không tồn tại"))))?;
    prompt.title = request.title;
    prompt.body = request.body;
    prompt.tags = request.tags;
    prompt.category = request.category;
    prompt.updated_at = chrono::Local::now().to_rfc3339();
    let result = prompt.clone();
    prompt_store::save(&data).map_err(log_err)?;
    Ok(result)
}

/// Xoá prompt.
#[tauri::command]
pub fn prompt_delete(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = prompt_store::load().map_err(log_err)?;
    data.prompts.retain(|p| p.id != id);
    prompt_store::save(&data).map_err(log_err)?;
    Ok(())
}

/// Tăng đếm lượt dùng (gọi khi người dùng bấm Copy/Insert) — không đổi `updated_at`.
#[tauri::command]
pub fn prompt_mark_used(id: i64) -> Result<Prompt, AppErrorPayload> {
    let mut data = prompt_store::load().map_err(log_err)?;
    let prompt = data
        .prompts
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Prompt #{id} không tồn tại"))))?;
    prompt.usage_count += 1;
    let result = prompt.clone();
    prompt_store::save(&data).map_err(log_err)?;
    Ok(result)
}
