/// Tầng truy cập dữ liệu — đọc/ghi database và file.
#[path = "../database"]
mod database {
    /// Data access cho module xác thực người dùng.
    pub mod auth_store;
    /// Khởi tạo database (tạo bảng + stored procedure) khi app khởi động.
    pub mod startup_store;
    /// Data access cho module quản lý người dùng.
    pub mod user_store;
    /// Data access cho bảng `menu_configs` (PostgreSQL).
    pub mod menu_config_store;
    /// Data access cho bảng `role_menu_permissions` (chỉ quyền hiệu lực).
    pub mod menu_permission_store;
}
