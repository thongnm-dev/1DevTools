//! Business logic cho module quản lý menu.

use crate::app::result::AppResult;
use crate::database::menu_store;
use crate::models::menu_entity::MenuEntity;
use crate::services::mock_data;

pub async fn list_menu_configs() -> AppResult<Vec<MenuEntity>> {
    match menu_store::list_all().await {
        Ok(configs) => Ok(configs),
        // Database chưa cấu hình/kết nối được — chỉ ở debug build, trả về
        // dữ liệu mock để có thể xem layout mà không cần Postgres thật.
        Err(e) if cfg!(debug_assertions) => {
            log::warn!("Database unavailable, falling back to mock menu configs: {e}");
            Ok(mock_data::mock_menu_configs())
        }
        Err(e) => Err(e),
    }
}
