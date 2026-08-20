//! Model cho thư viện Rule — file markdown rời (đính kèm, không nhập nội dung
//! trực tiếp khi đăng ký skill), tương đương 1 file trong thư mục `references/`
//! cạnh `SKILL.md` (quy ước Claude Agent Skill). Skill đính kèm Rule qua
//! `rule_ids` thay vì lưu nội dung trùng lặp.

use serde::{Deserialize, Serialize};

fn default_string() -> String {
    String::new()
}

/// Một rule — nội dung markdown tái sử dụng, có thể đính kèm vào nhiều Skill.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: i64,
    pub name: String,
    #[serde(default = "default_string")]
    pub description: String,
    /// Nội dung markdown của rule (nhập tay hoặc import từ file .md có sẵn).
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request tạo/cập nhật rule.
#[derive(Debug, Deserialize)]
pub struct RuleRequest {
    pub name: String,
    #[serde(default = "default_string")]
    pub description: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
