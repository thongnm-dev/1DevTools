// Danh sách toàn bộ Tauri command handlers, tách riêng khỏi `lib.rs` cho gọn.
//
// `tauri::generate_handler!` chỉ nhận được MỘT danh sách token duy nhất tại chỗ gọi
// (không thể chia thành nhiều lời gọi macro rồi gộp lại), nên toàn bộ danh sách vẫn
// nằm trong một khối — nhưng được gom về đây thay vì làm phình `lib.rs`.

use commands::auth_commands::*;
use commands::db_config_commands::*;
use commands::explorer_commands::*;
use commands::git_commands::*;
use commands::menu_commands::*;
use commands::menu_permission_commands::*;
use commands::pagination_commands::*;
use commands::system_commands::*;
use commands::terminal_commands::*;
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
        get_pagination_config,
        // === Explorer commands (dùng bởi Git Desktop "Show in folder" và cây thư mục Terminal) ===
        explorer_open,
        explorer_read_dir,
        explorer_open_file,
        // === Git Desktop commands: quản lý danh sách repo (JSON cục bộ) ===
        git_list_repos,
        git_add_repo,
        git_remove_repo,
        git_touch_repo,
        // === Git Desktop commands: đọc trạng thái/diff/log/branch/stash/worktree/tag ===
        git_repo_info,
        git_status,
        git_file_diff,
        git_commit_file_diff,
        git_log,
        git_log_search,
        git_graph,
        git_commit_detail,
        git_branches,
        git_stash_list,
        git_worktree_list,
        git_list_conflicts,
        git_tag_list,
        git_blame,
        git_compare_file_diff,
        // === Git Desktop commands: thao tác ghi/mạng ===
        git_stage,
        git_unstage,
        git_discard,
        git_commit,
        git_amend_commit,
        git_checkout_branch,
        git_create_branch,
        git_delete_branch,
        git_fetch,
        git_pull,
        git_push,
        git_stash_save,
        git_stash_apply,
        git_stash_drop,
        git_clone,
        git_undo_last_commit,
        git_reset,
        git_revert,
        git_revert_abort,
        git_rebase,
        git_rebase_abort,
        git_rebase_continue,
        git_tag_create,
        git_tag_delete,
        git_merge,
        git_merge_abort,
        git_commit_no_edit,
        git_resolve_conflict,
        git_cleanup_scan,
        git_cleanup_delete,
        git_compare,
        git_create_pull_request,
        git_list_pull_requests,
        git_open_url,
        git_open_terminal,
        git_open_vscode,
        git_cherry_pick,
        git_cherry_pick_abort,
        git_cherry_pick_continue,
        git_worktree_add,
        git_worktree_remove,
        // === Git Desktop commands: file watcher (auto-refresh tab Changes) ===
        git_watch_start,
        git_watch_stop,
        // === Terminal commands (PTY nhúng: spawn/write/resize/kill) ===
        terminal_spawn,
        terminal_write,
        terminal_resize,
        terminal_kill
    ]
}
