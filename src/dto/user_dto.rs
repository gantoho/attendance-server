use serde::Deserialize;
use utoipa::ToSchema;
use crate::domain::UserRole;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub admin_id: Option<String>,
}
