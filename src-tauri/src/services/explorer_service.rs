//! Đọc thư mục và mở file/folder trong Explorer (Windows) hoặc Finder (macOS).

use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::explorer::{FileEntry, ReadDirResult};

/// Kiểm tra file ẩn trên Windows: dựa vào cờ thuộc tính `FILE_ATTRIBUTE_HIDDEN`.
#[cfg(target_os = "windows")]
fn is_hidden(_path: &Path, meta: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
    meta.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0
}

/// Kiểm tra file ẩn trên Unix: theo quy ước tên bắt đầu bằng dấu chấm.
#[cfg(not(target_os = "windows"))]
fn is_hidden(path: &Path, _meta: &fs::Metadata) -> bool {
    path.file_name()
        .map(|n| n.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}

/// Dựng một `FileEntry` từ metadata: tên, đường dẫn, is_dir, kích thước, thời
/// điểm sửa (định dạng chuỗi) và phần mở rộng viết thường.
fn build_file_entry(path: &Path, meta: &fs::Metadata) -> Option<FileEntry> {
    let name = path.file_name()?.to_string_lossy().to_string();
    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| {
            let secs = d.as_secs() as i64;
            chrono::DateTime::from_timestamp(secs, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default()
        })
        .unwrap_or_default();
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    Some(FileEntry {
        name,
        path: path.to_string_lossy().to_string(),
        is_dir: meta.is_dir(),
        size: if meta.is_file() { meta.len() } else { 0 },
        modified,
        extension,
    })
}

/// Liệt kê nội dung một thư mục (bỏ qua file/thư mục ẩn), thư mục xếp trước file.
pub fn read_dir(dir_path: &str) -> AppResult<ReadDirResult> {
    let path = Path::new(dir_path);
    if !path.exists() {
        return Err(AppError::new(format!("Path does not exist: {dir_path}")));
    }
    if !path.is_dir() {
        return Err(AppError::new(format!("Not a directory: {dir_path}")));
    }

    let mut entries: Vec<FileEntry> = Vec::new();
    let read = fs::read_dir(path)
        .map_err(|e| AppError::new(format!("Cannot read directory: {e}")))?;

    for dir_entry in read.flatten() {
        let ep = dir_entry.path();
        let Ok(meta) = dir_entry.metadata() else {
            continue;
        };
        if is_hidden(&ep, &meta) {
            continue;
        }
        if let Some(fe) = build_file_entry(&ep, &meta) {
            entries.push(fe);
        }
    }

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(ReadDirResult {
        path: path.to_string_lossy().to_string(),
        entries,
    })
}

/// Mở một file bằng ứng dụng mặc định của hệ điều hành (khác `open_in_explorer`,
/// vốn chỉ chọn/hiển thị file trong Explorer/Finder mà không chạy nó).
pub fn open_file(path: &str) -> AppResult<()> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(AppError::new(format!("Path does not exist: {path}")));
    }
    if p.is_dir() {
        return Err(AppError::new(format!("Expected a file path, got a directory: {path}")));
    }

    #[cfg(target_os = "windows")]
    {
        let windows_path = path.replace('/', "\\");
        std::process::Command::new("explorer")
            .arg(&windows_path)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open file: {e}")))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open file: {e}")))?;
    }

    Ok(())
}

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
