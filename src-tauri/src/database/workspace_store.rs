//! Lưu trữ cục bộ cho danh sách Workspace đã mở (registry, đồng thời là tab bar).
//!
//! Giống pattern của `git_repo_store` — 1 file JSON `workspaces.json` trong thư
//! mục `data` bên trong AppData, mỗi máy có danh sách riêng.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::workspace::Workspace;
use crate::utils::app_config;

const DATA_FILE: &str = "workspaces.json";

/// Toàn bộ dữ liệu danh sách workspace được serialize xuống file.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WorkspaceData {
    #[serde(default)]
    pub workspaces: Vec<Workspace>,
    /// Bộ đếm id tự tăng (bắt đầu từ 1).
    #[serde(default)]
    pub next_id: i64,
}

fn data_path() -> PathBuf {
    app_config::data_subdir().join(DATA_FILE)
}

/// Đọc dữ liệu từ file. File chưa tồn tại → trả về mặc định (rỗng).
pub fn load() -> AppResult<WorkspaceData> {
    let path = data_path();
    if !path.exists() {
        return Ok(WorkspaceData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

/// Ghi dữ liệu xuống file (pretty JSON, ghi đè).
pub fn save(data: &WorkspaceData) -> AppResult<()> {
    let path = data_path();
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
