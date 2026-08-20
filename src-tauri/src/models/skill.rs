//! Model cho thư viện Skill — chỉ dẫn tái sử dụng để "nạp" cho agent khi mở
//! một terminal session (dùng độc lập hoặc gắn vào 1 step của Workflow).

use serde::{Deserialize, Serialize};

use crate::models::rule::Rule;

fn default_skill_icon() -> String {
    "pi pi-book".to_string()
}

fn default_string() -> String {
    String::new()
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
    /// Danh mục skill — tên (`name`) một mục `master_data` với `keygroup = "SKILL_TYPE"`.
    #[serde(default = "default_string")]
    pub category: String,
    /// Tech stack cụ thể, VD: "Vue 3", "FastAPI", "Flutter" ...
    #[serde(default = "default_string")]
    pub stack: String,
    /// Nội dung chỉ dẫn (markdown) — tương đương SKILL.md.
    #[serde(default)]
    pub instructions: String,
    /// Danh sách id các Rule (`crate::models::rule::Rule`) đính kèm — tương
    /// đương thư mục `references/` cạnh `SKILL.md`, nhưng đính kèm bằng cách
    /// chọn rule có sẵn thay vì nhập nội dung trực tiếp.
    #[serde(default)]
    pub rule_ids: Vec<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Skill {
    /// Xuất skill ra nội dung markdown — tương đương `SKILL.md` cộng phần
    /// `references/` được gộp vào cùng 1 file để xem nhanh (quy ước Claude
    /// Agent Skill: front-matter mô tả skill, theo sau là các mục nội dung).
    /// `attached_rules` là các Rule ứng với `rule_ids`, do caller resolve sẵn
    /// (model Skill không tự truy cập `rule_store`).
    pub fn to_markdown(&self, attached_rules: &[Rule]) -> String {
        let mut out = String::new();
        out.push_str("---\n");
        out.push_str(&format!("name: {}\n", self.name));
        if !self.description.is_empty() {
            out.push_str(&format!("description: {}\n", self.description));
        }
        if !self.category.is_empty() {
            out.push_str(&format!("category: {}\n", self.category));
        }
        if !self.stack.is_empty() {
            out.push_str(&format!("stack: {}\n", self.stack));
        }
        if !self.tags.is_empty() {
            out.push_str(&format!("tags: [{}]\n", self.tags.join(", ")));
        }
        out.push_str("---\n\n");

        out.push_str(&format!("# {}\n\n", self.name));
        if !self.description.is_empty() {
            out.push_str(&format!("{}\n\n", self.description));
        }
        out.push_str(&self.instructions);
        out.push_str("\n");

        if !attached_rules.is_empty() {
            out.push_str("\n## References\n");
            for r in attached_rules {
                out.push_str(&format!("\n### {}\n\n", r.name));
                out.push_str(&r.content);
                out.push_str("\n");
            }
        }

        out
    }
}

/// Request tạo/cập nhật skill.
#[derive(Debug, Deserialize)]
pub struct SkillRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_skill_icon")]
    pub icon: String,
    #[serde(default = "default_string")]
    pub category: String,
    #[serde(default = "default_string")]
    pub stack: String,
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub rule_ids: Vec<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
}
