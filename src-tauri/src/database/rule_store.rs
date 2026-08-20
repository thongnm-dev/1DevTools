//! Lưu trữ cục bộ cho thư viện Rule — 1 file JSON `rules.json`, cùng pattern
//! với `skill_store`/`git_repo_store`/`workflow_store`.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::rule::Rule;
use crate::utils::app_config;

const DATA_FILE: &str = "rules.json";

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RuleData {
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub next_id: i64,
}

fn data_path() -> PathBuf {
    app_config::data_subdir().join(DATA_FILE)
}

pub fn load() -> AppResult<RuleData> {
    let path = data_path();
    if !path.exists() {
        return Ok(RuleData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save(data: &RuleData) -> AppResult<()> {
    let path = data_path();
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
