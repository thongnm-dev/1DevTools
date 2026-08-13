//! Model cho chức năng Dev Runner — phát hiện và chạy lệnh phát triển từ repo.

use serde::{Deserialize, Serialize};

/// Nguồn gốc của một lệnh: tự phát hiện từ project file hay do người dùng thêm tay.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CommandSource {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "custom")]
    Custom,
}

/// Loại tác dụng của lệnh (giúp UI/Automation phân biệt lệnh test khỏi lệnh chạy/build).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CommandKind {
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "build")]
    Build,
}

/// Mặc định khi dữ liệu cũ (lưu trước khi có field này) không có `kind`.
fn default_kind() -> CommandKind {
    CommandKind::Run
}

/// Nhóm (loại project) của một lệnh, giúp UI phân nhóm hiển thị.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CommandCategory {
    #[serde(rename = "npm")]
    Npm,
    #[serde(rename = "flutter")]
    Flutter,
    #[serde(rename = "maven")]
    Maven,
    #[serde(rename = "gradle")]
    Gradle,
    #[serde(rename = "cargo")]
    Cargo,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "dotnet")]
    Dotnet,
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "custom")]
    Custom,
}

/// Một lệnh phát triển (detected hoặc custom).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevCommand {
    pub id: String,
    pub label: String,
    pub command: String,
    pub category: CommandCategory,
    pub source: CommandSource,
    /// File nguồn đã phát hiện ra lệnh (vd. "package.json"), rỗng nếu custom.
    #[serde(default)]
    pub source_file: String,
    #[serde(default = "default_kind")]
    pub kind: CommandKind,
}
