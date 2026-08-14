//! Lưu trữ cục bộ (JSON file) cho liên kết Workspace <-> Task.
//!
//! Giống pattern của `workspace_store` / `git_repo_store` — 1 file JSON
//! `workspace_tasks.json` trong thư mục `data` bên trong AppData, mỗi máy có
//! danh sách riêng.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::workspace_task::WorkspaceTask;
use crate::utils::app_config;

const DATA_FILE: &str = "workspace_tasks.json";

/// Toàn bộ dữ liệu liên kết workspace-task được serialize xuống file.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WorkspaceTaskData {
    #[serde(default)]
    pub links: Vec<WorkspaceTask>,
    /// Bộ đếm id tự tăng (bắt đầu từ 1).
    #[serde(default)]
    pub next_id: i64,
}

fn data_path() -> PathBuf {
    app_config::data_subdir().join(DATA_FILE)
}

/// Đọc dữ liệu từ file. File chưa tồn tại → trả về mặc định (rỗng).
pub fn load() -> AppResult<WorkspaceTaskData> {
    let path = data_path();
    if !path.exists() {
        return Ok(WorkspaceTaskData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

/// Ghi dữ liệu xuống file (pretty JSON, ghi đè).
pub fn save(data: &WorkspaceTaskData) -> AppResult<()> {
    let path = data_path();
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
