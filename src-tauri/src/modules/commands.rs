/// Tauri command handlers — điểm vào từ frontend qua IPC invoke.
#[path = "../commands"]
mod commands {
    /// Commands cho module xác thực người dùng.
    pub mod auth_commands;
    /// Commands cho cấu hình kết nối database.
    pub mod db_config_commands;
    /// Commands lấy thông tin hệ thống và kiểm tra mạng.
    pub mod system_commands;
    /// Commands cho module quản lý người dùng (governance: CRUD + list).
    pub mod user_commands;
    /// Commands cho module quản lý role (governance).
    pub mod role_commands;
    /// Commands đọc/ghi cấu hình ứng dụng (config.ini).
    pub mod app_config_commands;
    /// Commands cho module quản lý menu (governance: list + save).
    pub mod menu_commands;
    /// Commands cho phân quyền menu theo user/role (hiệu lực + override).
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
    /// Commands cho module AI Usage (quản lý account AI + probe usage + auto-switch).
    pub mod ai_usage_commands;
    /// Commands cho chức năng Dev Runner (phát hiện và chạy lệnh phát triển).
    pub mod dev_runner_commands;
    /// Commands cho màn hình Workflow (CRUD + layout canvas).
    pub mod workflow_commands;
    /// Commands cho registry Workspace (CRUD, đồng thời là tab bar).
    pub mod workspace_commands;
    /// Commands cho thư viện Skill (CRUD).
    pub mod skill_commands;
    /// Commands cho thư viện Prompt (CRUD + đếm lượt dùng).
    pub mod prompt_commands;
}
