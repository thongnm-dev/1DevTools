//! Tauri command handlers cho cấu hình phân trang dùng chung toàn app.

use crate::services::pagination_service::{self, PaginationConfig};

/// Trả về cấu hình phân trang mặc định (kích thước trang, danh sách lựa chọn…)
/// để frontend khởi tạo các bảng dữ liệu đồng nhất.
#[tauri::command]
pub fn get_pagination_config() -> PaginationConfig {
    pagination_service::get_pagination_config()
}
