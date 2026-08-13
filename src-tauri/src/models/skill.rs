//! Model cho thư viện Skill — chỉ dẫn tái sử dụng để "nạp" cho agent khi mở
//! một terminal session (dùng độc lập hoặc gắn vào 1 step của Workflow).

use serde::{Deserialize, Serialize};

fn default_skill_icon() -> String {
    "pi pi-book".to_string()
}

/// Nhóm skill — giúp UI phân loại/hiển thị badge màu.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    #[serde(rename = "implement")]
    Implement,
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "docs")]
    Docs,
    #[serde(rename = "custom")]
    Custom,
}

/// Một skill — thư viện global, tái sử dụng cho mọi workspace/workflow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_skill_icon")]
    pub icon: String,
    pub category: SkillCategory,
    /// Nội dung chỉ dẫn (markdown) — tương đương SKILL.md.
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request tạo/cập nhật skill — dùng chung field cho cả 2 (không cần request riêng).
#[derive(Debug, Deserialize)]
pub struct SkillRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_skill_icon")]
    pub icon: String,
    pub category: SkillCategory,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
