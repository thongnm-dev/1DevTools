//! Model cho module quản lý role (governance).

use serde::{Deserialize, Serialize};

/// Một role kèm số lượng user đang được gán.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleSummary {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_count: i64,
    pub created_at: String,
}

/// Request tạo role mới từ frontend.
#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Request cập nhật role từ frontend.
#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub description: Option<String>,
}
