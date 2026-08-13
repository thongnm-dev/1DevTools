//! Lưu trữ cục bộ cho danh sách Workflow.
//!
//! Toàn bộ workflow (kèm steps + layout canvas) được lưu trong 1 file JSON
//! `workflows.json` trong thư mục `data` bên trong AppData — giống pattern
//! của `git_repo_store`/`ai_account_store`, không đẩy lên database dùng chung.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::workflow::Workflow;
use crate::utils::app_config;

const DATA_FILE: &str = "workflows.json";

/// Toàn bộ dữ liệu danh sách workflow được serialize xuống file.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WorkflowData {
    #[serde(default)]
    pub workflows: Vec<Workflow>,
    /// Bộ đếm id tự tăng (bắt đầu từ 1).
    #[serde(default)]
    pub next_id: i64,
}

fn data_path() -> PathBuf {
    app_config::data_subdir().join(DATA_FILE)
}

/// Đọc dữ liệu từ file. File chưa tồn tại → trả về mặc định (rỗng).
pub fn load() -> AppResult<WorkflowData> {
    let path = data_path();
    if !path.exists() {
        return Ok(WorkflowData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

/// Ghi dữ liệu xuống file (pretty JSON, ghi đè).
pub fn save(data: &WorkflowData) -> AppResult<()> {
    let path = data_path();
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
