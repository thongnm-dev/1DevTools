// Danh sách toàn bộ Tauri command handlers, tách riêng khỏi `lib.rs` cho gọn.
//
// `tauri::generate_handler!` chỉ nhận được MỘT danh sách token duy nhất tại chỗ gọi
// (không thể chia thành nhiều lời gọi macro rồi gộp lại), nên toàn bộ danh sách vẫn
// nằm trong một khối — nhưng được gom về đây thay vì làm phình `lib.rs`.

use commands::auth_commands::*;
use commands::db_config_commands::*;
use commands::menu_commands::*;
use commands::menu_permission_commands::*;
use commands::pagination_commands::*;
use commands::system_commands::*;
use commands::user_commands::*;

/// Xây dựng handler cho `Builder::invoke_handler`, gộp toàn bộ command đã đăng ký.
///
/// Dùng thẳng runtime `tauri::Wry` (desktop) thay vì generic `R: Runtime`, vì một số
/// command nhận tham số kiểu cụ thể `AppHandle`/`Window` (mặc định là `AppHandle<Wry>`)
/// — nếu để generic, trình biên dịch không thể suy ra `AppHandle<Wry>: CommandArg<'_, R>`.
fn build_invoke_handler() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        // === Auth commands ===
        login,
        request_password_reset,
        verify_password_reset,
        // === System commands ===
        get_system_info,
        check_internet_connection,
        // === Database config commands ===
        check_database_status,
        get_database_config,
        test_database_config,
        save_database_config,
        // === User commands (list only, used by the member picker) ===
        list_users,
        // === Menu config commands ===
        list_menu_configs,
        // === Menu permission commands (effective only, drives the sidebar) ===
        list_effective_menu_permissions,
        // === Pagination config command ===
        get_pagination_config
    ]
}
