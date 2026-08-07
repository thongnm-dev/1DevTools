//! Tầng lưu trữ cục bộ cho danh sách "Docker project" (build/rebuild đã lưu) của
//! màn hình Docker Desktop.
//!
//! Danh sách được lưu trong file JSON `docker_projects.json` trong thư mục AppData
//! (`%LOCALAPPDATA%\1Devtools`) — mỗi máy có danh sách riêng, không đẩy lên database
//! dùng chung. Cùng cách tiếp cận với `git_repo_store`.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::docker::DockerProject;
use crate::utils::app_config;

/// Tên file dữ liệu cục bộ.
const DATA_FILE: &str = "docker_projects.json";

/// Toàn bộ dữ liệu danh sách project được serialize xuống file.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DockerProjectData {
    #[serde(default)]
    pub projects: Vec<DockerProject>,
    /// Bộ đếm id tự tăng (bắt đầu từ 1).
    #[serde(default)]
    pub next_id: i64,
}

fn data_path() -> PathBuf {
    app_config::data_dir().join(DATA_FILE)
}

/// Đọc dữ liệu từ file. File chưa tồn tại → trả về mặc định (rỗng).
pub fn load() -> AppResult<DockerProjectData> {
    let path = data_path();
    if !path.exists() {
        return Ok(DockerProjectData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

/// Ghi dữ liệu xuống file (pretty JSON, ghi đè).
pub fn save(data: &DockerProjectData) -> AppResult<()> {
    let path = data_path();
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
