-- ============================================================================
-- 1DevTools — PostgreSQL Database Schema (Identity & Access + Menu & Permissions)
-- Dialect:    PostgreSQL 15+
-- Idempotent: dùng IF NOT EXISTS / OR REPLACE, chạy lại nhiều lần không lỗi.
--
-- Xem mô tả cột chi tiết trong `docs/database/schema.md`.
-- Sau khi chạy file này, chạy tiếp `stored_procedures.sql` rồi `seed_data.sql`.
-- ============================================================================

-- ============================================================================
-- IDENTITY & ACCESS
-- ============================================================================

CREATE TABLE IF NOT EXISTS users (
    id            SERIAL       PRIMARY KEY,
    username      VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT         NOT NULL,
    full_name     VARCHAR(200) NOT NULL DEFAULT '',
    email         VARCHAR(255) NOT NULL DEFAULT '',
    phone         VARCHAR(50)  NOT NULL DEFAULT '',
    address       TEXT         NOT NULL DEFAULT '',
    position      VARCHAR(100) NOT NULL DEFAULT '',
    is_active     BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS roles (
    id          SERIAL       PRIMARY KEY,
    name        VARCHAR(50)  NOT NULL UNIQUE,
    description TEXT         NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id     INTEGER     NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id     INTEGER     NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE IF NOT EXISTS password_reset_codes (
    id         SERIAL       PRIMARY KEY,
    user_id    INTEGER      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code       VARCHAR(6)   NOT NULL,
    expires_at TIMESTAMPTZ  NOT NULL,
    used       BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reset_codes_user ON password_reset_codes(user_id, used);

CREATE TABLE IF NOT EXISTS user_settings (
    user_id    INTEGER     PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    theme      VARCHAR(10) NOT NULL DEFAULT 'light'
        CHECK (theme IN ('light', 'dark')),
    language   VARCHAR(5)  NOT NULL DEFAULT 'vi'
        CHECK (language IN ('vi', 'en', 'ja')),
    tab_mode   BOOLEAN     NOT NULL DEFAULT false,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS master_data (
    id          SERIAL       PRIMARY KEY,
    name        VARCHAR(100) NOT NULL UNIQUE,
    keygroup       VARCHAR(50)  NOT NULL DEFAULT '',
    description TEXT         NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MENU & PERMISSIONS
-- ============================================================================

CREATE TABLE IF NOT EXISTS menu_configs (
    key           VARCHAR(50)  PRIMARY KEY,
    title         VARCHAR(100) NOT NULL,
    path          VARCHAR(200) NOT NULL,
    icon          VARCHAR(50)  NOT NULL DEFAULT 'pi-circle',
    menu_group    VARCHAR(50)  NOT NULL DEFAULT '—',
    is_visible    BOOLEAN      NOT NULL DEFAULT TRUE,
    display_order INTEGER      NOT NULL DEFAULT 0,
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Phân quyền menu theo role: có bản ghi = role được phép truy cập menu đó.
CREATE TABLE IF NOT EXISTS role_menu_permissions (
    role_id    INTEGER     NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    menu_key   VARCHAR(50) NOT NULL REFERENCES menu_configs(key) ON DELETE CASCADE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (role_id, menu_key)
);

-- Phân quyền menu riêng cho từng user — ghi đè kết quả tổng hợp từ role.
-- `is_allowed = TRUE` là cấp thêm quyền, `FALSE` là thu hồi quyền role đã cấp.
CREATE TABLE IF NOT EXISTS user_menu_permissions (
    user_id    INTEGER     NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    menu_key   VARCHAR(50) NOT NULL REFERENCES menu_configs(key) ON DELETE CASCADE,
    is_allowed BOOLEAN     NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, menu_key)
);

CREATE INDEX IF NOT EXISTS idx_role_menu_perm_menu ON role_menu_permissions(menu_key);
CREATE INDEX IF NOT EXISTS idx_user_menu_perm_menu ON user_menu_permissions(menu_key);

-- ============================================================================
-- WORKFLOW
-- ============================================================================

CREATE TABLE IF NOT EXISTS workflows (
    id          SERIAL       PRIMARY KEY,
    name        VARCHAR(200) NOT NULL,
    description TEXT         NOT NULL DEFAULT '',
    layout      JSONB        NOT NULL DEFAULT '{}',
    created_by  VARCHAR(100) NOT NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_workflows_created_by ON workflows(created_by);

-- Danh mục model AI để chọn cho từng workflow step. Hiện chỉ đối ứng provider 'claude'.
CREATE TABLE IF NOT EXISTS ai_models (
    id       SERIAL       PRIMARY KEY,
    provider VARCHAR(50)  NOT NULL DEFAULT 'claude',
    model    VARCHAR(100) NOT NULL,
    version  VARCHAR(50)  NOT NULL DEFAULT '',
    UNIQUE (provider, model, version)
);

CREATE TABLE IF NOT EXISTS workflow_steps (
    id            SERIAL       PRIMARY KEY,
    workflow_id   INTEGER      NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    name          VARCHAR(200) NOT NULL,
    step_type     VARCHAR(20)  NOT NULL DEFAULT '',
    skill_name    VARCHAR(200) NOT NULL DEFAULT '',
    description   TEXT         NOT NULL DEFAULT '',
    icon          VARCHAR(50)  NOT NULL DEFAULT 'pi pi-cog',
    step_order    INTEGER      NOT NULL DEFAULT 0,
    is_latest_step BOOLEAN     NOT NULL DEFAULT FALSE,
    model_id      INTEGER      REFERENCES ai_models(id) ON DELETE SET NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_workflow_steps_workflow ON workflow_steps(workflow_id);

CREATE TABLE IF NOT EXISTS tasks (
    id           SERIAL       PRIMARY KEY,
    task_cd      VARCHAR(100) NOT NULL,
    task_name    VARCHAR(300) NOT NULL DEFAULT '',
    category_id     VARCHAR(30)  NOT NULL DEFAULT '',
    is_complete  BOOLEAN      NOT NULL DEFAULT FALSE,
    completed_at TIMESTAMPTZ,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by   VARCHAR(100) NOT NULL,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by   VARCHAR(100) NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_tasks_task_cd ON tasks(task_cd);

CREATE TABLE IF NOT EXISTS task_wf_proc (
    id             SERIAL       PRIMARY KEY,
    task_id        INTEGER      NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    wf_id          INTEGER      NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    latest_step_id INTEGER      REFERENCES workflow_steps(id) ON DELETE SET NULL,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by     VARCHAR(100) NOT NULL,
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by     VARCHAR(100) NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_task_wf_proc_task ON task_wf_proc(task_id);

CREATE TABLE IF NOT EXISTS task_wf_proc_step (
    id              SERIAL       PRIMARY KEY,
    wf_proc_id      INTEGER      NOT NULL REFERENCES task_wf_proc(id) ON DELETE CASCADE,
    wf_step_id      INTEGER      NOT NULL REFERENCES workflow_steps(id) ON DELETE CASCADE,
    status          VARCHAR(20)  NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by      VARCHAR(100) NOT NULL,
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by      VARCHAR(100) NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_task_wf_proc_step_proc ON task_wf_proc_step(wf_proc_id);

-- ============================================================================
-- TRIGGERS — auto-update updated_at
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_users_updated_at
    BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE OR REPLACE TRIGGER trg_user_settings_updated_at
    BEFORE UPDATE ON user_settings FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE OR REPLACE TRIGGER trg_menu_configs_updated_at
    BEFORE UPDATE ON menu_configs FOR EACH ROW EXECUTE FUNCTION update_updated_at();
