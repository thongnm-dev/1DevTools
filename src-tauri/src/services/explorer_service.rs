//! Mở file/folder trong Explorer (Windows) hoặc Finder (macOS) của hệ điều hành.

use std::path::Path;

use crate::app::error::AppError;
use crate::app::result::AppResult;

/// Hiển thị `path` trong Explorer/Finder — chọn sẵn file nếu `path` là file,
/// hoặc mở thẳng thư mục nếu `path` là folder.
pub fn open_in_explorer(path: &str) -> AppResult<()> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(AppError::new(format!("Path does not exist: {path}")));
    }

    #[cfg(target_os = "windows")]
    {
        // explorer.exe does not reliably resolve paths with forward slashes (it can
        // misparse them as switches), so normalize to backslashes before invoking it —
        // callers may build paths with `/` (e.g. `${projectPath}/input`).
        let windows_path = path.replace('/', "\\");
        if p.is_dir() {
            std::process::Command::new("explorer")
                .arg(&windows_path)
                .spawn()
                .map_err(|e| AppError::new(format!("Failed to open explorer: {e}")))?;
        } else {
            std::process::Command::new("explorer")
                .args(["/select,", &windows_path])
                .spawn()
                .map_err(|e| AppError::new(format!("Failed to open explorer: {e}")))?;
        }
    }

    #[cfg(target_os = "macos")]
    {
        let target = if p.is_file() {
            p.parent()
                .map(|pp| pp.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string())
        } else {
            path.to_string()
        };
        std::process::Command::new("open")
            .arg(&target)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open Finder: {e}")))?;
    }

    Ok(())
}
