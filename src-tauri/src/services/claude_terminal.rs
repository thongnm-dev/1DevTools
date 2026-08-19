//! Mở terminal desktop (cmd.exe / Terminal.app / xterm…) chạy `claude`, tách biệt khỏi
//! nghiệp vụ quản lý account ở [`crate::services::ai_usage_service`].
//!
//! `TerminalPlatform` là trait dùng chung cho các struct platform-specific — xem
//! [`crate::services::claude_terminal_windows::WindowsTerminal`] /
//! [`crate::services::claude_terminal_macos::MacosTerminal`].

use std::path::Path;

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::services::{claude_capture, claude_detected};

/// Giá trị mặc định (tương thích cũ) khi frontend chưa truyền cấu hình provider.
const DEFAULT_COMMAND: &str = "claude";
const DEFAULT_ARGS: &str = "--dangerously-skip-permissions";
const DEFAULT_MODEL_FLAG: &str = "--model";
const DEFAULT_CONFIG_ENV: &str = "CLAUDE_CONFIG_DIR";

/// Mở terminal chạy agent CLI trong working directory chỉ định.
///
/// Toàn bộ phần đặc thù agent lấy từ DB (`agent_providers`) và truyền xuống:
/// - `command`: lệnh CLI (VD "claude", "codex").
/// - `args`: preset cờ mặc định (VD "--dangerously-skip-permissions").
/// - `model_flag` + `model`: ghép thành `<flag> <model>` (VD "--model opus").
/// - `config_env`: tên biến môi trường trỏ config dir (VD "CLAUDE_CONFIG_DIR").
///
/// `prompt`, nếu có, được truyền sẵn cho agent như một câu lệnh skill
/// (vd. `/translator-qa QA20260724`). Các tham số `None` rơi về mặc định Claude
/// để giữ tương thích ngược.
#[allow(clippy::too_many_arguments)]
pub fn open_terminal(
    config_dir: &str,
    work_dir: &str,
    prompt: Option<&str>,
    model: Option<&str>,
    command: Option<&str>,
    args: Option<&str>,
    model_flag: Option<&str>,
    config_env: Option<&str>,
) -> AppResult<()> {
    let full_command = build_agent_command(
        command.unwrap_or(DEFAULT_COMMAND),
        args.unwrap_or(DEFAULT_ARGS),
        model_flag.unwrap_or(DEFAULT_MODEL_FLAG),
        model,
        prompt,
    );
    spawn_terminal(
        config_dir,
        work_dir,
        Some(&full_command),
        config_env.unwrap_or(DEFAULT_CONFIG_ENV),
    )
}

/// Ghép câu lệnh agent: `<command> <args> <model_flag> <model> "<prompt>"`.
/// Model/prompt được sanitize (loại dấu ngoặc kép để tránh phá vỡ quoting theo OS).
/// Bỏ qua model khi `model_flag` rỗng, bỏ qua các phần rỗng.
fn build_agent_command(
    command: &str,
    args: &str,
    model_flag: &str,
    model: Option<&str>,
    prompt: Option<&str>,
) -> String {
    let base = command.trim();
    let mut out = if base.is_empty() { DEFAULT_COMMAND.to_string() } else { base.to_string() };

    let args = args.trim();
    if !args.is_empty() {
        out.push_str(&format!(" {args}"));
    }

    if let Some(m) = model {
        let sanitized = m.trim().replace('"', "");
        let flag = model_flag.trim();
        if !sanitized.is_empty() && !flag.is_empty() {
            out.push_str(&format!(" {flag} {sanitized}"));
        }
    }

    if let Some(p) = prompt {
        let sanitized = p.trim().replace('"', "");
        if !sanitized.is_empty() {
            out.push_str(&format!(" \"{sanitized}\""));
        }
    }
    out
}

/// Mở terminal chạy `claude /login` với `CLAUDE_CONFIG_DIR` tuỳ chỉnh.
pub fn open_login_terminal(config_dir: &str, work_dir: &str) -> AppResult<()> {
    spawn_terminal(config_dir, work_dir, Some("claude /login"), DEFAULT_CONFIG_ENV)
}

/// Mở terminal theo từng platform, chạy một script tạm ghép sẵn `cd` + `CLAUDE_CONFIG_DIR` + command.
///
/// `script_extension`/`script_content`/`launch` là phần khác nhau giữa các OS (bắt buộc
/// override — xem struct implement ở
/// [`crate::services::claude_terminal_windows::WindowsTerminal`] /
/// [`crate::services::claude_terminal_macos::MacosTerminal`]); `spawn` xử lý chung — ghi
/// script ra file tạm rồi launch — viết một lần rồi Windows/macOS kế thừa gọi lại thay vì
/// lặp lại logic ghi file ở mỗi platform.
pub(crate) trait TerminalPlatform {
    /// Đuôi file script tạm (`bat` cho Windows, `command` cho macOS).
    fn script_extension() -> &'static str;

    /// Nội dung script tạm ghép từ working dir / config dir / command.
    /// `config_env` là tên biến môi trường trỏ config dir (rỗng = không export).
    fn script_content(
        expanded_wd: &str,
        is_default: bool,
        expanded_dir: &str,
        config_env: &str,
        command: Option<&str>,
    ) -> String;

    /// Mở terminal chạy script đã ghi ra `script_path`.
    fn launch(script_path: &Path) -> AppResult<()>;

    /// Ghi script ra file tạm rồi launch. Dùng chung mọi platform.
    fn spawn(
        expanded_wd: &str,
        is_default: bool,
        expanded_dir: &str,
        config_env: &str,
        command: Option<&str>,
    ) -> AppResult<()> {
        static SEQ: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let script_path = std::env::temp_dir().join(format!(
            "ai_usage_terminal_{}_{seq}.{}",
            std::process::id(),
            Self::script_extension()
        ));
        let content = Self::script_content(expanded_wd, is_default, expanded_dir, config_env, command);
        std::fs::write(&script_path, content)
            .map_err(|e| AppError::new(&format!("Không thể tạo script terminal: {e}")))?;
        Self::launch(&script_path)
    }
}

/// Validate config dir / working dir, mở rộng `~`, xác định có phải config dir mặc
/// định không, rồi uỷ quyền cho struct terminal theo từng platform (Windows/macOS/Linux).
fn spawn_terminal(config_dir: &str, work_dir: &str, command: Option<&str>, config_env: &str) -> AppResult<()> {
    let dir = config_dir.trim();
    if dir.is_empty() {
        return Err(AppError::new("Config directory is required."));
    }
    let wd = work_dir.trim();
    if wd.is_empty() {
        return Err(AppError::new("Working directory is required."));
    }
    let expanded_wd = claude_capture::expand_tilde(wd)
        .to_string_lossy()
        .to_string();

    let is_default = claude_detected::is_default_config_dir(dir);
    let expanded_dir = claude_capture::expand_tilde(dir)
        .to_string_lossy()
        .to_string();

    #[cfg(target_os = "windows")]
    {
        crate::services::claude_terminal_windows::WindowsTerminal::spawn(
            &expanded_wd,
            is_default,
            &expanded_dir,
            config_env,
            command,
        )?;
    }
    #[cfg(target_os = "macos")]
    {
        crate::services::claude_terminal_macos::MacosTerminal::spawn(
            &expanded_wd,
            is_default,
            &expanded_dir,
            config_env,
            command,
        )?;
    }
    #[cfg(target_os = "linux")]
    {
        let mut bash_body = format!("cd \"{expanded_wd}\"");
        if !is_default && !config_env.trim().is_empty() {
            bash_body.push_str(&format!(" && export {config_env}=\"{expanded_dir}\""));
        }
        if let Some(cmd) = command {
            bash_body.push_str(&format!(" && {cmd}"));
        }
        bash_body.push_str("; exec bash");
        let arg = format!("bash -c '{bash_body}'");
        for term in ["x-terminal-emulator", "gnome-terminal", "xterm"] {
            if std::process::Command::new(term)
                .args(["-e", &arg])
                .spawn()
                .is_ok()
            {
                return Ok(());
            }
        }
        return Err(AppError::new("Không tìm thấy terminal emulator."));
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        return Err(AppError::new("Hệ điều hành không được hỗ trợ."));
    }

    Ok(())
}
