//! Model cho quản lý Master Data — danh mục dùng chung của ứng dụng (loại,
//! nhóm, tuỳ chọn tĩnh...). Lưu ở bảng `master_data` (PostgreSQL), truy cập qua
//! stored procedure `sp_master_data_*`.

use serde::{Deserialize, Serialize};

fn default_string() -> String {
    String::new()
}

/// Một mục danh mục (master data) được đăng ký trong hệ thống.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MasterData {
    pub id: i32,
    /// Tên hiển thị (duy nhất).
    pub name: String,
    /// Icon (lớp PrimeIcons, VD: "pi pi-tag").
    #[serde(default = "default_string")]
    pub icon: String,
    /// Nhóm phân loại danh mục (VD: "category", "status").
    #[serde(default = "default_string")]
    pub keygroup: String,
    /// Thứ tự hiển thị trong nhóm.
    #[serde(default)]
    pub display_order: i32,
    /// Mô tả ngắn.
    #[serde(default = "default_string")]
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request đăng ký / cập nhật một mục danh mục.
#[derive(Debug, Deserialize)]
pub struct MasterDataRequest {
    pub name: String,
    #[serde(default = "default_string")]
    pub icon: String,
    #[serde(default = "default_string")]
    pub keygroup: String,
    #[serde(default)]
    pub display_order: i32,
    #[serde(default = "default_string")]
    pub description: String,
}
