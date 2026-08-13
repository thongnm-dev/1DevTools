/// Tầng truy cập dữ liệu — đọc/ghi database và file.
#[path = "../database"]
mod database {
    /// Data access cho module xác thực người dùng.
    pub mod auth_store;
    /// Khởi tạo database (tạo bảng + stored procedure) khi app khởi động.
    pub mod startup_store;
    /// Data access cho module quản lý người dùng.
    pub mod user_store;
    /// Data access cho bảng `roles` (PostgreSQL).
    pub mod role_store;
    /// Data access cho bảng `menu_configs` (PostgreSQL).
    pub mod menu_store;
    /// Data access cho bảng `role_menu_permissions` (chỉ quyền hiệu lực).
    pub mod menu_permission_store;
    /// Lưu trữ cục bộ (JSON file) danh sách repository của màn hình Git Desktop.
    pub mod git_repo_store;
    /// Lưu trữ cục bộ (JSON file) danh sách project build của màn hình Docker Desktop.
    pub mod docker_project_store;
    /// Lưu trữ cục bộ (JSON file) danh sách account AI + settings (AI Usage).
    pub mod ai_account_store;
    /// Lưu trữ cục bộ (JSON file) token profile đã capture của account AI Usage.
    pub mod ai_profile_store;
    /// Lưu trữ cục bộ (JSON file) custom dev commands per-repo.
    pub mod dev_runner_store;
    /// Lưu trữ cục bộ (JSON file) danh sách Workflow (kèm steps + layout canvas).
    pub mod workflow_store;
}
