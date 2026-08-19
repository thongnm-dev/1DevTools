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

-- ----------------------------------------------------------------------------
-- sp_workflow_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_insert
-- Insert a new workflow (no steps yet) and return the created row.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_insert(
    p_name        VARCHAR(200),
    p_description TEXT,
    p_icon        VARCHAR(50),
    p_created_by  VARCHAR(100)
)
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(200),
    description TEXT,
    icon        VARCHAR(50),
    layout      JSONB,
    created_by  VARCHAR(100),
    step_count  BIGINT,
    created_at  TEXT,
    updated_at  TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO workflows (name, description, icon, created_by)
    VALUES (p_name, p_description, p_icon, p_created_by)
    RETURNING
        workflows.id, workflows.name, workflows.description, workflows.icon, workflows.layout,
        workflows.created_by, 0::BIGINT,
        to_char(workflows.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(workflows.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_select_list
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_select_list
-- List all workflows owned by a user, most recently updated first, with a
-- computed step_count.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_select_list(p_created_by VARCHAR(100))
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(200),
    description TEXT,
    icon        VARCHAR(50),
    layout      JSONB,
    created_by  VARCHAR(100),
    step_count  BIGINT,
    created_at  TEXT,
    updated_at  TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        w.id, w.name, w.description, w.icon, w.layout, w.created_by,
        (SELECT COUNT(*) FROM workflow_steps s WHERE s.workflow_id = w.id),
        to_char(w.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(w.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM workflows w
    WHERE w.created_by = p_created_by
    ORDER BY w.updated_at DESC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_update
-- Update a workflow's name/description/icon (not its steps). Scoped by owner.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_update(
    p_id          INTEGER,
    p_name        VARCHAR(200),
    p_description TEXT,
    p_icon        VARCHAR(50),
    p_created_by  VARCHAR(100)
)
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(200),
    description TEXT,
    icon        VARCHAR(50),
    layout      JSONB,
    created_by  VARCHAR(100),
    step_count  BIGINT,
    created_at  TEXT,
    updated_at  TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE workflows w
    SET name = p_name, description = p_description, icon = p_icon
    WHERE w.id = p_id AND w.created_by = p_created_by
    RETURNING
        w.id, w.name, w.description, w.icon, w.layout, w.created_by,
        (SELECT COUNT(*) FROM workflow_steps s WHERE s.workflow_id = w.id),
        to_char(w.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(w.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_delete
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_delete
-- Delete a workflow by ID, scoped by owner. Cascades to workflow_steps.
-- Returns the count of deleted rows.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_delete(
    p_id         INTEGER,
    p_created_by VARCHAR(100)
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM workflows WHERE id = p_id AND created_by = p_created_by;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_update_layout
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_update_layout
-- Persist canvas node positions (keyed by step id) without touching steps.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_update_layout(
    p_id         INTEGER,
    p_layout     JSONB,
    p_created_by VARCHAR(100)
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE workflows SET layout = p_layout WHERE id = p_id AND created_by = p_created_by;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_duplicate
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_duplicate
-- Clone a workflow (name suffixed " (copy)") together with all of its steps,
-- preserving step_order. Canvas layout is intentionally not copied (the
-- frontend auto-lays-out a workflow with an empty layout).
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_duplicate(
    p_id         INTEGER,
    p_created_by VARCHAR(100)
)
RETURNS TABLE (
    id          INTEGER,
    name        VARCHAR(200),
    description TEXT,
    icon        VARCHAR(50),
    layout      JSONB,
    created_by  VARCHAR(100),
    step_count  BIGINT,
    created_at  TEXT,
    updated_at  TEXT
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_new_id INTEGER;
BEGIN
    INSERT INTO workflows (name, description, icon, created_by)
    SELECT w.name || ' (copy)', w.description, w.icon, p_created_by
    FROM workflows w
    WHERE w.id = p_id AND w.created_by = p_created_by
    RETURNING workflows.id INTO v_new_id;

    IF v_new_id IS NULL THEN
        RETURN;
    END IF;

    INSERT INTO workflow_steps (
        workflow_id, name, step_type, skill_name, prompt_id, runner_command, ai_account_id,
        description, icon, step_order, is_latest_step, model_id
    )
    SELECT v_new_id, s.name, s.step_type, s.skill_name, s.prompt_id, s.runner_command, s.ai_account_id,
        s.description, s.icon, s.step_order, s.is_latest_step, s.model_id
    FROM workflow_steps s
    WHERE s.workflow_id = p_id
    ORDER BY s.step_order;

    RETURN QUERY
    SELECT
        w.id, w.name, w.description, w.icon, w.layout, w.created_by,
        (SELECT COUNT(*) FROM workflow_steps s WHERE s.workflow_id = w.id),
        to_char(w.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(w.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM workflows w
    WHERE w.id = v_new_id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_step_select
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_step_select
-- List all steps of a workflow, ordered by step_order.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_step_select(p_workflow_id INTEGER)
RETURNS TABLE (
    id             INTEGER,
    workflow_id    INTEGER,
    name           VARCHAR(200),
    step_type      VARCHAR(20),
    skill_name     VARCHAR(200),
    prompt_id      INTEGER,
    runner_command TEXT,
    ai_account_id  INTEGER,
    description    TEXT,
    icon           VARCHAR(50),
    step_order     INTEGER,
    is_latest_step BOOLEAN,
    model_id       INTEGER,
    created_at     TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT s.id, s.workflow_id, s.name, s.step_type, s.skill_name, s.prompt_id, s.runner_command,
        s.ai_account_id, s.description, s.icon, s.step_order, s.is_latest_step, s.model_id,
        to_char(s.created_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM workflow_steps s
    WHERE s.workflow_id = p_workflow_id
    ORDER BY s.step_order ASC, s.id ASC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_step_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_step_insert
-- Insert a new step. When p_is_latest_step is set, first clears the flag on
-- every other step of the same workflow, so at most one step is ever "latest".
-- Also bumps the parent workflow's updated_at.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_step_insert(
    p_workflow_id    INTEGER,
    p_name           VARCHAR(200),
    p_step_type      VARCHAR(20),
    p_skill_name     VARCHAR(200),
    p_prompt_id      INTEGER,
    p_runner_command TEXT,
    p_ai_account_id  INTEGER,
    p_description    TEXT,
    p_icon           VARCHAR(50),
    p_step_order     INTEGER,
    p_is_latest_step BOOLEAN,
    p_model_id       INTEGER
)
RETURNS TABLE (
    id             INTEGER,
    workflow_id    INTEGER,
    name           VARCHAR(200),
    step_type      VARCHAR(20),
    skill_name     VARCHAR(200),
    prompt_id      INTEGER,
    runner_command TEXT,
    ai_account_id  INTEGER,
    description    TEXT,
    icon           VARCHAR(50),
    step_order     INTEGER,
    is_latest_step BOOLEAN,
    model_id       INTEGER,
    created_at     TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    IF p_is_latest_step THEN
        UPDATE workflow_steps SET is_latest_step = FALSE
        WHERE workflow_steps.workflow_id = p_workflow_id;
    END IF;

    RETURN QUERY
    INSERT INTO workflow_steps (
        workflow_id, name, step_type, skill_name, prompt_id, runner_command, ai_account_id,
        description, icon, step_order, is_latest_step, model_id
    )
    VALUES (
        p_workflow_id, p_name, p_step_type, p_skill_name, p_prompt_id, p_runner_command, p_ai_account_id,
        p_description, p_icon, p_step_order, p_is_latest_step, p_model_id
    )
    RETURNING
        workflow_steps.id, workflow_steps.workflow_id, workflow_steps.name, workflow_steps.step_type,
        workflow_steps.skill_name, workflow_steps.prompt_id, workflow_steps.runner_command,
        workflow_steps.ai_account_id, workflow_steps.description, workflow_steps.icon,
        workflow_steps.step_order, workflow_steps.is_latest_step, workflow_steps.model_id,
        to_char(workflow_steps.created_at, 'YYYY-MM-DD HH24:MI:SS');

    UPDATE workflows SET updated_at = NOW() WHERE workflows.id = p_workflow_id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_step_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_step_update
-- Update a step. When p_is_latest_step is set, first clears the flag on every
-- sibling step. Also bumps the parent workflow's updated_at.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_step_update(
    p_id             INTEGER,
    p_name           VARCHAR(200),
    p_step_type      VARCHAR(20),
    p_skill_name     VARCHAR(200),
    p_prompt_id      INTEGER,
    p_runner_command TEXT,
    p_ai_account_id  INTEGER,
    p_description    TEXT,
    p_icon           VARCHAR(50),
    p_step_order     INTEGER,
    p_is_latest_step BOOLEAN,
    p_model_id       INTEGER
)
RETURNS TABLE (
    id             INTEGER,
    workflow_id    INTEGER,
    name           VARCHAR(200),
    step_type      VARCHAR(20),
    skill_name     VARCHAR(200),
    prompt_id      INTEGER,
    runner_command TEXT,
    ai_account_id  INTEGER,
    description    TEXT,
    icon           VARCHAR(50),
    step_order     INTEGER,
    is_latest_step BOOLEAN,
    model_id       INTEGER,
    created_at     TEXT
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_workflow_id INTEGER;
BEGIN
    SELECT s.workflow_id INTO v_workflow_id FROM workflow_steps s WHERE s.id = p_id;

    -- Cột trong RETURNS TABLE (id, workflow_id, ...) tạo ra biến OUT trùng tên cột
    -- bảng, nên mọi tham chiếu cột ở đây PHẢI qua alias/tên bảng — nếu không
    -- Postgres báo "ambiguous" (không biết là biến OUT hay cột bảng).
    IF p_is_latest_step AND v_workflow_id IS NOT NULL THEN
        UPDATE workflow_steps SET is_latest_step = FALSE
        WHERE workflow_steps.workflow_id = v_workflow_id AND workflow_steps.id <> p_id;
    END IF;

    RETURN QUERY
    UPDATE workflow_steps s
    SET name = p_name, step_type = p_step_type, skill_name = p_skill_name, prompt_id = p_prompt_id,
        runner_command = p_runner_command, ai_account_id = p_ai_account_id, description = p_description,
        icon = p_icon, step_order = p_step_order, is_latest_step = p_is_latest_step, model_id = p_model_id
    WHERE s.id = p_id
    RETURNING s.id, s.workflow_id, s.name, s.step_type, s.skill_name, s.prompt_id, s.runner_command,
        s.ai_account_id, s.description, s.icon, s.step_order, s.is_latest_step, s.model_id,
        to_char(s.created_at, 'YYYY-MM-DD HH24:MI:SS');

    IF v_workflow_id IS NOT NULL THEN
        UPDATE workflows SET updated_at = NOW() WHERE workflows.id = v_workflow_id;
    END IF;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_step_delete
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_step_delete
-- Delete a step by ID and bump the parent workflow's updated_at.
-- Returns the count of deleted rows.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_step_delete(p_id INTEGER)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_workflow_id INTEGER;
    v_count       INTEGER;
BEGIN
    SELECT workflow_id INTO v_workflow_id FROM workflow_steps WHERE id = p_id;
    DELETE FROM workflow_steps WHERE id = p_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    IF v_workflow_id IS NOT NULL THEN
        UPDATE workflows SET updated_at = NOW() WHERE id = v_workflow_id;
    END IF;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_workflow_step_reorder
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_workflow_step_reorder
-- Reassign step_order (0-based) to match the given id order, scoped by
-- workflow. Also bumps the parent workflow's updated_at.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_workflow_step_reorder(
    p_workflow_id INTEGER,
    p_step_ids    INTEGER[]
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
DECLARE
    v_order INTEGER := 0;
    v_id    INTEGER;
BEGIN
    FOREACH v_id IN ARRAY p_step_ids LOOP
        UPDATE workflow_steps SET step_order = v_order
        WHERE id = v_id AND workflow_id = p_workflow_id;
        v_order := v_order + 1;
    END LOOP;
    UPDATE workflows SET updated_at = NOW() WHERE id = p_workflow_id;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_select_enabled
-- List only the ENABLED provider models (kèm tên provider) — dùng cho workflow
-- step "Model" picker. Cùng shape với `sp_agent_provider_model_select_list`.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_select_enabled()
RETURNS TABLE (
    id            INTEGER,
    provider_id   INTEGER,
    provider_name VARCHAR(100),
    name          VARCHAR(150),
    code          VARCHAR(150),
    version       VARCHAR(50),
    description   TEXT,
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        m.id, m.provider_id, ap.name, m.name, m.code, m.version,
        m.description, m.enabled,
        to_char(m.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(m.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM agent_provider_models m
    JOIN agent_providers ap ON ap.id = m.provider_id
    WHERE m.enabled = TRUE
    ORDER BY ap.name ASC, m.name ASC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_insert
-- Insert a new task and return the created row.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_insert(
    p_task_cd    VARCHAR(100),
    p_task_name  VARCHAR(300),
    p_category_id VARCHAR(30),
    p_created_by VARCHAR(100)
)
RETURNS TABLE (
    id           INTEGER,
    task_cd      VARCHAR(100),
    task_name    VARCHAR(300),
    category_id  VARCHAR(30),
    is_complete  BOOLEAN,
    completed_at TEXT,
    created_at   TEXT,
    created_by   VARCHAR(100),
    updated_at   TEXT,
    updated_by   VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO tasks (task_cd, task_name, category_id, created_by, updated_by)
    VALUES (p_task_cd, p_task_name, p_category_id, p_created_by, p_created_by)
    RETURNING
        tasks.id, tasks.task_cd, tasks.task_name, tasks.category_id, tasks.is_complete,
        COALESCE(to_char(tasks.completed_at, 'YYYY-MM-DD HH24:MI:SS'), ''),
        to_char(tasks.created_at, 'YYYY-MM-DD HH24:MI:SS'), tasks.created_by,
        to_char(tasks.updated_at, 'YYYY-MM-DD HH24:MI:SS'), tasks.updated_by;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_select_list
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_select_list
-- Search tasks by keyword (task_cd/task_name/category_id) and completion
-- state, joining each task's current in-progress step (if any) for display.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_select_list(
    p_keyword     TEXT DEFAULT NULL,
    p_is_complete BOOLEAN DEFAULT NULL
)
RETURNS TABLE (
    id                  INTEGER,
    task_cd             VARCHAR(100),
    task_name           VARCHAR(300),
    category_id         VARCHAR(30),
    is_complete         BOOLEAN,
    completed_at        TEXT,
    created_at          TEXT,
    created_by          VARCHAR(100),
    updated_at          TEXT,
    updated_by          VARCHAR(100),
    current_wf_name     TEXT,
    current_step_name   TEXT,
    current_step_status TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        t.id, t.task_cd, t.task_name, t.category_id, t.is_complete,
        COALESCE(to_char(t.completed_at, 'YYYY-MM-DD HH24:MI:SS'), ''),
        to_char(t.created_at, 'YYYY-MM-DD HH24:MI:SS'), t.created_by,
        to_char(t.updated_at, 'YYYY-MM-DD HH24:MI:SS'), t.updated_by,
        COALESCE(cur.wf_name, '')::TEXT, COALESCE(cur.step_name, '')::TEXT, COALESCE(cur.step_status, '')::TEXT
    FROM tasks t
    LEFT JOIN LATERAL (
        SELECT w.name AS wf_name, s.name AS step_name, ps.status AS step_status
        FROM task_wf_proc p
        JOIN task_wf_proc_step ps ON ps.wf_proc_id = p.id
        JOIN workflows w ON w.id = p.wf_id
        JOIN workflow_steps s ON s.id = ps.wf_step_id
        WHERE p.task_id = t.id AND ps.status = 'in_progress'
        ORDER BY ps.updated_at DESC
        LIMIT 1
    ) cur ON TRUE
    WHERE (p_keyword IS NULL OR p_keyword = ''
       OR t.task_cd ILIKE '%' || p_keyword || '%'
       OR t.task_name ILIKE '%' || p_keyword || '%'
       OR t.category_id ILIKE '%' || p_keyword || '%')
      AND (p_is_complete IS NULL OR t.is_complete = p_is_complete)
    ORDER BY t.created_at DESC
    LIMIT 200;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_update
-- Update a task. Auto-manages completed_at: sets it to NOW() the moment
-- is_complete flips TRUE, clears it when is_complete flips back to FALSE.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_update(
    p_id          INTEGER,
    p_task_cd     VARCHAR(100),
    p_task_name   VARCHAR(300),
    p_category_id VARCHAR(30),
    p_is_complete BOOLEAN,
    p_updated_by  VARCHAR(100)
)
RETURNS TABLE (
    id           INTEGER,
    task_cd      VARCHAR(100),
    task_name    VARCHAR(300),
    category_id  VARCHAR(30),
    is_complete  BOOLEAN,
    completed_at TEXT,
    created_at   TEXT,
    created_by   VARCHAR(100),
    updated_at   TEXT,
    updated_by   VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE tasks t
    SET task_cd = p_task_cd,
        task_name = p_task_name,
        category_id = p_category_id,
        is_complete = p_is_complete,
        completed_at = CASE
            WHEN p_is_complete AND NOT t.is_complete THEN NOW()
            WHEN NOT p_is_complete THEN NULL
            ELSE t.completed_at
        END,
        updated_by = p_updated_by
    WHERE t.id = p_id
    RETURNING
        t.id, t.task_cd, t.task_name, t.category_id, t.is_complete,
        COALESCE(to_char(t.completed_at, 'YYYY-MM-DD HH24:MI:SS'), ''),
        to_char(t.created_at, 'YYYY-MM-DD HH24:MI:SS'), t.created_by,
        to_char(t.updated_at, 'YYYY-MM-DD HH24:MI:SS'), t.updated_by;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_wf_proc_insert
-- Start tracking a task's progress through a workflow.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_wf_proc_insert(
    p_task_id    INTEGER,
    p_wf_id      INTEGER,
    p_created_by VARCHAR(100)
)
RETURNS TABLE (
    id             INTEGER,
    task_id        INTEGER,
    wf_id          INTEGER,
    latest_step_id INTEGER,
    created_at     TEXT,
    created_by     VARCHAR(100),
    updated_at     TEXT,
    updated_by     VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO task_wf_proc (task_id, wf_id, created_by, updated_by)
    VALUES (p_task_id, p_wf_id, p_created_by, p_created_by)
    RETURNING
        task_wf_proc.id, task_wf_proc.task_id, task_wf_proc.wf_id, task_wf_proc.latest_step_id,
        to_char(task_wf_proc.created_at, 'YYYY-MM-DD HH24:MI:SS'), task_wf_proc.created_by,
        to_char(task_wf_proc.updated_at, 'YYYY-MM-DD HH24:MI:SS'), task_wf_proc.updated_by;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_select_by_task
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_wf_proc_select_by_task
-- List every workflow-process a task has been attached to.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_wf_proc_select_by_task(p_task_id INTEGER)
RETURNS TABLE (
    id             INTEGER,
    task_id        INTEGER,
    wf_id          INTEGER,
    latest_step_id INTEGER,
    created_at     TEXT,
    created_by     VARCHAR(100),
    updated_at     TEXT,
    updated_by     VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT p.id, p.task_id, p.wf_id, p.latest_step_id,
        to_char(p.created_at, 'YYYY-MM-DD HH24:MI:SS'), p.created_by,
        to_char(p.updated_at, 'YYYY-MM-DD HH24:MI:SS'), p.updated_by
    FROM task_wf_proc p
    WHERE p.task_id = p_task_id
    ORDER BY p.created_at DESC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_wf_proc_update
-- Advance a task's workflow-process to a new "latest" step.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_wf_proc_update(
    p_id             INTEGER,
    p_latest_step_id INTEGER,
    p_updated_by     VARCHAR(100)
)
RETURNS TABLE (
    id             INTEGER,
    task_id        INTEGER,
    wf_id          INTEGER,
    latest_step_id INTEGER,
    created_at     TEXT,
    created_by     VARCHAR(100),
    updated_at     TEXT,
    updated_by     VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE task_wf_proc p
    SET latest_step_id = p_latest_step_id, updated_by = p_updated_by
    WHERE p.id = p_id
    RETURNING p.id, p.task_id, p.wf_id, p.latest_step_id,
        to_char(p.created_at, 'YYYY-MM-DD HH24:MI:SS'), p.created_by,
        to_char(p.updated_at, 'YYYY-MM-DD HH24:MI:SS'), p.updated_by;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_step_insert
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_wf_proc_step_insert
-- Record a task's status at a specific workflow step.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_wf_proc_step_insert(
    p_wf_proc_id INTEGER,
    p_wf_step_id INTEGER,
    p_status     VARCHAR(20),
    p_created_by VARCHAR(100)
)
RETURNS TABLE (
    id         INTEGER,
    wf_proc_id INTEGER,
    wf_step_id INTEGER,
    status     VARCHAR(20),
    created_at TEXT,
    created_by VARCHAR(100),
    updated_at TEXT,
    updated_by VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO task_wf_proc_step (wf_proc_id, wf_step_id, status, created_by, updated_by)
    VALUES (p_wf_proc_id, p_wf_step_id, p_status, p_created_by, p_created_by)
    RETURNING
        task_wf_proc_step.id, task_wf_proc_step.wf_proc_id, task_wf_proc_step.wf_step_id,
        task_wf_proc_step.status,
        to_char(task_wf_proc_step.created_at, 'YYYY-MM-DD HH24:MI:SS'), task_wf_proc_step.created_by,
        to_char(task_wf_proc_step.updated_at, 'YYYY-MM-DD HH24:MI:SS'), task_wf_proc_step.updated_by;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_step_select_by_proc
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_wf_proc_step_select_by_proc
-- List every step status recorded for a workflow-process.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_wf_proc_step_select_by_proc(p_wf_proc_id INTEGER)
RETURNS TABLE (
    id         INTEGER,
    wf_proc_id INTEGER,
    wf_step_id INTEGER,
    status     VARCHAR(20),
    created_at TEXT,
    created_by VARCHAR(100),
    updated_at TEXT,
    updated_by VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT s.id, s.wf_proc_id, s.wf_step_id, s.status,
        to_char(s.created_at, 'YYYY-MM-DD HH24:MI:SS'), s.created_by,
        to_char(s.updated_at, 'YYYY-MM-DD HH24:MI:SS'), s.updated_by
    FROM task_wf_proc_step s
    WHERE s.wf_proc_id = p_wf_proc_id
    ORDER BY s.created_at ASC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_step_update
-- ----------------------------------------------------------------------------
-- ============================================================================
-- sp_task_wf_proc_step_update
-- Update the status of a task's recorded step (pending/in_progress/
-- completed/skipped).
-- ============================================================================

-- ============================================================================
-- sp_task_wf_proc_delete
-- Delete a task workflow process by ID. Cascades to task_wf_proc_step.
-- Returns the count of deleted rows.
-- ============================================================================

CREATE OR REPLACE FUNCTION sp_task_wf_proc_delete(p_id INTEGER)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM task_wf_proc WHERE id = p_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_task_wf_proc_step_update
-- ----------------------------------------------------------------------------
-- ============================================================================
CREATE OR REPLACE FUNCTION sp_task_wf_proc_step_update(
    p_id         INTEGER,
    p_status     VARCHAR(20),
    p_updated_by VARCHAR(100)
)
RETURNS TABLE (
    id         INTEGER,
    wf_proc_id INTEGER,
    wf_step_id INTEGER,
    status     VARCHAR(20),
    created_at TEXT,
    created_by VARCHAR(100),
    updated_at TEXT,
    updated_by VARCHAR(100)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE task_wf_proc_step s
    SET status = p_status, updated_by = p_updated_by
    WHERE s.id = p_id
    RETURNING s.id, s.wf_proc_id, s.wf_step_id, s.status,
        to_char(s.created_at, 'YYYY-MM-DD HH24:MI:SS'), s.created_by,
        to_char(s.updated_at, 'YYYY-MM-DD HH24:MI:SS'), s.updated_by;
END;
$$;


-- ============================================================================
-- AI Agent Provider registry — CRUD + bật/tắt cho phép sử dụng
-- ============================================================================

-- ----------------------------------------------------------------------------
-- sp_agent_provider_select_list
-- List all providers, most-recently updated first.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_select_list()
RETURNS TABLE (
    id            INTEGER,
    name          VARCHAR(100),
    code          VARCHAR(100),
    provider_type VARCHAR(30),
    description   TEXT,
    icon          VARCHAR(100),
    command       VARCHAR(255),
    website       VARCHAR(255),
    models        TEXT[],
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.id, p.name, p.code, p.provider_type, p.description, p.icon,
        p.command, p.website, p.models, p.enabled,
        to_char(p.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(p.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM agent_providers p
    ORDER BY p.updated_at DESC, p.id DESC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_insert
-- Insert a new provider and return the created row.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_insert(
    p_name          VARCHAR(100),
    p_code          VARCHAR(100),
    p_provider_type VARCHAR(30),
    p_description   TEXT,
    p_icon          VARCHAR(100),
    p_command       VARCHAR(255),
    p_website       VARCHAR(255),
    p_models        TEXT[],
    p_enabled       BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    name          VARCHAR(100),
    code          VARCHAR(100),
    provider_type VARCHAR(30),
    description   TEXT,
    icon          VARCHAR(100),
    command       VARCHAR(255),
    website       VARCHAR(255),
    models        TEXT[],
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO agent_providers
        (name, code, provider_type, description, icon, command, website, models, enabled)
    VALUES
        (p_name, p_code, p_provider_type, p_description, p_icon, p_command, p_website,
         COALESCE(p_models, '{}'), p_enabled)
    RETURNING
        agent_providers.id, agent_providers.name, agent_providers.code,
        agent_providers.provider_type, agent_providers.description, agent_providers.icon,
        agent_providers.command, agent_providers.website, agent_providers.models,
        agent_providers.enabled,
        to_char(agent_providers.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(agent_providers.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_update
-- Update an existing provider. Returns the updated row (NULL if not found).
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_update(
    p_id            INTEGER,
    p_name          VARCHAR(100),
    p_code          VARCHAR(100),
    p_provider_type VARCHAR(30),
    p_description   TEXT,
    p_icon          VARCHAR(100),
    p_command       VARCHAR(255),
    p_website       VARCHAR(255),
    p_models        TEXT[],
    p_enabled       BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    name          VARCHAR(100),
    code          VARCHAR(100),
    provider_type VARCHAR(30),
    description   TEXT,
    icon          VARCHAR(100),
    command       VARCHAR(255),
    website       VARCHAR(255),
    models        TEXT[],
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE agent_providers p
    SET name = p_name,
        code = p_code,
        provider_type = p_provider_type,
        description = p_description,
        icon = p_icon,
        command = p_command,
        website = p_website,
        models = COALESCE(p_models, '{}'),
        enabled = p_enabled
    WHERE p.id = p_id
    RETURNING
        p.id, p.name, p.code, p.provider_type, p.description, p.icon,
        p.command, p.website, p.models, p.enabled,
        to_char(p.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(p.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_set_enabled
-- Toggle whether a provider may be used. Returns the updated row.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_set_enabled(
    p_id      INTEGER,
    p_enabled BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    name          VARCHAR(100),
    code          VARCHAR(100),
    provider_type VARCHAR(30),
    description   TEXT,
    icon          VARCHAR(100),
    command       VARCHAR(255),
    website       VARCHAR(255),
    models        TEXT[],
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE agent_providers p
    SET enabled = p_enabled
    WHERE p.id = p_id
    RETURNING
        p.id, p.name, p.code, p.provider_type, p.description, p.icon,
        p.command, p.website, p.models, p.enabled,
        to_char(p.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(p.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_delete
-- Delete a provider by ID. Returns the count of deleted rows.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_delete(
    p_id INTEGER
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM agent_providers WHERE id = p_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_code_exists
-- Check if a (non-empty) code exists, optionally excluding a specific ID.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_code_exists(
    p_code       VARCHAR(100),
    p_exclude_id INTEGER DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
BEGIN
    IF p_code IS NULL OR p_code = '' THEN
        RETURN FALSE;
    END IF;
    RETURN EXISTS (
        SELECT 1
        FROM agent_providers p
        WHERE LOWER(p.code) = LOWER(p_code)
          AND (p_exclude_id IS NULL OR p.id <> p_exclude_id)
    );
END;
$$;

-- ============================================================================
-- AI Agent Provider Model — CRUD + bật/tắt cho phép sử dụng
-- ============================================================================

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_select_list
-- List all models joined with their provider name, newest-updated first.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_select_list()
RETURNS TABLE (
    id            INTEGER,
    provider_id   INTEGER,
    provider_name VARCHAR(100),
    name          VARCHAR(150),
    code          VARCHAR(150),
    version       VARCHAR(50),
    description   TEXT,
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        m.id, m.provider_id, ap.name, m.name, m.code, m.version,
        m.description, m.enabled,
        to_char(m.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(m.updated_at, 'YYYY-MM-DD HH24:MI:SS')
    FROM agent_provider_models m
    JOIN agent_providers ap ON ap.id = m.provider_id
    ORDER BY m.updated_at DESC, m.id DESC;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_insert
-- Insert a new model and return the created row (with provider name).
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_insert(
    p_provider_id INTEGER,
    p_name        VARCHAR(150),
    p_code        VARCHAR(150),
    p_version     VARCHAR(50),
    p_description TEXT,
    p_enabled     BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    provider_id   INTEGER,
    provider_name VARCHAR(100),
    name          VARCHAR(150),
    code          VARCHAR(150),
    version       VARCHAR(50),
    description   TEXT,
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO agent_provider_models
        (provider_id, name, code, version, description, enabled)
    VALUES
        (p_provider_id, p_name, p_code, p_version, p_description, p_enabled)
    RETURNING
        agent_provider_models.id,
        agent_provider_models.provider_id,
        (SELECT ap.name FROM agent_providers ap WHERE ap.id = agent_provider_models.provider_id),
        agent_provider_models.name,
        agent_provider_models.code,
        agent_provider_models.version,
        agent_provider_models.description,
        agent_provider_models.enabled,
        to_char(agent_provider_models.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(agent_provider_models.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_update
-- Update an existing model. Returns the updated row (NULL if not found).
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_update(
    p_id          INTEGER,
    p_provider_id INTEGER,
    p_name        VARCHAR(150),
    p_code        VARCHAR(150),
    p_version     VARCHAR(50),
    p_description TEXT,
    p_enabled     BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    provider_id   INTEGER,
    provider_name VARCHAR(100),
    name          VARCHAR(150),
    code          VARCHAR(150),
    version       VARCHAR(50),
    description   TEXT,
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE agent_provider_models m
    SET provider_id = p_provider_id,
        name = p_name,
        code = p_code,
        version = p_version,
        description = p_description,
        enabled = p_enabled
    WHERE m.id = p_id
    RETURNING
        m.id, m.provider_id,
        (SELECT ap.name FROM agent_providers ap WHERE ap.id = m.provider_id),
        m.name, m.code, m.version, m.description, m.enabled,
        to_char(m.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(m.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_set_enabled
-- Toggle whether a model may be used. Returns the updated row.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_set_enabled(
    p_id      INTEGER,
    p_enabled BOOLEAN
)
RETURNS TABLE (
    id            INTEGER,
    provider_id   INTEGER,
    provider_name VARCHAR(100),
    name          VARCHAR(150),
    code          VARCHAR(150),
    version       VARCHAR(50),
    description   TEXT,
    enabled       BOOLEAN,
    created_at    TEXT,
    updated_at    TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    UPDATE agent_provider_models m
    SET enabled = p_enabled
    WHERE m.id = p_id
    RETURNING
        m.id, m.provider_id,
        (SELECT ap.name FROM agent_providers ap WHERE ap.id = m.provider_id),
        m.name, m.code, m.version, m.description, m.enabled,
        to_char(m.created_at, 'YYYY-MM-DD HH24:MI:SS'),
        to_char(m.updated_at, 'YYYY-MM-DD HH24:MI:SS');
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_delete
-- Delete a model by ID. Returns the count of deleted rows.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_delete(
    p_id INTEGER
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM agent_provider_models WHERE id = p_id;
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$;

-- ----------------------------------------------------------------------------
-- sp_agent_provider_model_code_exists
-- Check if a (non-empty) model code exists within a provider, optionally
-- excluding a specific model ID.
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION sp_agent_provider_model_code_exists(
    p_provider_id INTEGER,
    p_code        VARCHAR(150),
    p_exclude_id  INTEGER DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
BEGIN
    IF p_code IS NULL OR p_code = '' THEN
        RETURN FALSE;
    END IF;
    RETURN EXISTS (
        SELECT 1
        FROM agent_provider_models m
        WHERE m.provider_id = p_provider_id
          AND LOWER(m.code) = LOWER(p_code)
          AND (p_exclude_id IS NULL OR m.id <> p_exclude_id)
    );
END;
$$;
