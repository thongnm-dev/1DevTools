//! Tauri commands đọc thư mục / mở file/folder trong Explorer/Finder — dùng bởi
//! màn hình Git Desktop ("Show in folder") và Terminal (cây thư mục làm việc).

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::explorer::ReadDirResult;
use crate::services::explorer_service;

/// Mở `path` trong trình quản lý file của hệ điều hành (Explorer/Finder).
/// Nếu `path` là file thì mở thư mục chứa và chọn file đó.
#[tauri::command]
pub fn explorer_open(path: String) -> Result<(), AppErrorPayload> {
    explorer_service::open_in_explorer(&path).map_err(log_err)
}

/// Liệt kê nội dung thư mục `path` (file/thư mục con) cho cây thư mục ở UI.
#[tauri::command]
pub fn explorer_read_dir(path: String) -> Result<ReadDirResult, AppErrorPayload> {
    explorer_service::read_dir(&path).map_err(log_err)
}

/// Đọc nội dung text file và trả về chuỗi UTF-8.
#[tauri::command]
pub fn explorer_read_file(path: String) -> Result<String, AppErrorPayload> {
    explorer_service::read_file(&path).map_err(log_err)
}

/// Mở file `path` bằng ứng dụng mặc định của hệ điều hành.
#[tauri::command]
pub fn explorer_open_file(path: String) -> Result<(), AppErrorPayload> {
    explorer_service::open_file(&path).map_err(log_err)
}
