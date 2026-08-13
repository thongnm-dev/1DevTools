//! Theo dõi thay đổi file trên đĩa của (nhiều) repo Git đang mở, để tab Changes
//! ở frontend tự cập nhật danh sách file mà không cần bấm nút reload.
//!
//! Cơ chế: mỗi lần frontend mở/chuyển sang 1 repo, gọi [`start`] cho path đó
//! (no-op nếu path này đã đang được theo dõi). Nhiều path có thể được theo dõi
//! đồng thời — cần thiết khi nhiều workspace (mỗi workspace 1 repo riêng) cùng
//! chạy nền trong 1 cửa sổ. `notify` báo sự kiện thay đổi file qua 1 kênh riêng
//! cho mỗi path; một thread nền gom (debounce) các sự kiện đến dồn dập rồi bắn
//! **1 event** `git-repo-changed` (kèm path) cho frontend — frontend tự lọc
//! theo path của mình để refresh đúng repo, không refresh nhầm repo khác.
//!
//! Bỏ qua thay đổi bên trong thư mục `.git/` — nội bộ Git tự ghi liên tục
//! (lock file, refs, logs...) và các thao tác Git trong app đã tự refresh UI
//! ngay sau khi gọi xong, không cần watcher báo lại (tránh nhiễu/refresh kép).

use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

/// Event bắn cho frontend khi phát hiện thay đổi trong working tree.
pub const REPO_CHANGED_EVENT: &str = "git-repo-changed";

/// Khoảng gom sự kiện trước khi bắn 1 lần (tránh bắn dồn dập lúc lưu nhiều
/// file / build tool ghi hàng loạt).
const DEBOUNCE: Duration = Duration::from_millis(500);

/// Watcher đang theo dõi 1 path. Drop struct này (remove khỏi map) là dừng
/// theo dõi: `RecommendedWatcher` đóng handle hệ điều hành, và channel Sender
/// (giữ trong closure event handler của watcher) bị đóng theo, khiến thread
/// debounce nhận `Disconnected` và tự thoát.
struct Active {
    _watcher: RecommendedWatcher,
}

/// Tập watcher đang hoạt động, key theo path đang theo dõi. Khởi tạo lười
/// bằng `OnceLock`.
fn active() -> &'static Mutex<HashMap<String, Active>> {
    static ACTIVE: OnceLock<Mutex<HashMap<String, Active>>> = OnceLock::new();
    ACTIVE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Bắt đầu theo dõi `path`. No-op nếu path này đã đang được theo dõi (không
/// đụng tới watcher của các path khác).
pub fn start(app: AppHandle, path: String) {
    let mut map = active().lock().unwrap();
    if map.contains_key(&path) {
        return;
    }

    let (tx, rx) = channel::<()>();

    let watch_path = path.clone();
    let mut watcher = match notify::recommended_watcher(move |res: notify::Result<Event>| {
        let Ok(event) = res else { return };
        if !matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            return;
        }
        if event.paths.iter().any(|p| is_ignored(p)) {
            return;
        }
        let _ = tx.send(());
    }) {
        Ok(w) => w,
        Err(e) => {
            log::warn!("Không khởi tạo được file watcher cho {watch_path}: {e}");
            return;
        }
    };

    if let Err(e) = watcher.watch(Path::new(&path), RecursiveMode::Recursive) {
        log::warn!("Không theo dõi được thư mục {path}: {e}");
        return;
    }

    let emit_path = path.clone();
    std::thread::spawn(move || {
        while rx.recv().is_ok() {
            // Gom tiếp các sự kiện đến trong khoảng DEBOUNCE trước khi bắn.
            loop {
                match rx.recv_timeout(DEBOUNCE) {
                    Ok(()) => continue,
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }
            let _ = app.emit(REPO_CHANGED_EVENT, emit_path.clone());
        }
    });

    map.insert(path, Active { _watcher: watcher });
}

/// Dừng theo dõi `path` (no-op nếu path này không đang được theo dõi). Không
/// ảnh hưởng tới watcher của các path khác.
pub fn stop(path: String) {
    active().lock().unwrap().remove(&path);
}

/// Bỏ qua thay đổi bên trong `.git/`.
fn is_ignored(p: &Path) -> bool {
    p.components().any(|c| c.as_os_str() == ".git")
}
