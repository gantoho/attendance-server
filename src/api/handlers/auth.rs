use axum::{extract::State, Json};
use crate::state::AppState;
use crate::dto::{LoginRequest, LoginResponse};
use crate::repository::users;
use crate::security::{crypto, jwt};
use crate::error::ApiError;

#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, body = LoginResponse),
        (status = 401, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    match users::get_by_username(&state.pool, &req.username).await {
        Ok(Some(user)) => {
            if crypto::verify_password(&req.password, &user.password) {
                let token = jwt::generate_token(
                    &user.id,
                    match user.role { crate::domain::UserRole::Admin => "admin", crate::domain::UserRole::User => "user" },
                    &state.jwt_secret,
                    state.token_exp_hours,
                ).ok();
                Ok(Json(LoginResponse { success: true, user: Some(user), message: None, token }))
            } else {
                Err(ApiError::Unauthorized("用户名或密码错误".into()))
            }
        }
        Ok(None) => Err(ApiError::Unauthorized("用户不存在".into())),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}
