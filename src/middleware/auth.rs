use axum::response::{IntoResponse, Response};
use axum::extract::Request;
use axum::middleware::Next;
use crate::security::jwt::{validate_token, Claims};
use crate::error::ApiError;
use jsonwebtoken::errors::ErrorKind;
use std::env;

pub async fn require_auth(mut req: Request, next: Next) -> Response {
    let state = req.extensions().get::<crate::state::AppState>().cloned();
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .or_else(|| req.headers().get("Token"))
        .or_else(|| req.headers().get("token"))
        .and_then(|v| v.to_str().ok());
    if let Some(header) = auth_header {
        let src = if req.headers().get(axum::http::header::AUTHORIZATION).is_some() {
            "Authorization"
        } else if req.headers().get("Token").is_some() {
            "Token"
        } else if req.headers().get("token").is_some() {
            "token"
        } else {
            "<unknown>"
        };
        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .map(|s| s.trim())
            .unwrap_or_else(|| header.trim());
        let preview = {
            let l = token.len();
            if l <= 12 { "*".repeat(l) } else { format!("{}...{}", &token[..6], &token[l-6..]) }
        };
        println!("auth token source={} len={} preview={}", src, token.len(), preview);
        let jwt_secret = state.as_ref().map(|s| s.jwt_secret.clone()).or_else(|| env::var("JWT_SECRET").ok());
        if !token.is_empty() {
            match jwt_secret {
                Some(secret) => match validate_token(token, &secret) {
                    Ok(claims) => {
                        req.extensions_mut().insert::<Claims>(claims);
                        next.run(req).await
                    }
                    Err(e) => {
                        let msg = match e.kind() {
                            ErrorKind::InvalidToken => "令牌无效",
                            ErrorKind::InvalidSignature => "签名不匹配",
                            ErrorKind::ExpiredSignature => "令牌过期",
                            ErrorKind::InvalidAlgorithm => "算法不匹配",
                            ErrorKind::MissingRequiredClaim(_) => "缺少必要声明",
                            ErrorKind::InvalidIssuer => "发行者无效",
                            ErrorKind::InvalidAudience => "受众无效",
                            ErrorKind::ImmatureSignature => "令牌尚未生效",
                            ErrorKind::InvalidSubject => "主题无效",
                            _ => "鉴权失败",
                        };
                        ApiError::Unauthorized(msg.into()).into_response()
                    },
                },
                None => ApiError::Internal("服务端未配置 JWT_SECRET".into()).into_response(),
            }
        } else {
            ApiError::Unauthorized("缺少令牌".into()).into_response()
        }
    } else {
        ApiError::Unauthorized("未提供鉴权信息".into()).into_response()
    }
}
