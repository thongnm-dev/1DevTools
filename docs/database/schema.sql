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
