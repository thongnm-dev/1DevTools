//! Model cho màn hình Workflow — chuỗi step tự động hoá (skill/prompt/runner/terminal)
//! có thể áp dụng lên một workspace. Lưu cục bộ (JSON), không phụ thuộc SQL.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Loại step — gắn với hành động thật trong app (không chỉ là nhãn hiển thị).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStepType {
    /// Mở terminal + nạp chỉ dẫn từ một Skill (thư viện Skill — bổ sung sau).
    #[serde(rename = "skill")]
    Skill,
    /// Điền placeholder rồi gõ một Prompt vào agent (thư viện Prompt — bổ sung sau).
    #[serde(rename = "prompt")]
    Prompt,
    /// Chạy một dev command (giống Runner ở Git Desktop).
    #[serde(rename = "runner")]
    Runner,
    /// Mở terminal thường với lệnh tự do.
    #[serde(rename = "terminal")]
    Terminal,
    /// Chỉ là ghi chú/checklist, không tự chạy gì.
    #[serde(rename = "custom")]
    Custom,
}

/// Một step trong workflow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Id cục bộ do frontend sinh (uuid) — không cần bộ đếm riêng vì step lồng trong workflow.
    pub id: String,
    pub name: String,
    pub step_type: WorkflowStepType,
    pub icon: String,
    #[serde(default)]
    pub description: String,
    /// Đánh dấu bước cuối cùng của workflow.
    #[serde(default)]
    pub is_latest_step: bool,
    /// Id skill gắn cho step (khi `step_type = skill`). Thư viện Skill sẽ bổ sung ở phase sau.
    #[serde(default)]
    pub skill_id: Option<i64>,
    /// Id prompt gắn cho step (khi `step_type = prompt`). Thư viện Prompt sẽ bổ sung ở phase sau.
    #[serde(default)]
    pub prompt_id: Option<i64>,
    /// Lệnh literal khi `step_type = runner` hoặc `terminal`.
    #[serde(default)]
    pub runner_command: Option<String>,
    /// Pin account AI (agent/provider) dùng cho step — trỏ vào `ai_account_store` đã có.
    #[serde(default)]
    pub ai_account_id: Option<i64>,
}

/// Vị trí một node trên canvas (kéo-thả tự do).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct NodePos {
    pub x: f64,
    pub y: f64,
}

/// Một workflow — thư viện global, tái sử dụng cho mọi workspace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub steps: Vec<WorkflowStep>,
    /// Vị trí canvas theo `step.id`.
    #[serde(default)]
    pub layout: HashMap<String, NodePos>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request tạo workflow mới (chưa có step).
#[derive(Debug, Deserialize)]
pub struct CreateWorkflowRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// Request cập nhật workflow — ghi đè toàn bộ `steps` (không có API riêng cho từng step).
#[derive(Debug, Deserialize)]
pub struct UpdateWorkflowRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub steps: Vec<WorkflowStep>,
}
