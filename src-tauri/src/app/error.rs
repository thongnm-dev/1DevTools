//! Kiểu lỗi chung (AppError) cho toàn bộ ứng dụng.
//!
//! Hỗ trợ chuyển đổi tự động từ lỗi `std::io::Error` thông qua `From` trait.
//!
//! Mỗi lỗi mang một `code` ổn định (ví dụ `AUTH_INVALID_CREDENTIALS`) để frontend
//! map sang khóa i18n và dịch theo ngôn ngữ đang chọn — backend không cần biết
//! locale của client. `message` chỉ dùng cho log và làm fallback hiển thị khi
//! frontend chưa có bản dịch cho `code` đó.

use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};

/// Mã lỗi mặc định khi chưa gán `code` cụ thể — frontend sẽ hiển thị nguyên `message`.
pub const UNKNOWN_ERROR_CODE: &str = "UNKNOWN";

/// Kiểu lỗi thống nhất cho toàn bộ tầng business logic và data access.
#[derive(Debug)]
pub struct AppError {
    /// Mã lỗi ổn định để frontend dịch theo ngôn ngữ đang chọn.
    code: String,
    /// Nội dung mô tả lỗi (tiếng Anh), dùng cho log và làm fallback hiển thị.
    message: String,
}

impl AppError {
    /// Tạo lỗi mới với mã mặc định `UNKNOWN` — `message` sẽ được hiển thị trực tiếp
    /// cho người dùng vì frontend chưa có bản dịch riêng cho lỗi này.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            code: UNKNOWN_ERROR_CODE.to_string(),
            message: message.into(),
        }
    }

    /// Tạo lỗi mới với `code` ổn định để frontend map sang khóa i18n.
    pub fn with_code(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Hiển thị nội dung lỗi dạng text (dùng cho `.to_string()` và logging).
impl Display for AppError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AppError {}

/// Chuyển đổi tự động từ `std::io::Error` (lỗi đọc/ghi file, network I/O).
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

/// Chuyển đổi tự động từ `serde_json::Error` (lỗi parse/serialize JSON).
impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(error.to_string())
    }
}

/// Payload lỗi gửi qua IPC cho frontend. `code` để dịch, `message` để log/fallback.
#[derive(Debug, Serialize)]
pub struct AppErrorPayload {
    pub code: String,
    pub message: String,
}

impl From<&AppError> for AppErrorPayload {
    fn from(error: &AppError) -> Self {
        Self {
            code: error.code.clone(),
            message: error.message.clone(),
        }
    }
}

/// Ghi log lỗi (dạng Debug chi tiết) rồi trả về payload `{code, message}` cho
/// frontend — dùng cho `.map_err(log_err)` ở tầng command.
pub fn log_err(error: AppError) -> AppErrorPayload {
    log::error!("{error:?}");
    AppErrorPayload::from(&error)
}
