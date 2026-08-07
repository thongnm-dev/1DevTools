/// Tauri command handlers — điểm vào từ frontend qua IPC invoke.
#[path = "../commands"]
mod commands {
    /// Commands cho module xác thực người dùng.
    pub mod auth_commands;
    /// Commands cho cấu hình kết nối database.
    pub mod db_config_commands;
    /// Commands lấy thông tin hệ thống và kiểm tra mạng.
    pub mod system_commands;
    /// Commands cho module quản lý người dùng (danh sách, dùng cho member picker).
    pub mod user_commands;
    /// Commands cho module quản lý menu.
    pub mod menu_commands;
    /// Commands cho phân quyền menu theo user/role (chỉ quyền hiệu lực, dùng cho sidebar).
    pub mod menu_permission_commands;
    /// Commands cho cấu hình phân trang DataTable (đọc từ config.ini).
    pub mod pagination_commands;
    /// Commands cho màn hình Git Desktop (thao tác git + quản lý danh sách repo).
    pub mod git_commands;
    /// Command mở file/folder trong Explorer/Finder.
    pub mod explorer_commands;
    /// Tauri IPC commands cho module Terminal nhúng.
    pub mod terminal_commands;
    /// Commands cho màn hình Docker Desktop (thao tác docker + quản lý project build).
    pub mod docker_commands;
}
