//! Model cho quản lý AI Agent Provider — khai báo các loại AI Agent được phép
//! sử dụng trong hệ thống (Claude Code, Codex, Gemini CLI, ...). Lưu ở bảng
//! `agent_providers` (PostgreSQL), truy cập qua stored procedure.

use serde::{Deserialize, Serialize};

fn default_icon() -> String {
    "pi pi-android".to_string()
}

fn default_string() -> String {
    String::new()
}

fn default_true() -> bool {
    true
}

/// Loại provider — phân nhóm theo nhà cung cấp AI Agent.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum AgentProviderType {
    #[serde(rename = "claude")]
    Claude,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "copilot")]
    Copilot,
    #[serde(rename = "cursor")]
    Cursor,
    /// Catch-all cho provider tuỳ chỉnh và giá trị chưa biết.
    #[serde(other)]
    #[default]
    Custom,
}

impl AgentProviderType {
    /// Mã lưu xuống DB (khớp `#[serde(rename = ...)]`).
    pub fn as_code(&self) -> &'static str {
        match self {
            AgentProviderType::Claude => "claude",
            AgentProviderType::Codex => "codex",
            AgentProviderType::Gemini => "gemini",
            AgentProviderType::Copilot => "copilot",
            AgentProviderType::Cursor => "cursor",
            AgentProviderType::Custom => "custom",
        }
    }

    /// Phân giải mã đọc từ DB về enum (giá trị lạ → Custom).
    pub fn from_code(code: &str) -> Self {
        match code {
            "claude" => AgentProviderType::Claude,
            "codex" => AgentProviderType::Codex,
            "gemini" => AgentProviderType::Gemini,
            "copilot" => AgentProviderType::Copilot,
            "cursor" => AgentProviderType::Cursor,
            _ => AgentProviderType::Custom,
        }
    }
}

/// Một AI Agent Provider được đăng ký trong hệ thống.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentProvider {
    pub id: i32,
    pub name: String,
    #[serde(default = "default_string")]
    pub code: String,
    pub provider_type: AgentProviderType,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_icon")]
    pub icon: String,
    /// Lệnh CLI để khởi chạy agent, VD: "claude".
    #[serde(default = "default_string")]
    pub command: String,
    /// Trang chủ / tài liệu.
    #[serde(default = "default_string")]
    pub website: String,
    /// Danh sách model được hỗ trợ.
    #[serde(default)]
    pub models: Vec<String>,
    /// Các cờ CLI dựng sẵn (preset); phần tử đầu là mặc định khi launch.
    #[serde(default)]
    pub presets: Vec<String>,
    /// Cờ chỉ định model khi chạy CLI, VD: "--model". Rỗng = không truyền model.
    #[serde(default = "default_string")]
    pub model_flag: String,
    /// Tên biến môi trường trỏ config dir, VD: "CLAUDE_CONFIG_DIR". Rỗng = không set.
    #[serde(default = "default_string")]
    pub config_env: String,
    /// Có cho phép sử dụng trong hệ thống hay không.
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request đăng ký / cập nhật provider.
#[derive(Debug, Deserialize)]
pub struct AgentProviderRequest {
    pub name: String,
    #[serde(default = "default_string")]
    pub code: String,
    pub provider_type: AgentProviderType,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_icon")]
    pub icon: String,
    #[serde(default = "default_string")]
    pub command: String,
    #[serde(default = "default_string")]
    pub website: String,
    #[serde(default)]
    pub models: Vec<String>,
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default = "default_string")]
    pub model_flag: String,
    #[serde(default = "default_string")]
    pub config_env: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}
