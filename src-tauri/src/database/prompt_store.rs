//! Lưu trữ cục bộ cho thư viện Prompt — 1 file JSON `prompts.json`, cùng
//! pattern với `git_repo_store`/`workflow_store`.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::result::AppResult;
use crate::models::prompt::Prompt;
use crate::utils::app_config;

const DATA_FILE: &str = "prompts.json";

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PromptData {
    #[serde(default)]
    pub prompts: Vec<Prompt>,
    #[serde(default)]
    pub next_id: i64,
}

fn data_path() -> PathBuf {
    app_config::data_subdir().join(DATA_FILE)
}

pub fn load() -> AppResult<PromptData> {
    let path = data_path();
    if !path.exists() {
        return Ok(PromptData::default());
    }
    let content = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn save(data: &PromptData) -> AppResult<()> {
    let path = data_path();
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(&path, content)?;
    Ok(())
}
