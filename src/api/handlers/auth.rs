use axum::{extract::State, Json};
use crate::state::AppState;
use crate::dto::{LoginRequest, LoginResponse, RefreshRequest};
use crate::domain::UserRole;
use crate::repository::users;
use crate::security::{crypto, jwt};
use crate::error::ApiError;

fn role_str(role: &UserRole) -> &'static str {
    match role {
        UserRole::Admin => "admin",
        UserRole::User => "user",
    }
}

fn token_pair(state: &AppState, user: &crate::domain::User) -> Result<(String, String), ApiError> {
    let role = role_str(&user.role);
    let access = jwt::generate_token(
        &user.id,
        role,
        &state.jwt_secret,
        state.token_exp_hours,
        "access",
    ).map_err(|e| ApiError::Internal(e.to_string()))?;
    let refresh = jwt::generate_token(
        &user.id,
        role,
        &state.jwt_secret,
        state.refresh_token_exp_hours,
        "refresh",
    ).map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok((access, refresh))
}

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
                let (access, refresh) = token_pair(&state, &user)?;
                Ok(Json(LoginResponse {
                    success: true,
                    user: Some(user),
                    message: None,
                    token: Some(access),
                    refresh_token: Some(refresh),
                    token_type: Some("bearer".into()),
                    expires_in: Some(state.token_exp_hours * 3600),
                }))
            } else {
                Err(ApiError::Unauthorized("用户名或密码错误".into()))
            }
        }
        Ok(None) => Err(ApiError::Unauthorized("用户不存在".into())),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, body = LoginResponse),
        (status = 401, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "auth"
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let claims = jwt::validate_token(&req.refresh_token, &state.jwt_secret)
        .map_err(|_| ApiError::Unauthorized("刷新令牌无效或已过期".into()))?;
    if claims.token_type != "refresh" {
        return Err(ApiError::Unauthorized("请使用刷新令牌".into()));
    }
    let user = users::get_by_id(&state.pool, &claims.sub)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::Unauthorized("用户不存在".into()))?;
    let (access, refresh) = token_pair(&state, &user)?;
    Ok(Json(LoginResponse {
        success: true,
        user: Some(user),
        message: None,
        token: Some(access),
        refresh_token: Some(refresh),
        token_type: Some("bearer".into()),
        expires_in: Some(state.token_exp_hours * 3600),
    }))
}
