use axum::{http::StatusCode, response::Response};
use axum::extract::Request;
use axum::middleware::Next;
use crate::security::jwt::{validate_token, Claims};

pub async fn require_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let state = req.extensions().get::<crate::state::AppState>().cloned();
    let auth_header = req.headers().get(axum::http::header::AUTHORIZATION).and_then(|v| v.to_str().ok());
        if let (Some(state), Some(header)) = (state, auth_header) {
        let token_opt = header.strip_prefix("Bearer ");
        if let Some(token) = token_opt {
            match validate_token(token, &state.jwt_secret) {
                Ok(claims) => {
                    req.extensions_mut().insert::<Claims>(claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
