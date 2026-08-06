use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::user::UserSummary;
use crate::utils::pgsql_connect;

pub async fn list_all() -> AppResult<Vec<UserSummary>> {
    let client = pgsql_connect::connect().await?;

    let rows = client
        .query("SELECT * FROM sp_user_select_list()", &[])
        .await
        .map_err(|e| AppError::new(format!("Failed to list users: {e}")))?;

    let users = rows
        .iter()
        .map(|row| {
            let roles_csv: String = row.get("roles");
            let roles: Vec<String> = if roles_csv.is_empty() {
                vec![]
            } else {
                roles_csv.split(',').map(|s| s.to_string()).collect()
            };
            UserSummary {
                id: row.get("id"),
                username: row.get("username"),
                full_name: row.get("full_name"),
                email: row.get("email"),
                phone: row.get("phone"),
                position: row.get("position"),
                is_active: row.get("is_active"),
                roles,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        })
        .collect();

    Ok(users)
}
