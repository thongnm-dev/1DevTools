//! Tauri command mở file/folder trong Explorer/Finder — dùng bởi màn hình Git Desktop
//! (hành động "Show in folder" trên danh sách file thay đổi).

use crate::app::error::{log_err, AppErrorPayload};
use crate::services::explorer_service;

#[tauri::command]
pub fn explorer_open(path: String) -> Result<(), AppErrorPayload> {
    explorer_service::open_in_explorer(&path).map_err(log_err)
}
