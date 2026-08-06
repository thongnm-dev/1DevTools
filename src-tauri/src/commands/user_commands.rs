use crate::app::error::AppErrorPayload;
use crate::models::user::UserSummary;
use crate::services::user_service;

#[tauri::command]
pub async fn list_users() -> Result<Vec<UserSummary>, AppErrorPayload> {
    user_service::list_users()
        .await
        .map_err(crate::app::error::log_err)
}
