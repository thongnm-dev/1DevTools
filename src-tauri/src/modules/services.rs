/// Tầng business logic — xử lý nghiệp vụ, validation, điều phối.
#[path = "../services"]
mod services {
    /// Service cho module xác thực người dùng.
    pub mod auth_service;
    /// Service cho cấu hình kết nối database.
    pub mod db_config_service;
    /// Service kiểm tra kết nối internet.
    pub mod network_service;
    /// Service lấy thông tin hệ thống.
    pub mod system_service;
    /// Service cho module quản lý người dùng.
    pub mod user_service;
    /// Service cho module quản lý role (governance).
    pub mod role_service;
    /// Service đọc/ghi cấu hình ứng dụng (config.ini).
    pub mod app_config_service;
    /// Service cho module quản lý menu.
    pub mod menu_service;
    /// Service cho phân quyền menu (chỉ quyền hiệu lực).
    pub mod menu_permission_service;
    /// Service đọc cấu hình phân trang DataTable từ config.ini.
    pub mod pagination_service;
    /// Service cho màn hình Git Desktop — gọi `git` CLI cho mọi thao tác.
    pub mod git_service;
    /// Theo dõi thay đổi file trên đĩa của repo Git đang mở (auto-refresh tab Changes).
    pub mod git_watch_service;
    /// Mở file/folder trong Explorer/Finder — dùng bởi màn hình Git Desktop.
    pub mod explorer_service;
    /// Service quản lý các phiên terminal nhúng (pseudo-terminal / PTY).
    pub mod terminal_service;
    /// Service cho màn hình Docker Desktop — gọi `docker` CLI cho mọi thao tác.
    pub mod docker_service;
    // ─── AI Usage (quản lý account AI + theo dõi usage + auto-switch) ───
    /// CRUD account + settings AI Usage (API công khai cho frontend).
    pub mod ai_acc_service;
    /// Lõi lưu trữ AI Usage + vòng lặp poll nền cập nhật usage.
    pub mod ai_usage_service;
    /// Nghiệp vụ riêng provider Claude: dò/import/capture login local.
    pub mod claude_service;
    /// Probe tình trạng usage cho từng account AI (rate-limit header, v.v.).
    pub mod ai_usage_probe;
    /// Probe usage riêng cho provider Claude (subscription OAuth usage + anthropic ratelimit).
    pub mod claude_probe;
    /// Probe usage riêng cho provider Codex/OpenAI (x-ratelimit header).
    pub mod codex_probe;
    /// Dò các login Claude đã tồn tại trên máy (đọc `.claude.json` + Keychain).
    pub mod claude_detected;
    /// Đọc credential store cho Windows + Linux (file `.credentials.json`, không có Keychain).
    #[cfg(not(target_os = "macos"))]
    pub mod claude_credentials_windows;
    /// Đọc credential store cho macOS (Keychain, lệnh `security`).
    #[cfg(target_os = "macos")]
    pub mod claude_credentials_macos;
    /// Mở terminal desktop chạy `claude`.
    pub mod claude_terminal;
    /// Mở terminal Windows (`cmd /k` qua file `.bat` tạm).
    #[cfg(target_os = "windows")]
    pub mod claude_terminal_windows;
    /// Mở terminal macOS (Terminal.app qua file `.command` tạm).
    #[cfg(target_os = "macos")]
    pub mod claude_terminal_macos;
    /// Capture login Claude đang active → lưu token vào profile (app data dir).
    pub mod claude_capture;
    /// Service phát hiện lệnh phát triển từ project files (npm/flutter/maven/cargo...).
    pub mod dev_runner_service;
    /// Service cho module cài đặt người dùng (profile + theme/language/tab_mode).
    pub mod settings_service;
}
