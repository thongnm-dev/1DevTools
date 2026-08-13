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
    /// Model cho module quản lý role (governance).
    pub mod role;
    /// Model cho cấu hình ứng dụng (config.ini).
    pub mod app_config;
    /// Model cho module quản lý menu.
    pub mod menu_entity;
    /// Model cho phân quyền menu (hiệu lực + override role/user).
    pub mod menu_permission;
    /// Model cho màn hình Git Desktop.
    pub mod git;
    /// Model cho explorer nhanh (cây thư mục Terminal, "Show in folder").
    pub mod explorer;
    /// Model cho màn hình Docker Desktop.
    pub mod docker;
    /// Model cho module AI Usage (account AI + theo dõi usage + auto-switch).
    pub mod ai_usage;
    /// Model cho chức năng Dev Runner (phát hiện và chạy lệnh phát triển).
    pub mod dev_runner;
    /// Model cho màn hình Workflow (chuỗi step tự động hoá skill/prompt/runner/terminal).
    pub mod workflow;
}
