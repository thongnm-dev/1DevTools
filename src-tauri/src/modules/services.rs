/// Tầng business logic — xử lý nghiệp vụ, validation, điều phối.
#[path = "../services"]
mod services {
    /// Service cho module xác thực người dùng.
    pub mod auth_service;
    /// Service cho cấu hình kết nối database.
    pub mod db_config_service;
    /// Service kiểm tra kết nối internet.
    pub mod network_service;
    /// Service lấy thông tin hệ thống.
    pub mod system_service;
    /// Service cho module quản lý người dùng.
    pub mod user_service;
    /// Service cho module quản lý menu.
    pub mod menu_service;
    /// Service cho phân quyền menu (chỉ quyền hiệu lực).
    pub mod menu_permission_service;
    /// Service đọc cấu hình phân trang DataTable từ config.ini.
    pub mod pagination_service;
    /// Dữ liệu mock dùng khi database chưa kết nối được (chỉ debug build).
    pub mod mock_data;
    /// Service cho màn hình Git Desktop — gọi `git` CLI cho mọi thao tác.
    pub mod git_service;
    /// Theo dõi thay đổi file trên đĩa của repo Git đang mở (auto-refresh tab Changes).
    pub mod git_watch_service;
    /// Mở file/folder trong Explorer/Finder — dùng bởi màn hình Git Desktop.
    pub mod explorer_service;
    /// Service quản lý các phiên terminal nhúng (pseudo-terminal / PTY).
    pub mod terminal_service;
    /// Service cho màn hình Docker Desktop — gọi `docker` CLI cho mọi thao tác.
    pub mod docker_service;
}
