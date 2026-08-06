use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub position: String,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
