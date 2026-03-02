use serde::{Deserialize, Serialize};
use crate::domain::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub user: Option<User>,
    pub message: Option<String>,
    pub token: Option<String>,
}
