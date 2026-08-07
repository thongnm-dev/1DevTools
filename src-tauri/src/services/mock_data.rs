//! Dữ liệu mock dùng khi database chưa được cấu hình/kết nối được, để có thể
//! xem layout (login, sidebar, phân quyền) trong lúc phát triển mà không cần
//! Postgres thật. Chỉ được dùng ở debug build — xem `cfg!(debug_assertions)`
//! tại nơi gọi trong các service tương ứng.

use crate::models::auth::LoginResponse;
use crate::models::menu_entity::MenuEntity;
use crate::models::menu_permission::EffectiveMenuPermission;

/// Nhóm menu top-level (không thuộc menu_group nào), khớp `UNGROUPED` trong menu store (frontend).
const UNGROUPED: &str = "—";

pub fn mock_login_response(username: &str) -> LoginResponse {
    LoginResponse {
        user_id: 1,
        username: if username.is_empty() { "demo".to_string() } else { username.to_string() },
        full_name: "Nguyễn Minh Thông".to_string(),
        email: "thongnm@allexceed.co.jp".to_string(),
        roles: vec!["admin".to_string()],
    }
}

pub fn mock_menu_configs() -> Vec<MenuEntity> {
    vec![
        MenuEntity {
            key: "overview".into(),
            title: "Overview".into(),
            path: "/overview".into(),
            icon: "pi-home".into(),
            group: UNGROUPED.into(),
            visible: true,
            order: 1,
        },
        MenuEntity {
            key: "Git".into(),
            title: "Snippets".into(),
            path: "/git".into(),
            icon: "pi-github".into(),
            group: UNGROUPED.into(),
            visible: true,
            order: 10,
        },
        MenuEntity {
            key: "ai-chat".into(),
            title: "Chat Assistant".into(),
            path: "/ai/chat".into(),
            icon: "pi-comments".into(),
            group: UNGROUPED.into(),
            visible: true,
            order: 30,
        },
        MenuEntity {
            key: "gov-users".into(),
            title: "Users".into(),
            path: "/governance/users".into(),
            icon: "pi-users".into(),
            group: "Governance".into(),
            visible: true,
            order: 40,
        },
        MenuEntity {
            key: "gov-roles".into(),
            title: "Roles".into(),
            path: "/governance/roles".into(),
            icon: "pi-shield".into(),
            group: "Governance".into(),
            visible: true,
            order: 41,
        },
        MenuEntity {
            key: "settings".into(),
            title: "Settings".into(),
            path: "/settings".into(),
            icon: "pi-cog".into(),
            group: UNGROUPED.into(),
            visible: true,
            order: 100,
        },
    ]
}

pub fn mock_effective_permissions() -> Vec<EffectiveMenuPermission> {
    mock_menu_configs()
        .into_iter()
        .map(|m| EffectiveMenuPermission {
            menu_key: m.key,
            is_allowed: true,
            role_allowed: true,
            source: "role".into(),
        })
        .collect()
}
