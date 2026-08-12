//! Lưu trữ cục bộ (JSON) danh sách custom dev commands per-repo.
//!
//! Mỗi repo được phân biệt bằng hash của đường dẫn, tránh ký tự đặc biệt trong
//! tên file. Dữ liệu lưu trong `<data_dir>/dev_runner_<hash>.json`.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::dev_runner::DevCommand;
use crate::utils::app_config;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DevRunnerData {
    #[serde(default)]
    pub commands: Vec<DevCommand>,
}

fn repo_hash(repo_path: &str) -> String {
    let mut hasher = DefaultHasher::new();
    repo_path.to_lowercase().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn data_path(repo_path: &str) -> PathBuf {
    app_config::data_dir().join(format!("dev_runner_{}.json", repo_hash(repo_path)))
}

pub fn load(repo_path: &str) -> AppResult<DevRunnerData> {
    let path = data_path(repo_path);
    if !path.exists() {
        return Ok(DevRunnerData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save(repo_path: &str, data: &DevRunnerData) -> AppResult<()> {
    let path = data_path(repo_path);
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
