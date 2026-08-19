/// Tầng truy cập dữ liệu — đọc/ghi database và file.
#[path = "../database"]
mod database {
    /// Data access cho module xác thực người dùng.
    pub mod auth_store;
    /// Khởi tạo database (tạo bảng + stored procedure) khi app khởi động.
    pub mod startup_store;
    /// Data access cho module quản lý người dùng.
    pub mod user_store;
    /// Data access cho bảng `roles` (PostgreSQL).
    pub mod role_store;
    /// Data access cho bảng `menu_configs` (PostgreSQL).
    pub mod menu_store;
    /// Data access cho bảng `role_menu_permissions` (chỉ quyền hiệu lực).
    pub mod menu_permission_store;
    /// Lưu trữ cục bộ (JSON file) danh sách repository của màn hình Git Desktop.
    pub mod git_repo_store;
    /// Lưu trữ cục bộ (JSON file) danh sách project build của màn hình Docker Desktop.
    pub mod docker_project_store;
    /// Lưu trữ cục bộ (JSON file) danh sách account AI + settings (AI Usage).
    pub mod ai_account_store;
    /// Lưu trữ cục bộ (JSON file) token profile đã capture của account AI Usage.
    pub mod ai_profile_store;
    /// Lưu trữ cục bộ (JSON file) custom dev commands per-repo.
    pub mod dev_runner_store;
    /// Data access cho bảng `workflows` / `workflow_steps` (PostgreSQL).
    pub mod workflow_store;
    /// Data access cho bảng `tasks` / `task_wf_proc` / `task_wf_proc_step` (PostgreSQL).
    pub mod task_store;
    /// Lưu trữ cục bộ (JSON file) registry Workspace (đồng thời là tab bar).
    pub mod workspace_store;
    /// Lưu trữ cục bộ (JSON file) liên kết Workspace <-> Task.
    pub mod workspace_task_store;
    /// Lưu trữ cục bộ (JSON file) thư viện Skill.
    pub mod skill_store;
    /// Lưu trữ cục bộ (JSON file) thư viện Prompt.
    pub mod prompt_store;
    /// Data access cho bảng `agent_providers` (PostgreSQL).
    pub mod agent_provider_store;
    /// Data access cho bảng `agent_provider_models` (PostgreSQL).
    pub mod agent_provider_model_store;
    /// Data access cho bảng `master_data` (PostgreSQL).
    pub mod master_data_store;
}
