use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use crate::state::AppState;
use crate::dto::CheckInRequest;
use crate::repository::{records, users};
use crate::service::record_service;
use std::collections::HashMap;

pub async fn get_records(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let Some(uid) = params.get("userId") {
        Json(records::get_by_user(&state.pool, uid).await.unwrap_or_default()).into_response()
    } else {
        Json(records::get_all(&state.pool).await.unwrap_or_default()).into_response()
    }
}

pub async fn get_records_by_admin(
    State(state): State<AppState>,
    Path(admin_id): Path<String>,
) -> impl IntoResponse {
    let users_ids: Vec<String> = users::get_all(&state.pool).await.unwrap_or_default()
        .into_iter()
        .filter(|u| u.admin_id.as_ref() == Some(&admin_id))
        .map(|u| u.id)
        .collect();
    let recs: Vec<crate::domain::AttendanceRecord> = records::get_all(&state.pool).await.unwrap_or_default()
        .into_iter()
        .filter(|r| users_ids.contains(&r.user_id))
        .collect();
    Json(recs)
}

pub async fn check_in(
    State(state): State<AppState>,
    Json(req): Json<CheckInRequest>,
) -> impl IntoResponse {
    Json(record_service::check_in(&state, req).await)
}
