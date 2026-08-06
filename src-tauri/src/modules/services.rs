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
    pub mod menu_config_service;
    /// Service cho phân quyền menu (chỉ quyền hiệu lực).
    pub mod menu_permission_service;
    /// Service đọc cấu hình phân trang DataTable từ config.ini.
    pub mod pagination_service;
}
