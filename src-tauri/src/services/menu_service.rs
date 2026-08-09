//! Business logic cho module quản lý menu.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::menu_store;
use crate::models::menu_entity::{MenuEntity, SaveAllMenuConfigsRequest, SaveMenuConfigRequest};

/// Lấy toàn bộ cấu hình menu đang lưu.
pub async fn list_menu_configs() -> AppResult<Vec<MenuEntity>> {
    menu_store::list_all().await
}

/// Upsert một mục menu; validate `key` và `title` không được rỗng.
pub async fn save_menu_config(request: SaveMenuConfigRequest) -> AppResult<()> {
    if request.key.trim().is_empty() {
        return Err(AppError::new("Menu key không được để trống."));
    }
    if request.title.trim().is_empty() {
        return Err(AppError::new("Menu title không được để trống."));
    }

    menu_store::upsert(&request).await
}

/// Lưu toàn bộ danh sách menu trong một lần (khi sắp xếp lại): validate mọi mục
/// trước, ghi đè cả bảng rồi trả về danh sách sau khi lưu.
pub async fn save_all_menu_configs(
    request: SaveAllMenuConfigsRequest,
) -> AppResult<Vec<MenuEntity>> {
    for item in &request.items {
        if item.key.trim().is_empty() {
            return Err(AppError::new("Menu key không được để trống."));
        }
        if item.title.trim().is_empty() {
            return Err(AppError::new(format!(
                "Menu title không được để trống (key: {}).",
                item.key
            )));
        }
    }

    menu_store::save_all(&request.items).await?;
    menu_store::list_all().await
}
