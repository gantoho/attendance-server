use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::state::AppState;
use axum::extract::State;
use crate::repositories::{users, locations, records};

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
}

pub async fn db_info() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"path": "mysql"})))
}

pub async fn stats(State(state): State<AppState>) -> impl IntoResponse {
    let users = users::count(&state.pool).await.unwrap_or(0);
    let locations = locations::count(&state.pool).await.unwrap_or(0);
    let records = records::count(&state.pool).await.unwrap_or(0);
    (StatusCode::OK, Json(serde_json::json!({ "users": users, "locations": locations, "records": records })))
}

