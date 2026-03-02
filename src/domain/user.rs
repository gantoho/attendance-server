use serde::{Deserialize, Serialize};
use super::role::UserRole;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub admin_id: Option<String>,
    pub location_id: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn new(username: String, password: String, role: UserRole, admin_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            password,
            role,
            admin_id,
            location_id: None,
            create_time: None,
            update_time: None,
        }
    }
}
