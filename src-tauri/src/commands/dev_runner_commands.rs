//! Tauri command handlers cho chức năng Dev Runner.

use crate::app::error::{log_err, AppError, AppErrorPayload};
use crate::database::dev_runner_store;
use crate::models::dev_runner::DevCommand;
use crate::services::dev_runner_service;

/// Phát hiện các lệnh phát triển từ project files trong `repo_path`.
#[tauri::command]
pub async fn detect_dev_commands(repo_path: String) -> Result<Vec<DevCommand>, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || {
        Ok(dev_runner_service::detect_commands(&repo_path))
    })
    .await
    .map_err(|e| log_err(AppError::new(e.to_string())))?
}

/// Đọc danh sách custom commands đã lưu cho `repo_path`.
#[tauri::command]
pub fn load_custom_commands(repo_path: String) -> Result<Vec<DevCommand>, AppErrorPayload> {
    let data = dev_runner_store::load(&repo_path).map_err(log_err)?;
    Ok(data.commands)
}

/// Lưu danh sách custom commands cho `repo_path` (ghi đè toàn bộ).
#[tauri::command]
pub fn save_custom_commands(
    repo_path: String,
    commands: Vec<DevCommand>,
) -> Result<(), AppErrorPayload> {
    let data = dev_runner_store::DevRunnerData { commands };
    dev_runner_store::save(&repo_path, &data).map_err(log_err)?;
    Ok(())
}
