use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::state::AppState;
use axum::extract::State;
use crate::repository::{users, locations, records};
use crate::error::ApiError;

#[utoipa::path(get, path = "/api/v1/health", responses((status = 200)), tag = "health")]
pub async fn health() -> Result<impl IntoResponse, ApiError> {
    Ok((StatusCode::OK, Json(serde_json::json!({"status": "ok"}))))
}

#[utoipa::path(get, path = "/api/v1/debug/dbpath", responses((status = 200)), tag = "health")]
pub async fn db_info() -> Result<impl IntoResponse, ApiError> {
    Ok((StatusCode::OK, Json(serde_json::json!({"path": "mysql"}))))
}

#[utoipa::path(get, path = "/api/v1/debug/stats", responses((status = 200)), tag = "health")]
pub async fn stats(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let users = users::count(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let locations = locations::count(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let records = records::count(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok((StatusCode::OK, Json(serde_json::json!({ "users": users, "locations": locations, "records": records }))))
}
