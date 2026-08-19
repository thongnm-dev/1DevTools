//! Đường dẫn dữ liệu và cấu hình ứng dụng.
//!
//! - `config_path()` → `config/config.ini` kế bên .exe (production) hoặc
//!   `src-tauri/config/config.ini` (development).

use ini::{EscapePolicy, Ini, WriteOption};
use std::path::{Path, PathBuf};

const APP_DIR_NAME: &str = "1Devtools";

/// Template mặc định cho `config.ini`, được embed vào binary lúc compile từ
/// `config.ini.example`. Dùng để:
/// - Tạo file khi cài đặt mới (file chưa tồn tại) → copy toàn bộ template.
/// - Migrate khi nâng cấp app: thêm những KEY mới có trong template nhưng chưa
///   tồn tại trong file hiện có (giá trị để rỗng, không đụng tới giá trị cũ).
const DEFAULT_CONFIG_TEMPLATE: &str = include_str!("../../config.ini.example");

/// Thư mục AppData dùng chung cho mọi file dữ liệu cục bộ (JSON, profile, v.v.).
///
/// Tự tạo thư mục nếu chưa tồn tại.
pub fn data_dir() -> PathBuf {
    if let Some(local_data) = dirs::data_local_dir() {
        let dir = local_data.join(APP_DIR_NAME);
        if std::fs::create_dir_all(&dir).is_ok() {
            return dir;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.to_path_buf();
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Xác định đường dẫn tới file `config.ini`.
///
/// Production: `exe_dir/config/config.ini` (hoặc `exe_dir/config.ini` legacy).
/// Development: fallback về `CARGO_MANIFEST_DIR/config/config.ini`.
///
/// Trước khi trả về, luôn đảm bảo file được tạo (cài đặt mới) hoặc migrate
/// (nâng cấp app) qua [`ensure_config_file`].
pub fn config_path() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

    if let Some(dir) = &exe_dir {
        // Tương thích cũ: config.ini nằm trực tiếp cạnh .exe.
        let legacy = dir.join("config.ini");
        if legacy.exists() {
            ensure_config_file(&legacy);
            return legacy;
        }

        let candidate = dir.join("config").join("config.ini");
        ensure_config_file(&candidate);
        return candidate;
    }

    // Development: config.ini nằm trong thư mục src-tauri/config/
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest.join("config").join("config.ini");
    ensure_config_file(&candidate);
    candidate
}

/// Đảm bảo file `config.ini` tại `path` tồn tại và đã được cập nhật theo template.
///
/// - Nếu file **chưa tồn tại**: copy toàn bộ nội dung từ template embedded
///   (`config.ini.example`).
/// - Nếu file **đã tồn tại**: migrate — thêm những KEY có trong template nhưng
///   chưa có trong file, với giá trị rỗng; giữ nguyên toàn bộ giá trị hiện có.
fn ensure_config_file(path: &Path) {
    if path.exists() {
        migrate_config_file(path);
        return;
    }

    // Cài đặt mới: tạo file từ template embedded.
    if let Some(parent) = path.parent() {
        if std::fs::create_dir_all(parent).is_err() {
            return;
        }
    }
    let _ = std::fs::write(path, DEFAULT_CONFIG_TEMPLATE);
}

/// Migrate file config hiện có: thêm các KEY mới từ template với giá trị rỗng,
/// không thay đổi các giá trị đã được người dùng thiết lập.
fn migrate_config_file(path: &Path) {
    let template = match Ini::load_from_str_noescape(DEFAULT_CONFIG_TEMPLATE) {
        Ok(t) => t,
        Err(_) => return,
    };
    let mut existing = match Ini::load_from_file_noescape(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut changed = false;
    for (section, props) in template.iter() {
        for (key, _) in props.iter() {
            let has_key = existing
                .section(section)
                .map(|s| s.contains_key(key))
                .unwrap_or(false);
            if !has_key {
                existing.with_section(section).set(key, "");
                changed = true;
            }
        }
    }

    if changed {
        let write_opt = WriteOption {
            escape_policy: EscapePolicy::Nothing,
            ..Default::default()
        };
        let _ = existing.write_to_file_opt(path, write_opt);
    }
}

/// Thư mục con `data` bên trong [`data_dir`] (AppData), nơi gom các file JSON dữ liệu
/// cục bộ (`ai_accounts.json`, `docker_projects.json`, `git_repos.json`, ...).
///
/// Tự tạo thư mục nếu chưa tồn tại.
pub fn data_subdir() -> PathBuf {
    let dir = data_dir().join("data");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Thư mục `data` cùng cấp với thư mục `config` (kế bên .exe ở production, hoặc
/// `src-tauri/data` ở development khi không lấy được exe path).
///
/// Khác với [`data_dir`] (AppData `%LOCALAPPDATA%`), thư mục này dùng cho các file
/// lịch sử làm việc gắn với từng bản cài đặt (ví dụ: state của màn AI Translate Cowork).
///
/// Tự tạo thư mục nếu chưa tồn tại.
pub fn local_data_dir() -> PathBuf {
    let dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("data")))
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data"));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_preserves_values_and_adds_missing_keys() {
        // File cũ: thiếu một số KEY, và có giá trị người dùng đã thiết lập.
        let old_config = "\
[database]
host=db.example.com
password=secret

[pagination]
ROWS=50
";
        let dir = std::env::temp_dir().join(format!("1devtools_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.ini");
        std::fs::write(&path, old_config).unwrap();

        migrate_config_file(&path);

        let ini = Ini::load_from_file_noescape(&path).unwrap();
        let db = ini.section(Some("database")).unwrap();
        // Giá trị cũ được giữ nguyên.
        assert_eq!(db.get("host"), Some("db.example.com"));
        assert_eq!(db.get("password"), Some("secret"));
        // KEY mới có trong template được thêm với giá trị rỗng.
        assert_eq!(db.get("port"), Some(""));
        assert_eq!(db.get("dbname"), Some(""));
        assert_eq!(db.get("user"), Some(""));

        let pg = ini.section(Some("pagination")).unwrap();
        assert_eq!(pg.get("ROWS"), Some("50"));
        assert_eq!(pg.get("ROWS_COMPACT"), Some(""));

        // Section hoàn toàn mới (có trong template, không có trong file cũ) cũng được thêm.
        let smtp = ini.section(Some("SMTP")).unwrap();
        assert_eq!(smtp.get("HOST"), Some(""));

        std::fs::remove_dir_all(&dir).ok();
    }
}
