//! Business logic cho module phân quyền menu.

use crate::app::result::AppResult;
use crate::database::menu_permission_store;
use crate::models::menu_permission::EffectiveMenuPermission;

pub async fn list_effective_menu_permissions(
    user_id: i32,
) -> AppResult<Vec<EffectiveMenuPermission>> {
    menu_permission_store::list_effective(user_id).await
}
