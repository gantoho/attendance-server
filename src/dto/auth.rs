use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::User;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub success: bool,
    pub user: Option<User>,
    pub message: Option<String>,
    /// 访问令牌（兼容旧字段，与 access_token 相同）
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}
