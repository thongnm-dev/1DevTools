//! Model cho màn hình Workflow — chuỗi step tự động hoá (skill/prompt/runner/terminal)
//! có thể áp dụng lên một workspace. Lưu ở PostgreSQL (bảng `workflows` +
//! `workflow_steps` + `ai_models`), mỗi user chỉ thấy workflow của mình
//! (`created_by`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Loại step — gắn với hành động thật trong app (không chỉ là nhãn hiển thị).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStepType {
    /// Mở terminal + gõ sẵn `/<skill_name>` (skill_name nhập tự do, không đối chiếu thư mục).
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

impl WorkflowStepType {
    /// Chuỗi lưu vào cột `step_type` (VARCHAR, không có CHECK constraint).
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkflowStepType::Skill => "skill",
            WorkflowStepType::Prompt => "prompt",
            WorkflowStepType::Runner => "runner",
            WorkflowStepType::Terminal => "terminal",
            WorkflowStepType::Custom => "custom",
        }
    }

    /// Đọc lại từ cột `step_type`. Giá trị lạ (dữ liệu cũ/hỏng) → `Custom`.
    pub fn from_str(value: &str) -> Self {
        match value {
            "skill" => WorkflowStepType::Skill,
            "prompt" => WorkflowStepType::Prompt,
            "runner" => WorkflowStepType::Runner,
            "terminal" => WorkflowStepType::Terminal,
            _ => WorkflowStepType::Custom,
        }
    }
}

/// Một step trong workflow (bảng `workflow_steps`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: i32,
    pub workflow_id: i32,
    pub name: String,
    pub step_type: WorkflowStepType,
    /// Slug skill dùng khi `step_type = skill` (vd `create-plan`), nhập tự do.
    #[serde(default)]
    pub skill_name: String,
    /// Id prompt gắn cho step (khi `step_type = prompt`). Thư viện Prompt cục bộ.
    #[serde(default)]
    pub prompt_id: Option<i32>,
    /// Lệnh literal khi `step_type = runner` hoặc `terminal`.
    #[serde(default)]
    pub runner_command: String,
    /// Pin account AI (agent/provider) dùng cho step — trỏ vào `ai_account_store` đã có.
    #[serde(default)]
    pub ai_account_id: Option<i32>,
    #[serde(default)]
    pub description: String,
    pub icon: String,
    #[serde(default)]
    pub step_order: i32,
    /// Đánh dấu bước cuối cùng của workflow (chỉ 1 step/workflow, service tự gỡ cờ ở step khác).
    #[serde(default)]
    pub is_latest_step: bool,
    /// Model AI dùng khi AI Cowork chạy step `skill` này.
    #[serde(default)]
    pub model_id: Option<i32>,
    pub created_at: String,
}

/// Vị trí một node trên canvas (kéo-thả tự do).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct NodePos {
    pub x: f64,
    pub y: f64,
}

/// Icon mặc định khi workflow chưa chọn icon riêng.
fn default_workflow_icon() -> String {
    "pi pi-sitemap".to_string()
}

/// Một workflow (bảng `workflows`) — thuộc sở hữu của 1 user (`created_by`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_workflow_icon")]
    pub icon: String,
    /// Vị trí canvas theo `step.id`.
    #[serde(default)]
    pub layout: HashMap<String, NodePos>,
    pub created_by: String,
    /// Số step hiện có — tính sẵn ở SP, không kèm danh sách step đầy đủ
    /// (gọi `workflow_step_list` riêng khi cần xem/sửa step của 1 workflow).
    #[serde(default)]
    pub step_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Danh mục model AI để chọn cho từng workflow step (bảng `ai_models`).
#[derive(Clone, Debug, Serialize)]
pub struct AiModel {
    pub id: i32,
    pub provider: String,
    pub model: String,
    pub version: String,
}

/// Request tạo workflow mới (chưa có step).
#[derive(Debug, Deserialize)]
pub struct CreateWorkflowRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_workflow_icon")]
    pub icon: String,
}

/// Request cập nhật tên/mô tả/icon workflow (không đụng tới step).
#[derive(Debug, Deserialize)]
pub struct UpdateWorkflowRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_workflow_icon")]
    pub icon: String,
}

/// Request tạo/cập nhật 1 step — dùng chung field cho cả 2.
#[derive(Debug, Deserialize)]
pub struct StepRequest {
    pub name: String,
    pub step_type: WorkflowStepType,
    #[serde(default)]
    pub skill_name: String,
    #[serde(default)]
    pub prompt_id: Option<i32>,
    #[serde(default)]
    pub runner_command: String,
    #[serde(default)]
    pub ai_account_id: Option<i32>,
    #[serde(default)]
    pub description: String,
    pub icon: String,
    #[serde(default)]
    pub step_order: i32,
    #[serde(default)]
    pub is_latest_step: bool,
    #[serde(default)]
    pub model_id: Option<i32>,
}
