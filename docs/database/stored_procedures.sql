-- ============================================================================
-- 1DevTools — Stored Procedures (Identity & Access + Menu & Permissions)
-- Dialect:    PostgreSQL 15+
-- Idempotent: mọi function dùng CREATE OR REPLACE.
-- Yêu cầu:    chạy sau schema.sql (các bảng phải tồn tại trước).
-- ============================================================================

-- ----------------------------------------------------------------------------
-- sp_auth_find_user_by_username
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_auth_find_user_by_username
-- Find a user by username for login authentication.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_auth_find_user_by_username(
    p_username VARCHAR(100)
)
RETURNS TABLE (
    id            INTEGER,
    username      VARCHAR(100),
    password_hash TEXT,
    full_name     VARCHAR(200),
    email         VARCHAR(255),
    is_active     BOOLEAN
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        u.id,
        u.username,
        u.password_hash,
        u.full_name,
        u.email,
        u.is_active
    FROM users u
    WHERE u.username = p_username;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_auth_get_user_roles
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_auth_get_user_roles
-- Get all role names assigned to a user.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_auth_get_user_roles(
    p_user_id INTEGER
)
RETURNS TABLE (
    name VARCHAR(50)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT r.name
    FROM roles r
    INNER JOIN user_roles ur ON ur.role_id = r.id
    WHERE ur.user_id = p_user_id
    ORDER BY r.name;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_auth_reset_code_save
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_auth_reset_code_save
-- Invalidate existing codes for a user, then insert a new reset code.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_auth_reset_code_save(
    p_user_id    INTEGER,
    p_code       VARCHAR(6),
    p_expires_at TIMESTAMPTZ
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE password_reset_codes
    SET used = TRUE
    WHERE user_id = p_user_id AND used = FALSE;

    INSERT INTO password_reset_codes (user_id, code, expires_at)
    VALUES (p_user_id, p_code, p_expires_at);
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_auth_reset_code_verify
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_auth_reset_code_verify
-- Verify a reset code: must match user_id, code, not used, not expired.
-- If valid, mark as used and return the row id; otherwise return 0.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_auth_reset_code_verify(
    p_user_id INTEGER,
    p_code    VARCHAR(6)
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_id INTEGER;
BEGIN
    SELECT id INTO v_id
    FROM password_reset_codes
    WHERE user_id = p_user_id
      AND code = p_code
      AND used = FALSE
      AND expires_at > NOW()
    LIMIT 1;

    IF v_id IS NOT NULL THEN
        UPDATE password_reset_codes SET used = TRUE WHERE id = v_id;
        RETURN v_id;
    END IF;

    RETURN 0;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_auth_reset_code_has_valid
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_auth_reset_code_has_valid
-- Check if a user has any unexpired, unused reset code.
-- Returns TRUE if exists, FALSE otherwise.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_auth_reset_code_has_valid(
    p_user_id INTEGER
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1
        FROM password_reset_codes
        WHERE user_id = p_user_id
          AND used = FALSE
          AND expires_at > NOW()
    );
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_auth_reset_password
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_auth_reset_password
-- Reset password for an active user. Returns count of updated rows.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_auth_reset_password(
    p_user_id       INTEGER,
    p_password_hash TEXT
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE users
    SET password_hash = p_password_hash,
        updated_at    = NOW()
    WHERE id = p_user_id
      AND is_active = TRUE;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_select_list
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_select_list
-- List all users with their roles as a comma-separated string.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_select_list()
RETURNS TABLE (
    id            INTEGER,
    username      VARCHAR(100),
    full_name     VARCHAR(200),
    email         VARCHAR(255),
    phone         VARCHAR(50),
    "position"    VARCHAR(100),
    is_active     BOOLEAN,
    roles         TEXT,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        u.id,
        u.username,
        u.full_name,
        u.email,
        u.phone,
        u.position,
        u.is_active,
        COALESCE(
            (SELECT string_agg(r.name, ',' ORDER BY r.name)
             FROM user_roles ur
             JOIN roles r ON r.id = ur.role_id
             WHERE ur.user_id = u.id),
            ''
        ),
        to_char(u.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(u.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM users u
    ORDER BY u.id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_select_by_id
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_select_by_id
-- Find a user by ID.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_select_by_id(
    p_user_id INTEGER
)
RETURNS TABLE (
    id            INTEGER,
    username      VARCHAR(100),
    full_name     VARCHAR(200),
    email         VARCHAR(255),
    phone         VARCHAR(50),
    address       TEXT,
    "position"    VARCHAR(100),
    is_active     BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        u.id,
        u.username,
        u.full_name,
        u.email,
        u.phone,
        u.address,
        u.position,
        u.is_active,
        to_char(u.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(u.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM users u
    WHERE u.id = p_user_id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_insert
-- Insert a new user and return the created row.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_insert(
    p_username      VARCHAR(100),
    p_password_hash TEXT,
    p_full_name     VARCHAR(200),
    p_email         VARCHAR(255),
    p_phone         VARCHAR(50),
    p_address       TEXT,
    p_position      VARCHAR(100)
)
RETURNS TABLE (
    id            INTEGER,
    username      VARCHAR(100),
    full_name     VARCHAR(200),
    email         VARCHAR(255),
    phone         VARCHAR(50),
    address       TEXT,
    "position"    VARCHAR(100),
    is_active     BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO users (username, password_hash, full_name, email, phone, address, position)
    VALUES (p_username, p_password_hash, p_full_name, p_email, p_phone, p_address, p_position)
    RETURNING
        users.id,
        users.username,
        users.full_name,
        users.email,
        users.phone,
        users.address,
        users.position,
        users.is_active,
        to_char(users.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(users.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_update
-- Update user info (not password) and return updated row.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_update(
    p_user_id   INTEGER,
    p_full_name VARCHAR(200),
    p_email     VARCHAR(255),
    p_phone     VARCHAR(50),
    p_address   TEXT,
    p_position  VARCHAR(100),
    p_is_active BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    username      VARCHAR(100),
    full_name     VARCHAR(200),
    email         VARCHAR(255),
    phone         VARCHAR(50),
    address       TEXT,
    "position"    VARCHAR(100),
    is_active     BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE users
    SET full_name  = p_full_name,
        email      = p_email,
        phone      = p_phone,
        address    = p_address,
        position   = p_position,
        is_active  = p_is_active,
        updated_at = NOW()
    WHERE users.id = p_user_id
    RETURNING
        users.id,
        users.username,
        users.full_name,
        users.email,
        users.phone,
        users.address,
        users.position,
        users.is_active,
        to_char(users.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(users.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_delete
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_delete
-- Delete a user by ID. Returns the count of deleted rows.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_delete(
    p_user_id INTEGER
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM users WHERE id = p_user_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_change_password
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_change_password
-- Update the password hash for a user. Returns count of updated rows.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_change_password(
    p_user_id       INTEGER,
    p_password_hash TEXT
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE users
    SET password_hash = p_password_hash,
        updated_at    = NOW()
    WHERE id = p_user_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_username_exists
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_username_exists
-- Check if a username exists, optionally excluding a specific user ID.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_username_exists(
    p_username   VARCHAR(100),
    p_exclude_id INTEGER DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1
        FROM users u
        WHERE LOWER(u.username) = LOWER(p_username)
          AND (p_exclude_id IS NULL OR u.id <> p_exclude_id)
    );
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_user_role_sync
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_user_role_sync
-- Delete all roles for a user, then re-insert the given role names.
-- Call once per user after create/update.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_user_role_sync(
    p_user_id    INTEGER,
    p_role_names TEXT[]
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM user_roles WHERE user_id = p_user_id;

    INSERT INTO user_roles (user_id, role_id)
    SELECT p_user_id, r.id
    FROM roles r
    WHERE r.name = ANY(p_role_names);
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_role_select_list
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_role_select_list
-- List all available roles.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_role_select_list()
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(50),
    description TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT r.id, r.name, r.description
    FROM roles r
    ORDER BY r.id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_role_select_detail_list
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_role_select_detail_list
-- List all roles with metadata (description, assigned user count, created_at)
-- for the governance role management screen.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_role_select_detail_list()
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(50),
    description TEXT,
    user_count  BIGINT,
    created_at  TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        r.id,
        r.name,
        r.description,
        (SELECT COUNT(*) FROM user_roles ur WHERE ur.role_id = r.id),
        to_char(r.created_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM roles r
    ORDER BY r.id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_role_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_role_insert
-- Insert a new role and return the created row.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_role_insert(
    p_name        VARCHAR(50),
    p_description TEXT
)
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(50),
    description TEXT,
    user_count  BIGINT,
    created_at  TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO roles (name, description)
    VALUES (p_name, p_description)
    RETURNING
        roles.id,
        roles.name,
        roles.description,
        0::BIGINT,
        to_char(roles.created_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_role_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_role_update
-- Update an existing role's name and description. Returns the updated row.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_role_update(
    p_id          INTEGER,
    p_name        VARCHAR(50),
    p_description TEXT
)
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(50),
    description TEXT,
    user_count  BIGINT,
    created_at  TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE roles r
    SET name = p_name,
        description = p_description
    WHERE r.id = p_id
    RETURNING
        r.id,
        r.name,
        r.description,
        (SELECT COUNT(*) FROM user_roles ur WHERE ur.role_id = r.id),
        to_char(r.created_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_role_delete
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_role_delete
-- Delete a role by ID. Returns the count of deleted rows.
-- Deleting a role cascades to user_roles; the service layer guards against
-- removing a role that is still assigned to users.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_role_delete(
    p_id INTEGER
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM roles WHERE id = p_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_role_name_exists
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_role_name_exists
-- Check if a role name exists, optionally excluding a specific role ID.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_role_name_exists(
    p_name       VARCHAR(50),
    p_exclude_id INTEGER DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1
        FROM roles r
        WHERE LOWER(r.name) = LOWER(p_name)
          AND (p_exclude_id IS NULL OR r.id <> p_exclude_id)
    );
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_config_select_list
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_config_select_list
-- List all menu configs ordered by display_order.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_config_select_list()
RETURNS TABLE (
    key           VARCHAR(50),
    title         VARCHAR(100),
    path          VARCHAR(200),
    icon          VARCHAR(50),
    menu_group    VARCHAR(50),
    is_visible    BOOLEAN,
    display_order INTEGER
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        m.key,
        m.title,
        m.path,
        m.icon,
        m.menu_group,
        m.is_visible,
        m.display_order
    FROM menu_configs m
    ORDER BY m.display_order;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_config_upsert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_config_upsert
-- Insert or update a single menu config item.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_config_upsert(
    p_key           VARCHAR(50),
    p_title         VARCHAR(100),
    p_path          VARCHAR(200),
    p_icon          VARCHAR(50),
    p_menu_group    VARCHAR(50),
    p_is_visible    BOOLEAN,
    p_display_order INTEGER
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO menu_configs (key, title, path, icon, menu_group, is_visible, display_order)
    VALUES (p_key, p_title, p_path, p_icon, p_menu_group, p_is_visible, p_display_order)
    ON CONFLICT (key) DO UPDATE SET
        title         = EXCLUDED.title,
        path          = EXCLUDED.path,
        icon          = EXCLUDED.icon,
        menu_group    = EXCLUDED.menu_group,
        is_visible    = EXCLUDED.is_visible,
        display_order = EXCLUDED.display_order;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_config_delete_all
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_config_delete_all
-- Delete all menu configs (used before bulk re-insert on reset).
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_config_delete_all()
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM menu_configs;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_permission_effective_select
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_permission_effective_select
-- Resolve the menus a user can actually reach.
-- A per-user override wins; otherwise the union of the user's roles applies.
-- `source` tells which layer decided: 'user' or 'role'.
-- ============================================================================

-- Postgres không cho phép CREATE OR REPLACE khi cột trả về thay đổi.
DROP FUNCTION IF EXISTS sp_menu_permission_effective_select(INTEGER);

CREATE OR REPLACE FUNCTION sp_menu_permission_effective_select(
    p_user_id INTEGER
)
RETURNS TABLE (
    menu_key     VARCHAR(50),
    is_allowed   BOOLEAN,
    role_allowed BOOLEAN,
    source       VARCHAR(10)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        m.key,
        COALESCE(u.is_allowed, r.granted, FALSE),
        COALESCE(r.granted, FALSE),
        (CASE WHEN u.menu_key IS NOT NULL THEN 'user' ELSE 'role' END)::VARCHAR(10)
    FROM menu_configs m
    LEFT JOIN user_menu_permissions u
        ON u.menu_key = m.key AND u.user_id = p_user_id
    LEFT JOIN LATERAL (
        SELECT TRUE AS granted
        FROM role_menu_permissions rp
        JOIN user_roles ur ON ur.role_id = rp.role_id
        WHERE ur.user_id = p_user_id AND rp.menu_key = m.key
        LIMIT 1
    ) r ON TRUE
    ORDER BY m.display_order;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_permission_role_select
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_permission_role_select
-- List the menu keys granted to a role.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_permission_role_select(
    p_role_id INTEGER
)
RETURNS TABLE (
    menu_key VARCHAR(50)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT p.menu_key
    FROM role_menu_permissions p
    JOIN menu_configs m ON m.key = p.menu_key
    WHERE p.role_id = p_role_id
    ORDER BY m.display_order;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_permission_role_sync
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_permission_role_sync
-- Replace the whole menu grant list of a role with the given menu keys.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_permission_role_sync(
    p_role_id   INTEGER,
    p_menu_keys TEXT[]
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM role_menu_permissions WHERE role_id = p_role_id;

    INSERT INTO role_menu_permissions (role_id, menu_key)
    SELECT p_role_id, m.key
    FROM menu_configs m
    WHERE m.key = ANY(p_menu_keys);
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_permission_user_select
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_permission_user_select
-- List the per-user menu overrides (grant or revoke) of a user.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_permission_user_select(
    p_user_id INTEGER
)
RETURNS TABLE (
    menu_key   VARCHAR(50),
    is_allowed BOOLEAN
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT p.menu_key, p.is_allowed
    FROM user_menu_permissions p
    JOIN menu_configs m ON m.key = p.menu_key
    WHERE p.user_id = p_user_id
    ORDER BY m.display_order;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_menu_permission_user_sync
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_menu_permission_user_sync
-- Replace the per-user menu overrides. Keys in p_allow_keys are granted,
-- keys in p_deny_keys are revoked, every other menu falls back to the roles.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_menu_permission_user_sync(
    p_user_id    INTEGER,
    p_allow_keys TEXT[],
    p_deny_keys  TEXT[]
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM user_menu_permissions WHERE user_id = p_user_id;

    INSERT INTO user_menu_permissions (user_id, menu_key, is_allowed)
    SELECT p_user_id, m.key, TRUE
    FROM menu_configs m
    WHERE m.key = ANY(p_allow_keys);

    INSERT INTO user_menu_permissions (user_id, menu_key, is_allowed)
    SELECT p_user_id, m.key, FALSE
    FROM menu_configs m
    WHERE m.key = ANY(p_deny_keys)
      AND NOT (m.key = ANY(p_allow_keys));
END;
$$;

