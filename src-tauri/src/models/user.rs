//! Model cho module quản lý người dùng (governance).

use serde::{Deserialize, Serialize};

/// Thông tin đầy đủ của một user (dùng khi tạo/sửa/xem chi tiết).
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetail {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub position: String,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Bản tóm tắt user cho danh sách (member picker + governance list).
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub position: String,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request tạo user mới từ frontend.
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub position: Option<String>,
    pub roles: Vec<String>,
}

/// Request cập nhật user từ frontend.
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub position: Option<String>,
    pub is_active: bool,
    pub roles: Vec<String>,
}

/// Request đổi mật khẩu user từ frontend.
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}
