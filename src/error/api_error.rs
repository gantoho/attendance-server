use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa::ToSchema;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Internal(String),
}

#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    error: String,
}

impl ApiError {
    fn status(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn message(self) -> String {
        match self {
            ApiError::BadRequest(m) => m,
            ApiError::Unauthorized(m) => m,
            ApiError::NotFound(m) => m,
            ApiError::Internal(m) => m,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status();
        let body = Json(ErrorBody { error: self.message() });
        (status, body).into_response()
    }
}
