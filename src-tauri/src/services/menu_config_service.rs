//! Business logic cho module quản lý menu.

use crate::app::result::AppResult;
use crate::database::menu_config_store;
use crate::models::menu_config::MenuConfig;

pub async fn list_menu_configs() -> AppResult<Vec<MenuConfig>> {
    menu_config_store::list_all().await
}
