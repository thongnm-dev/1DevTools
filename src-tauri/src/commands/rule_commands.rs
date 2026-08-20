//! Tauri IPC commands cho thư viện Rule (CRUD + export xem markdown).

use crate::app::error::{log_err, AppError, AppErrorPayload};
use crate::database::rule_store;
use crate::models::rule::{Rule, RuleRequest};
use crate::utils::app_config;

/// Danh sách toàn bộ rule, mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub fn rule_list() -> Result<Vec<Rule>, AppErrorPayload> {
    let data = rule_store::load().map_err(log_err)?;
    let mut rules = data.rules;
    rules.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(rules)
}

/// Tạo rule mới.
#[tauri::command]
pub fn rule_create(request: RuleRequest) -> Result<Rule, AppErrorPayload> {
    let mut data = rule_store::load().map_err(log_err)?;
    data.next_id += 1;
    let now = chrono::Local::now().to_rfc3339();
    let rule = Rule {
        id: data.next_id,
        name: request.name,
        description: request.description,
        content: request.content,
        tags: request.tags,
        created_at: now.clone(),
        updated_at: now,
    };
    data.rules.push(rule.clone());
    rule_store::save(&data).map_err(log_err)?;
    Ok(rule)
}

/// Cập nhật rule.
#[tauri::command]
pub fn rule_update(id: i64, request: RuleRequest) -> Result<Rule, AppErrorPayload> {
    let mut data = rule_store::load().map_err(log_err)?;
    let rule = data
        .rules
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Rule #{id} không tồn tại"))))?;
    rule.name = request.name;
    rule.description = request.description;
    rule.content = request.content;
    rule.tags = request.tags;
    rule.updated_at = chrono::Local::now().to_rfc3339();
    let result = rule.clone();
    rule_store::save(&data).map_err(log_err)?;
    Ok(result)
}

/// Xoá rule.
#[tauri::command]
pub fn rule_delete(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = rule_store::load().map_err(log_err)?;
    data.rules.retain(|r| r.id != id);
    rule_store::save(&data).map_err(log_err)?;
    Ok(())
}

/// Xuất rule ra file markdown (`.md`) trong thư mục dữ liệu app, trả về
/// đường dẫn file để mở lại bằng `MarkdownPreviewDialog`.
#[tauri::command]
pub fn rule_export(id: i64) -> Result<String, AppErrorPayload> {
    let data = rule_store::load().map_err(log_err)?;
    let rule = data
        .rules
        .iter()
        .find(|r| r.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Rule #{id} không tồn tại"))))?;

    let safe_name: String = rule
        .name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let file_name = format!("{safe_name}-{id}.md");

    let dir = app_config::data_subdir().join("rule_exports");
    std::fs::create_dir_all(&dir).map_err(|e| log_err(AppError::new(format!("Không tạo được thư mục export: {e}"))))?;
    let path = dir.join(file_name);

    std::fs::write(&path, &rule.content)
        .map_err(|e| log_err(AppError::new(format!("Không ghi được file: {e}"))))?;

    Ok(path.to_string_lossy().to_string())
}
