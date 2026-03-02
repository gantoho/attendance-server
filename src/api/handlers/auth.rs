use axum::{extract::State, Json};
use crate::state::AppState;
use crate::dto::{LoginRequest, LoginResponse};
use crate::repository::users;
use crate::security::{crypto, jwt};
use crate::error::ApiError;

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
                Err(ApiError::Unauthorized)
            }
        }
        Ok(None) => Err(ApiError::Unauthorized),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}
