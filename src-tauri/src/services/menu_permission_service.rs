//! Business logic cho module phân quyền menu.

use crate::app::result::AppResult;
use crate::database::menu_permission_store;
use crate::models::menu_permission::EffectiveMenuPermission;
use crate::services::mock_data;

pub async fn list_effective_menu_permissions(
    user_id: i32,
) -> AppResult<Vec<EffectiveMenuPermission>> {
    match menu_permission_store::list_effective(user_id).await {
        Ok(permissions) => Ok(permissions),
        // Database chưa cấu hình/kết nối được — chỉ ở debug build, trả về
        // dữ liệu mock để có thể xem layout mà không cần Postgres thật.
        Err(e) if cfg!(debug_assertions) => {
            log::warn!("Database unavailable, falling back to mock effective permissions: {e}");
            Ok(mock_data::mock_effective_permissions())
        }
        Err(e) => Err(e),
    }
}
