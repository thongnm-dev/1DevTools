//! Model cho thư viện Prompt — snippet tái sử dụng, hỗ trợ placeholder dạng
//! `{{variable}}` (parse ở frontend, backend chỉ lưu chuỗi thô).

use serde::{Deserialize, Serialize};

/// Một prompt — thư viện global, tái sử dụng cho mọi workspace/workflow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prompt {
    pub id: i64,
    pub title: String,
    /// Nội dung prompt, có thể chứa placeholder `{{var}}`.
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Free-text, không cần enum cứng.
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub usage_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// Request tạo/cập nhật prompt — dùng chung field cho cả 2.
#[derive(Debug, Deserialize)]
pub struct PromptRequest {
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub category: String,
}
