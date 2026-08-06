//! Model cho module phân quyền menu.

use serde::{Deserialize, Serialize};

/// Quyền menu của một user sau khi đã gộp role + override riêng.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectiveMenuPermission {
    pub menu_key: String,
    pub is_allowed: bool,
    /// Quyền suy ra từ các role của user, trước khi áp override riêng.
    pub role_allowed: bool,
    /// `user` nếu do override riêng quyết định, `role` nếu suy ra từ role.
    pub source: String,
}
