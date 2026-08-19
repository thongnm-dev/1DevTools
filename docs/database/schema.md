# Database Schema

Dialect: **PostgreSQL 15+**. DDL idempotent (`IF NOT EXISTS` / `OR REPLACE`), chạy lại nhiều lần không lỗi.

Tài liệu này mô tả nhóm bảng **Identity & Access** và **Menu & Permissions** — nguồn dữ liệu cho các màn hình Governance (Users / Roles / Menus / Permissions) và luồng đăng nhập / phân quyền sidebar.

> Các bảng `users`, `user_settings`, `menu_configs` tự cập nhật cột `updated_at` qua trigger `update_updated_at()` (BEFORE UPDATE FOR EACH ROW).

## Quan hệ tổng quan

```
users ─┬─< user_roles >─ roles
       ├─< password_reset_codes
       ├─── user_settings (1–1)
       └─< user_menu_permissions >─ menu_configs
                                    │
roles ──────< role_menu_permissions >┘
```

Quyền menu hiệu lực của một user = hợp quyền từ các role của user (`role_menu_permissions`), sau đó áp override riêng ở cấp user (`user_menu_permissions`, `is_allowed = FALSE` là thu hồi).

---

## Identity & Access

### `users`

Tài khoản người dùng.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `username` | `VARCHAR(100)` | NOT NULL, UNIQUE | Tên đăng nhập. |
| `password_hash` | `TEXT` | NOT NULL | Mật khẩu đã hash (bcrypt). |
| `full_name` | `VARCHAR(200)` | NOT NULL, DEFAULT `''` | Họ tên. |
| `email` | `VARCHAR(255)` | NOT NULL, DEFAULT `''` | Email. |
| `phone` | `VARCHAR(50)` | NOT NULL, DEFAULT `''` | Số điện thoại. |
| `address` | `TEXT` | NOT NULL, DEFAULT `''` | Địa chỉ. |
| `position` | `VARCHAR(100)` | NOT NULL, DEFAULT `''` | Chức danh. |
| `is_active` | `BOOLEAN` | NOT NULL, DEFAULT `TRUE` | Còn hoạt động không. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |

### `roles`

Vai trò/nhóm quyền.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `name` | `VARCHAR(50)` | NOT NULL, UNIQUE | Tên role. |
| `description` | `TEXT` | NOT NULL, DEFAULT `''` | Mô tả. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |

### `user_roles`

Bảng nối user ↔ role (nhiều–nhiều). Được `sp_user_role_sync` / `sp_auth_get_user_roles` sử dụng.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `user_id` | `INTEGER` | NOT NULL, FK → `users(id)` ON DELETE CASCADE | User. |
| `role_id` | `INTEGER` | NOT NULL, FK → `roles(id)` ON DELETE CASCADE | Role. |
| `assigned_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm gán. |
| | | PRIMARY KEY (`user_id`, `role_id`) | Mỗi cặp user–role là duy nhất. |

### `password_reset_codes`

Mã reset mật khẩu gửi qua email.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `user_id` | `INTEGER` | NOT NULL, FK → `users(id)` ON DELETE CASCADE | User yêu cầu reset. |
| `code` | `VARCHAR(6)` | NOT NULL | Mã 6 ký tự. |
| `expires_at` | `TIMESTAMPTZ` | NOT NULL | Thời điểm hết hạn. |
| `used` | `BOOLEAN` | NOT NULL, DEFAULT `FALSE` | Đã dùng chưa. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |

Index: `idx_reset_codes_user` trên (`user_id`, `used`).

### `user_settings`

Tùy chọn cá nhân của user (1–1 với `users`).

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `user_id` | `INTEGER` | PRIMARY KEY, FK → `users(id)` ON DELETE CASCADE | User (1–1). |
| `theme` | `VARCHAR(10)` | NOT NULL, DEFAULT `'light'`, CHECK IN (`light`, `dark`) | Giao diện sáng/tối. |
| `language` | `VARCHAR(5)` | NOT NULL, DEFAULT `'vi'`, CHECK IN (`vi`, `en`, `ja`) | Ngôn ngữ. |
| `tab_mode` | `BOOLEAN` | NOT NULL, DEFAULT `false` | Bật chế độ tab. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |

---

## Menu & Permissions

### `menu_configs`

Cấu hình các mục menu trên sidebar. Khóa chính là `key` dạng chuỗi (không phải id số).

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `key` | `VARCHAR(50)` | PRIMARY KEY | Khóa định danh menu (vd `gov-users`). |
| `title` | `VARCHAR(100)` | NOT NULL | Nhãn hiển thị. |
| `path` | `VARCHAR(200)` | NOT NULL | Đường dẫn route (vd `/governance/users`). |
| `icon` | `VARCHAR(50)` | NOT NULL, DEFAULT `'pi-circle'` | Icon PrimeIcons. |
| `menu_group` | `VARCHAR(50)` | NOT NULL, DEFAULT `'—'` | Nhóm menu (`—` = không nhóm). |
| `is_visible` | `BOOLEAN` | NOT NULL, DEFAULT `TRUE` | Có hiển thị không. |
| `display_order` | `INTEGER` | NOT NULL, DEFAULT `0` | Thứ tự sắp xếp. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |

### `role_menu_permissions`

Phân quyền menu theo role: **có bản ghi = role được phép truy cập menu đó**.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `role_id` | `INTEGER` | NOT NULL, FK → `roles(id)` ON DELETE CASCADE | Role. |
| `menu_key` | `VARCHAR(50)` | NOT NULL, FK → `menu_configs(key)` ON DELETE CASCADE | Menu. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật. |
| | | PRIMARY KEY (`role_id`, `menu_key`) | Mỗi cặp role–menu là duy nhất. |

Index: `idx_role_menu_perm_menu` trên (`menu_key`).

### `user_menu_permissions`

Override quyền menu riêng cho từng user — ghi đè kết quả tổng hợp từ role. `is_allowed = TRUE` là cấp thêm quyền, `FALSE` là thu hồi quyền role đã cấp.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `user_id` | `INTEGER` | NOT NULL, FK → `users(id)` ON DELETE CASCADE | User. |
| `menu_key` | `VARCHAR(50)` | NOT NULL, FK → `menu_configs(key)` ON DELETE CASCADE | Menu. |
| `is_allowed` | `BOOLEAN` | NOT NULL, DEFAULT `TRUE` | `TRUE` = cấp thêm, `FALSE` = thu hồi. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật. |
| | | PRIMARY KEY (`user_id`, `menu_key`) | Mỗi cặp user–menu là duy nhất. |

Index: `idx_user_menu_perm_menu` trên (`menu_key`).

---

## Workflow & Tasks

Nhóm bảng cho màn hình **Workflow** (chuỗi step tái sử dụng, hiển thị dạng canvas) và tính năng **AI Tasks / AI Cowork** (theo dõi một task đi qua từng step của workflow, mở terminal chạy `claude` cho step dạng `skill`).

```
workflows ─┬─< workflow_steps
           │        │
           │        └─< task_wf_proc_step >─┐
           │                                │
tasks ─────┴─< task_wf_proc >───────────────┘
```

### `workflows`

Thư viện workflow — không gắn cố định vào 1 workspace, chọn workspace/task đích tại thời điểm chạy.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `name` | `VARCHAR(200)` | NOT NULL | Tên workflow. |
| `description` | `TEXT` | NOT NULL, DEFAULT `''` | Mô tả. |
| `icon` | `VARCHAR(50)` | NOT NULL, DEFAULT `'pi pi-sitemap'` | Icon PrimeIcons hiển thị ở sidebar/canvas. |
| `layout` | `JSONB` | NOT NULL, DEFAULT `'{}'` | Vị trí node trên canvas, dạng `{ "<step_id>": {"x":.., "y":..} }`. |
| `created_by` | `VARCHAR(100)` | NOT NULL | Username chủ sở hữu (mỗi user chỉ thấy workflow của mình). |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |

Index: `idx_workflows_created_by` trên (`created_by`).

> **Lưu ý:** Bảng `ai_models` cũ đã bị loại bỏ. Danh mục model cho workflow
> step "Model" picker giờ lấy từ `agent_provider_models` (chỉ model `enabled`),
> qua `sp_agent_provider_model_select_enabled`. Xem `agent_providers` / `agent_provider_models`.

### `workflow_steps`

Một step trong workflow, theo thứ tự `step_order`.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `workflow_id` | `INTEGER` | NOT NULL, FK → `workflows(id)` ON DELETE CASCADE | Workflow cha. |
| `name` | `VARCHAR(200)` | NOT NULL | Tên step. |
| `step_type` | `VARCHAR(20)` | NOT NULL, DEFAULT `''` | `skill` \| `prompt` \| `runner` \| `terminal` \| `custom`. |
| `skill_name` | `VARCHAR(200)` | NOT NULL, DEFAULT `''` | Slug skill dùng khi `step_type = 'skill'` (vd `create-plan`), gõ tự do — không đối chiếu thư mục `.claude/skills`. |
| `prompt_id` | `INTEGER` | NULLABLE | Id prompt (thư viện Prompt cục bộ) khi `step_type = 'prompt'`. |
| `runner_command` | `TEXT` | NOT NULL, DEFAULT `''` | Lệnh literal khi `step_type = 'runner'` hoặc `'terminal'`. |
| `ai_account_id` | `INTEGER` | NULLABLE | Account AI (thư viện AI Usage cục bộ) cần active trước khi chạy step. |
| `description` | `TEXT` | NOT NULL, DEFAULT `''` | Mô tả. |
| `icon` | `VARCHAR(50)` | NOT NULL, DEFAULT `'pi pi-cog'` | Icon PrimeIcons. |
| `step_order` | `INTEGER` | NOT NULL, DEFAULT `0` | Thứ tự hiển thị/chạy. |
| `is_latest_step` | `BOOLEAN` | NOT NULL, DEFAULT `FALSE` | Đánh dấu step cuối cùng của workflow (chỉ 1 step/workflow — service tự gỡ cờ ở step khác khi set). |
| `model_id` | `INTEGER` | FK → `agent_provider_models(id)` ON DELETE SET NULL | Model AI dùng khi AI Cowork chạy step `skill` này. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |

Index: `idx_workflow_steps_workflow` trên (`workflow_id`).

### `tasks`

Một hạng mục công việc (task) có thể được xử lý qua nhiều workflow.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `task_cd` | `VARCHAR(100)` | NOT NULL | Mã task (vd `SZTN_G_SC01`). |
| `task_name` | `VARCHAR(300)` | NOT NULL, DEFAULT `''` | Tên task. |
| `category_id` | `VARCHAR(30)` | NOT NULL, DEFAULT `''` | Loại task (`screen` \| `batch` \| `part` \| `other`). |
| `is_complete` | `BOOLEAN` | NOT NULL, DEFAULT `FALSE` | Đã hoàn thành chưa. |
| `completed_at` | `TIMESTAMPTZ` | NULLABLE | Thời điểm hoàn thành (auto set khi `is_complete` chuyển `TRUE`). |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |
| `created_by` | `VARCHAR(100)` | NOT NULL | Username tạo. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |
| `updated_by` | `VARCHAR(100)` | NOT NULL, DEFAULT `''` | Username cập nhật gần nhất. |

Index: `idx_tasks_task_cd` trên (`task_cd`).

### `task_wf_proc`

Một lượt "chạy" workflow cho 1 task — 1 bản ghi cho mỗi cặp (`task_id`, `wf_id`).

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `task_id` | `INTEGER` | NOT NULL, FK → `tasks(id)` ON DELETE CASCADE | Task. |
| `wf_id` | `INTEGER` | NOT NULL, FK → `workflows(id)` ON DELETE CASCADE | Workflow đang áp dụng. |
| `latest_step_id` | `INTEGER` | FK → `workflow_steps(id)` ON DELETE SET NULL | Step gần nhất task đã chạy tới. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |
| `created_by` | `VARCHAR(100)` | NOT NULL | Username tạo. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |
| `updated_by` | `VARCHAR(100)` | NOT NULL, DEFAULT `''` | Username cập nhật gần nhất. |

Index: `idx_task_wf_proc_task` trên (`task_id`).

### `task_wf_proc_step`

Trạng thái của task tại từng step cụ thể trong 1 lượt chạy workflow.

| Column | Type | Constraints | Mô tả |
|---|---|---|---|
| `id` | `SERIAL` | PRIMARY KEY | Khóa chính tự tăng. |
| `wf_proc_id` | `INTEGER` | NOT NULL, FK → `task_wf_proc(id)` ON DELETE CASCADE | Lượt chạy workflow. |
| `wf_step_id` | `INTEGER` | NOT NULL, FK → `workflow_steps(id)` ON DELETE CASCADE | Step. |
| `status` | `VARCHAR(20)` | NOT NULL, DEFAULT `''` | `pending` \| `in_progress` \| `completed` \| `skipped`. |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm tạo. |
| `created_by` | `VARCHAR(100)` | NOT NULL | Username tạo. |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Thời điểm cập nhật (auto qua trigger). |
| `updated_by` | `VARCHAR(100)` | NOT NULL, DEFAULT `''` | Username cập nhật gần nhất. |

Index: `idx_task_wf_proc_step_proc` trên (`wf_proc_id`).
