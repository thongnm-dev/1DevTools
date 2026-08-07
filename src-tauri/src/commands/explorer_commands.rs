//! Tauri commands đọc thư mục / mở file/folder trong Explorer/Finder — dùng bởi
//! màn hình Git Desktop ("Show in folder") và Terminal (cây thư mục làm việc).

use crate::app::error::{log_err, AppErrorPayload};
use crate::models::explorer::ReadDirResult;
use crate::services::explorer_service;

#[tauri::command]
pub fn explorer_open(path: String) -> Result<(), AppErrorPayload> {
    explorer_service::open_in_explorer(&path).map_err(log_err)
}

#[tauri::command]
pub fn explorer_read_dir(path: String) -> Result<ReadDirResult, AppErrorPayload> {
    explorer_service::read_dir(&path).map_err(log_err)
}

#[tauri::command]
pub fn explorer_open_file(path: String) -> Result<(), AppErrorPayload> {
    explorer_service::open_file(&path).map_err(log_err)
}
