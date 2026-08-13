-- ============================================================================
-- 1DevTools — Seed Data
-- Idempotent: dùng ON CONFLICT DO NOTHING, chạy lại nhiều lần không lỗi.
-- Yêu cầu:    chạy sau schema.sql.
--
-- Menu seed khớp với route thật của app (xem `src/app/router/routes.ts` và
-- `src-tauri/src/services/mock_data.rs`). Sidebar/route lấy hoàn toàn từ DB nên
-- PHẢI seed menu_configs + role_menu_permissions, nếu không admin đăng nhập sẽ
-- thấy sidebar rỗng.
-- ============================================================================

-- ── Roles mặc định ─────────────────────────────────────────────────────────
INSERT INTO roles (name, description) VALUES
    ('admin',  'Administrator with full access'),
    ('member', 'Regular team member'),
    ('viewer', 'Read-only access')
ON CONFLICT (name) DO NOTHING;

-- ── User mặc định (password mọi user: ad@123456) ───────────────────────────
-- `admin`  : tài khoản quản trị chung.
-- `thongnm`: tài khoản mẫu (khớp dữ liệu mock cũ — Nguyễn Minh Thông).
INSERT INTO users (username, password_hash, full_name, email, position)
VALUES
    ('admin',
     '$2b$12$dTyhgIqskYwXMkSfe6Luyuq0Ve7EMFS7Rrq7Z5eXvx7apv0bk9cOy',
     'Administrator', '', 'Admin'),
    ('thongnm',
     '$2b$12$dTyhgIqskYwXMkSfe6Luyuq0Ve7EMFS7Rrq7Z5eXvx7apv0bk9cOy',
     'Nguyễn Minh Thông', 'thongnm@allexceed.co.jp', 'Developer')
ON CONFLICT (username) DO NOTHING;

-- Gán role admin cho cả hai user.
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u, roles r
WHERE u.username IN ('admin', 'thongnm') AND r.name = 'admin'
ON CONFLICT (user_id, role_id) DO NOTHING;

-- ── Menu configuration (khớp route của app) ────────────────────────────────
INSERT INTO menu_configs (key, title, path, icon, menu_group, is_visible, display_order) VALUES
    ('overview',        'Overview',    '/overview',               'pi-home',       '—',          TRUE, 1),
    ('workspaces',      'Workspaces',  '/workspaces',             'pi-th-large',   '—',          TRUE, 5),
    ('git',             'Git',         '/git',                    'pi-github',     '—',          TRUE, 10),
    ('terminal',        'Terminal',    '/terminal',               'pi-desktop',    '—',          TRUE, 11),
    ('docker',          'Docker',      '/docker',                 'pi-server',     '—',          TRUE, 12),
    ('ai-usage',        'AI Usage',    '/ai/usage',               'pi-chart-bar',  '—',          TRUE, 31),
    ('workflow',        'Workflow',    '/workflow',               'pi-sitemap',    '—',          TRUE, 32),
    ('gov-users',       'Users',       '/governance/users',       'pi-users',      'Governance', TRUE, 40),
    ('gov-roles',       'Roles',       '/governance/roles',       'pi-shield',     'Governance', TRUE, 41),
    ('gov-menus',       'Menus',       '/governance/menus',       'pi-list',       'Governance', TRUE, 42),
    ('gov-permissions', 'Permissions', '/governance/permissions', 'pi-key',        'Governance', TRUE, 43),
    ('app-config',      'App Config',  '/governance/app-config',  'pi-sliders-h',  'Governance', TRUE, 44),
    ('settings',        'Settings',    '/settings',               'pi-cog',        '—',          TRUE, 100)
ON CONFLICT (key) DO NOTHING;

-- ── Cấp toàn bộ menu cho role admin ────────────────────────────────────────
INSERT INTO role_menu_permissions (role_id, menu_key)
SELECT r.id, m.key
FROM roles r
CROSS JOIN menu_configs m
WHERE r.name = 'admin'
ON CONFLICT (role_id, menu_key) DO NOTHING;
