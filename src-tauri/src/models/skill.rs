//! Model cho thư viện Skill — chỉ dẫn tái sử dụng để "nạp" cho agent khi mở
//! một terminal session (dùng độc lập hoặc gắn vào 1 step của Workflow).

use serde::{Deserialize, Serialize};

fn default_skill_icon() -> String {
    "pi pi-book".to_string()
}

fn default_string() -> String {
    String::new()
}

/// Loại skill — phân theo domain/mục đích sử dụng.
/// Các giá trị cũ (implement, review, test, release, docs) sẽ fallback về Custom.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum SkillType {
    #[serde(rename = "general")]
    General,
    #[serde(rename = "frontend")]
    Frontend,
    #[serde(rename = "backend")]
    Backend,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "devops")]
    DevOps,
    #[serde(rename = "translation")]
    Translation,
    #[serde(rename = "design")]
    Design,
    #[serde(rename = "writing")]
    Writing,
    #[serde(rename = "data")]
    Data,
    /// Catch-all cho giá trị cũ và loại tùy chỉnh
    #[serde(other)]
    #[default]
    Custom,
}

// Backward compat alias
pub type SkillCategory = SkillType;

/// Một skill — thư viện global, tái sử dụng cho mọi workspace/workflow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_skill_icon")]
    pub icon: String,
    pub category: SkillType,
    /// Tech stack cụ thể, VD: "Vue 3", "FastAPI", "Flutter" ...
    #[serde(default = "default_string")]
    pub stack: String,
    /// Nội dung chỉ dẫn (markdown) — tương đương SKILL.md.
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request tạo/cập nhật skill.
#[derive(Debug, Deserialize)]
pub struct SkillRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_skill_icon")]
    pub icon: String,
    pub category: SkillType,
    #[serde(default = "default_string")]
    pub stack: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
