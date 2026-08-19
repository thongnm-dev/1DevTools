// Danh sách toàn bộ Tauri command handlers, tách riêng khỏi `lib.rs` cho gọn.
//
// `tauri::generate_handler!` chỉ nhận được MỘT danh sách token duy nhất tại chỗ gọi
// (không thể chia thành nhiều lời gọi macro rồi gộp lại), nên toàn bộ danh sách vẫn
// nằm trong một khối — nhưng được gom về đây thay vì làm phình `lib.rs`.

use commands::ai_usage_commands::*;
use commands::app_config_commands::*;
use commands::auth_commands::*;
use commands::db_config_commands::*;
use commands::dev_runner_commands::*;
use commands::docker_commands::*;
use commands::explorer_commands::*;
use commands::git_commands::*;
use commands::menu_commands::*;
use commands::menu_permission_commands::*;
use commands::pagination_commands::*;
use commands::role_commands::*;
use commands::system_commands::*;
use commands::task_commands::*;
use commands::terminal_commands::*;
use commands::user_commands::*;
use commands::workflow_commands::*;
use commands::workspace_commands::*;
use commands::workspace_task_commands::*;
use commands::skill_commands::*;
use commands::agent_provider_commands::*;
use commands::agent_provider_model_commands::*;
use commands::prompt_commands::*;
use commands::settings_commands::*;

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
        // === User commands (governance: CRUD + list) ===
        list_users,
        get_user_detail,
        create_user,
        update_user,
        delete_user,
        change_user_password,
        list_roles,
        // === Role commands (governance) ===
        list_role_details,
        create_role,
        update_role,
        delete_role,
        // === App config commands (config.ini editor) ===
        get_app_config,
        save_app_config,
        // === Menu config commands (governance: list + save) ===
        list_menu_configs,
        save_menu_config,
        save_all_menu_configs,
        // === Menu permission commands (effective drives sidebar; role/user for governance) ===
        list_effective_menu_permissions,
        list_role_menu_permissions,
        save_role_menu_permissions,
        list_user_menu_permissions,
        save_user_menu_permissions,
        // === AI Usage commands (account management + usage probing + auto-switch) ===
        ai_usage_add_account,
        ai_usage_detect_local,
        ai_usage_import_detected,
        ai_usage_capture_preview,
        ai_usage_capture_add,
        ai_usage_config_dir_preview,
        ai_usage_add_config_dir,
        ai_usage_list_accounts,
        ai_usage_update_account,
        ai_usage_delete_account,
        ai_usage_set_active,
        ai_usage_get_token,
        ai_usage_report_signal,
        ai_usage_refresh_account,
        ai_usage_refresh,
        ai_usage_get_settings,
        ai_usage_save_settings,
        ai_usage_open_terminal,
        ai_usage_open_login,
        // === Pagination config command ===
        get_pagination_config,
        // === Explorer commands (dùng bởi Git Desktop "Show in folder" và cây thư mục Terminal) ===
        explorer_open,
        explorer_read_dir,
        explorer_read_file,
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
        terminal_kill,
        // === Docker Desktop commands: trạng thái + container/image ===
        docker_available,
        docker_start_desktop,
        docker_list_containers,
        docker_list_images,
        docker_start_container,
        docker_stop_container,
        docker_restart_container,
        docker_remove_container,
        docker_remove_image,
        docker_prune_containers,
        docker_prune_images,
        docker_prune_system,
        // === Docker Desktop commands: build / compose (stream output) ===
        docker_build,
        docker_tag,
        docker_push,
        docker_compose_up,
        docker_compose_down,
        // === Docker Desktop commands: danh sách project build đã lưu (JSON cục bộ) ===
        docker_list_projects,
        // === Dev Runner commands: phát hiện + chạy lệnh phát triển ===
        detect_dev_commands,
        load_custom_commands,
        save_custom_commands,
        docker_add_project,
        docker_update_project,
        docker_remove_project,
        docker_touch_project,
        // === Workflow commands: CRUD workflow + step + layout canvas (PostgreSQL) ===
        workflow_list,
        workflow_create,
        workflow_update,
        workflow_delete,
        workflow_duplicate,
        workflow_save_layout,
        workflow_step_list,
        workflow_step_create,
        workflow_step_update,
        workflow_step_delete,
        workflow_step_reorder,
        // === AI Tasks / AI Cowork commands: tasks + task_wf_proc + task_wf_proc_step ===
        task_create,
        task_list,
        task_update,
        task_wf_proc_create,
        task_wf_proc_list,
        task_wf_proc_update,
        task_wf_proc_delete,
        task_wf_proc_step_create,
        task_wf_proc_step_list,
        task_wf_proc_step_update,
        // === Workspace commands: registry (đồng thời là tab bar), JSON cục bộ ===
        workspace_list,
        workspace_create,
        workspace_update,
        workspace_remove,
        workspace_touch,
        // === Workspace<->Task link commands (JSON cục bộ) ===
        workspace_task_list,
        workspace_task_add,
        workspace_task_remove,
        // === Skill commands: CRUD (JSON cục bộ) ===
        skill_list,
        skill_create,
        skill_update,
        skill_delete,
        // === AI Agent Provider commands: CRUD + bật/tắt (JSON cục bộ) ===
        agent_provider_list,
        agent_provider_create,
        agent_provider_update,
        agent_provider_set_enabled,
        agent_provider_delete,
        // === AI Agent Provider Model commands: CRUD + bật/tắt (PostgreSQL) ===
        agent_provider_model_list,
        agent_provider_model_list_enabled,
        agent_provider_model_create,
        agent_provider_model_update,
        agent_provider_model_set_enabled,
        agent_provider_model_delete,
        // === Prompt commands: CRUD + đếm lượt dùng (JSON cục bộ) ===
        prompt_list,
        prompt_create,
        prompt_update,
        prompt_delete,
        prompt_mark_used,
        // === Settings commands (profile + theme/language/tab_mode) ===
        get_settings,
        save_settings,
    ]
}
