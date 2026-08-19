//! Model cho quản lý AI Agent Provider Model — khai báo các model của từng
//! provider được phép sử dụng trong hệ thống (VD: Claude → opus/sonnet/haiku).
//! Lưu ở bảng `agent_provider_models` (PostgreSQL), truy cập qua stored procedure.

use serde::{Deserialize, Serialize};

fn default_string() -> String {
    String::new()
}

fn default_true() -> bool {
    true
}

/// Một model thuộc về một AI Agent Provider.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentProviderModel {
    pub id: i32,
    pub provider_id: i32,
    /// Tên provider (join sẵn để hiển thị trên danh sách).
    #[serde(default = "default_string")]
    pub provider_name: String,
    pub name: String,
    #[serde(default = "default_string")]
    pub code: String,
    #[serde(default = "default_string")]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request đăng ký / cập nhật model.
#[derive(Debug, Deserialize)]
pub struct AgentProviderModelRequest {
    pub provider_id: i32,
    pub name: String,
    #[serde(default = "default_string")]
    pub code: String,
    #[serde(default = "default_string")]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}
