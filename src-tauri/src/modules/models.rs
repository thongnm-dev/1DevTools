/// Các kiểu dữ liệu (model/DTO) chia theo domain.
#[path = "../models"]
mod models {
    /// Model cho module xác thực người dùng.
    pub mod auth;
    /// Model cho cấu hình kết nối database.
    pub mod db_config;
    /// Model thông tin hệ thống (username, IP, version).
    pub mod system;
    /// Model cho module quản lý người dùng.
    pub mod user;
    /// Model cho module quản lý menu.
    pub mod menu_entity;
    /// Model cho phân quyền menu (chỉ quyền hiệu lực).
    pub mod menu_permission;
}
