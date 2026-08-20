/// Các kiểu dữ liệu (model/DTO) chia theo domain.
#[path = "../models"]
mod models {
    /// Model cho module xác thực người dùng.
    pub mod auth;
    /// Model cho cấu hình kết nối database.
    pub mod db_config;
    /// Model thông tin hệ thống (username, IP, version).
    pub mod system;
    /// Model cho module quản lý người dùng.
    pub mod user;
    /// Model cho module quản lý role (governance).
    pub mod role;
    /// Model cho cấu hình ứng dụng (config.ini).
    pub mod app_config;
    /// Model cho module quản lý menu.
    pub mod menu_entity;
    /// Model cho phân quyền menu (hiệu lực + override role/user).
    pub mod menu_permission;
    /// Model cho màn hình Git Desktop.
    pub mod git;
    /// Model cho explorer nhanh (cây thư mục Terminal, "Show in folder").
    pub mod explorer;
    /// Model cho màn hình Docker Desktop.
    pub mod docker;
    /// Model cho module AI Usage (account AI + theo dõi usage + auto-switch).
    pub mod ai_usage;
    /// Model cho chức năng Dev Runner (phát hiện và chạy lệnh phát triển).
    pub mod dev_runner;
    /// Model cho màn hình Workflow (chuỗi step tự động hoá skill/prompt/runner/terminal).
    pub mod workflow;
    /// Model cho AI Tasks / AI Cowork (task + task_wf_proc + task_wf_proc_step).
    pub mod task;
    /// Model cho registry Workspace (mở nhiều project cùng lúc, đồng thời là tab bar).
    pub mod workspace;
    /// Model cho liên kết Workspace <-> Task (JSON cục bộ).
    pub mod workspace_task;
    /// Model cho thư viện Skill (chỉ dẫn tái sử dụng cho agent).
    pub mod skill;
    /// Model cho thư viện Rule (file markdown đính kèm, dùng làm reference cho Skill).
    pub mod rule;
    /// Model cho thư viện Prompt (snippet tái sử dụng, hỗ trợ placeholder).
    pub mod prompt;
    /// Model cho registry AI Agent Provider (các loại AI Agent được phép dùng).
    pub mod agent_provider;
    /// Model cho AI Agent Provider Model (các model của provider được phép dùng).
    pub mod agent_provider_model;
    /// Model cho Master Data (danh mục dùng chung của ứng dụng).
    pub mod master_data;
    /// Model cho module cài đặt người dùng.
    pub mod settings;
}
