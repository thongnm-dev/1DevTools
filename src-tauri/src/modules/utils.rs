/// Tiện ích hạ tầng dùng chung (network, time, encoding, database).
#[path = "../utils"]
mod utils {
    /// Kiểm tra kết nối internet và lấy IP local.
    pub mod network;
    /// Kết nối PostgreSQL, tạo bảng và stored procedure.
    pub mod pgsql_connect;
    /// Hàm tiện ích lấy timestamp hiện tại.
    pub mod time;
    /// Đường dẫn dữ liệu và cấu hình ứng dụng (AppData + config.ini).
    pub mod app_config;
    /// Gửi email qua SMTP (dùng cho reset password, v.v.).
    pub mod email;
    /// Ghi log lỗi ra file (logs/errors_log.log).
    pub mod logger;
}
