/// Tầng command: Tauri IPC handlers, nhận request từ frontend.
#[path = "../app"]
mod app {
    /// Kiểu lỗi thống nhất `AppError`.
    pub mod error;
    /// Type alias `AppResult<T>` cho `Result<T, AppError>`.
    pub mod result;
}
