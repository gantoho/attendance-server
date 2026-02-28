use axum::{extract::State, Json};
use crate::state::AppState;
use crate::models::{LoginRequest, LoginResponse};
use crate::repositories::users;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<LoginResponse> {
    let resp = match users::get_by_username(&state.pool, &req.username).await {
        Ok(Some(user)) => {
            if user.password == req.password {
                LoginResponse { success: true, user: Some(user), message: None }
            } else {
                LoginResponse { success: false, user: None, message: Some("密码错误".into()) }
            }
        }
        Ok(None) => LoginResponse { success: false, user: None, message: Some("用户不存在".into()) },
        Err(e) => LoginResponse { success: false, user: None, message: Some(format!("登录失败: {}", e)) },
    };
    Json(resp)
}

