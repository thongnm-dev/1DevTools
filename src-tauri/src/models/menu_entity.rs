//! Model cho module quản lý menu.

use serde::{Deserialize, Serialize};

/// Một mục menu trong hệ thống.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuEntity {
    pub key: String,
    pub title: String,
    pub path: String,
    pub icon: String,
    pub group: String,
    pub visible: bool,
    pub order: i32,
}
