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
