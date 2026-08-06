use crate::app::result::AppResult;
use crate::database::user_store;
use crate::models::user::UserSummary;

pub async fn list_users() -> AppResult<Vec<UserSummary>> {
    user_store::list_all().await
}
