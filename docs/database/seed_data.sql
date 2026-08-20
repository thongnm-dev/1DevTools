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
    ('workspaces',      'Workspaces',  '/workspaces',             'pi-th-large',   '—',          TRUE, 2),
    ('git',             'Git',         '/git',                    'pi-github',     '—',          TRUE, 3),
    ('terminal',        'Terminal',    '/terminal',               'pi-desktop',    '—',          TRUE, 4),
    ('docker',          'Docker',      '/docker',                 'pi-server',     '—',          TRUE, 5),
    ('ai-usage',        'AI Usage',    '/ai/usage',               'pi-chart-bar',  '—',          TRUE, 6),
    ('workflow',        'Workflow',    '/workflow',               'pi-sitemap',    '—',          TRUE, 7),
    ('skill',           'Skill',       '/skill',                  'pi-book',       '—',          TRUE, 8),
    ('prompt',          'Prompt',      '/prompt',                 'pi-comment',    '—',          TRUE, 9),
    ('ai-tasks',        'AI Tasks',    '/ai/tasks',               'pi-check-square', '—',        TRUE, 10),
    ('ai-cowork',       'AI Cowork',   '/ai/cowork',              'pi-objects-column', '—',      TRUE, 11),
    ('ai-providers',    'AI Providers','/governance/providers',           'pi-android',    'Governance', TRUE, 12),
    ('ai-provider-models','Provider Models','/governance/provider-models', 'pi-box',        'Governance',TRUE, 13),
    ('gov-users',       'Users',       '/governance/users',       'pi-users',      'Governance', TRUE, 14),
    ('gov-roles',       'Roles',       '/governance/roles',       'pi-shield',     'Governance', TRUE, 15),
    ('gov-menus',       'Menus',       '/governance/menus',       'pi-list',       'Governance', TRUE, 16),
    ('gov-permissions', 'Permissions', '/governance/permissions', 'pi-key',        'Governance', TRUE, 17),
    ('app-config',      'App Config',  '/governance/app-config',  'pi-sliders-h',  'Governance', TRUE, 18),
    ('master-data',     'Master Data', '/governance/master-data', 'pi-database',   'Governance', TRUE, 19)
ON CONFLICT (key) DO NOTHING;

-- ── Cấp toàn bộ menu cho role admin ────────────────────────────────────────
INSERT INTO role_menu_permissions (role_id, menu_key)
SELECT r.id, m.key
FROM roles r
CROSS JOIN menu_configs m
WHERE r.name = 'admin'
ON CONFLICT (role_id, menu_key) DO NOTHING;

-- ── AI Agent Provider mặc định (chỉ seed khi bảng còn rỗng) ─────────────────
INSERT INTO agent_providers (name, code, provider_type, icon, command, website, description, presets, model_flag, config_env)
SELECT v.name, v.code, v.provider_type, v.icon, v.command, v.website, v.description, v.presets, v.model_flag, v.config_env
FROM (VALUES
    ('Claude Code', 'claude-code', 'claude', 'pi pi-android',  'claude', 'https://claude.com/claude-code', 'Anthropic Claude Code CLI agent.',
        ARRAY['--dangerously-skip-permissions','--resume','']::text[], '--model', 'CLAUDE_CONFIG_DIR'),
    ('Codex',       'codex',       'codex',  'pi pi-code',     'codex',  'https://openai.com/codex',      'OpenAI Codex CLI agent.',
        ARRAY['--full-auto','--dangerously-bypass-approvals-and-sandbox','']::text[], '--model', 'CODEX_HOME'),
    ('Gemini CLI',  'gemini-cli',  'gemini', 'pi pi-sparkles', 'gemini', 'https://ai.google.dev',         'Google Gemini command-line agent.',
        ARRAY['']::text[], '--model', '')
) AS v(name, code, provider_type, icon, command, website, description, presets, model_flag, config_env)
WHERE NOT EXISTS (SELECT 1 FROM agent_providers);

-- Backfill cấu hình mới (presets/model_flag/config_env) cho provider đã seed từ
-- bản cũ. Chỉ chạy khi `presets` còn rỗng → không ghi đè giá trị người dùng đã
-- chỉnh (chỉnh presets qua UI khiến nó khác rỗng, lần sau sẽ bỏ qua).
UPDATE agent_providers SET
    presets    = ARRAY['--dangerously-skip-permissions','--resume','']::text[],
    model_flag = '--model',
    config_env = 'CLAUDE_CONFIG_DIR'
WHERE code = 'claude-code' AND (presets IS NULL OR presets = '{}');

UPDATE agent_providers SET
    presets    = ARRAY['--full-auto','--dangerously-bypass-approvals-and-sandbox','']::text[],
    model_flag = '--model',
    config_env = 'CODEX_HOME'
WHERE code = 'codex' AND (presets IS NULL OR presets = '{}');

UPDATE agent_providers SET
    presets    = ARRAY['']::text[],
    model_flag = '--model',
    config_env = ''
WHERE code = 'gemini-cli' AND (presets IS NULL OR presets = '{}');

-- ── Danh mục Skill (master_data, keygroup = 'SKILL_TYPE') ───────────────────
-- Thay cho danh sách category hardcode trước đây trong `src/models/skill.ts`
-- (SKILL_TYPE_META). Nhãn hiển thị vẫn lấy qua i18n `skill.category.<name>`.
INSERT INTO master_data (name, icon, keygroup, display_order, description) VALUES
    ('general',     'pi pi-star',      'SKILL_TYPE', 1,  'General-purpose skill'),
    ('frontend',    'pi pi-desktop',   'SKILL_TYPE', 2,  'Frontend development'),
    ('backend',     'pi pi-server',    'SKILL_TYPE', 3,  'Backend development'),
    ('mobile',      'pi pi-mobile',    'SKILL_TYPE', 4,  'Mobile development'),
    ('devops',      'pi pi-wrench',    'SKILL_TYPE', 5,  'DevOps / infrastructure'),
    ('translation', 'pi pi-language',  'SKILL_TYPE', 6,  'Translation'),
    ('design',      'pi pi-palette',   'SKILL_TYPE', 7,  'Design'),
    ('writing',     'pi pi-file-edit', 'SKILL_TYPE', 8,  'Writing'),
    ('data',        'pi pi-chart-bar', 'SKILL_TYPE', 9,  'Data'),
    ('custom',      'pi pi-cog',       'SKILL_TYPE', 10, 'Custom / other')
ON CONFLICT (name) DO NOTHING;

-- ── Model mặc định cho provider đã seed (chỉ seed khi bảng còn rỗng) ─────────
INSERT INTO agent_provider_models (provider_id, name, code, version, description)
SELECT ap.id, v.name, v.code, v.version, v.description
FROM (VALUES
    ('claude-code', 'Claude Opus 4.8',   'claude-opus-4-8',   '4.8', 'Most capable Claude model.'),
    ('claude-code', 'Claude Sonnet 4.6', 'claude-sonnet-4-6', '4.6', 'Balanced Claude model.'),
    ('claude-code', 'Claude Haiku 4.5',  'claude-haiku-4-5',  '4.5', 'Fast, lightweight Claude model.'),
    ('codex',       'GPT-5 Codex',       'gpt-5-codex',       '',    'OpenAI Codex model.'),
    ('gemini-cli',  'Gemini 2.5 Pro',    'gemini-2.5-pro',    '2.5', 'Google Gemini Pro model.')
) AS v(provider_code, name, code, version, description)
JOIN agent_providers ap ON ap.code = v.provider_code
WHERE NOT EXISTS (SELECT 1 FROM agent_provider_models);
