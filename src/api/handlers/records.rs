use axum::{extract::{Path, Query, State}, Json};
use crate::state::AppState;
use crate::dto::CheckInRequest;
use crate::repository::{records, users};
use crate::service::record_service;
use std::collections::HashMap;
use crate::error::ApiError;

#[utoipa::path(
    get,
    path = "/api/v1/records",
    responses(
        (status = 200, body = [crate::domain::AttendanceRecord]),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "records"
)]
pub async fn get_records(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<crate::domain::AttendanceRecord>>, ApiError> {
    let data = if let Some(uid) = params.get("userId") {
        records::get_by_user(&state.pool, uid).await.map_err(|e| ApiError::Internal(e.to_string()))?
    } else {
        records::get_all(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?
    };
    Ok(Json(data))
}

#[utoipa::path(
    get,
    path = "/api/v1/records/admin/{admin_id}",
    responses(
        (status = 200, body = [crate::domain::AttendanceRecord]),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "records"
)]
pub async fn get_records_by_admin(
    State(state): State<AppState>,
    Path(admin_id): Path<String>,
) -> Result<Json<Vec<crate::domain::AttendanceRecord>>, ApiError> {
    let users_ids: Vec<String> = users::get_all(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?
        .into_iter()
        .filter(|u| u.admin_id.as_ref() == Some(&admin_id))
        .map(|u| u.id)
        .collect();
    let recs: Vec<crate::domain::AttendanceRecord> = records::get_all(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?
        .into_iter()
        .filter(|r| users_ids.contains(&r.user_id))
        .collect();
    Ok(Json(recs))
}

#[utoipa::path(
    post,
    path = "/api/v1/checkin",
    request_body = CheckInRequest,
    responses(
        (status = 200, body = crate::dto::CheckInResponse),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "records"
)]
pub async fn check_in(
    State(state): State<AppState>,
    Json(req): Json<CheckInRequest>,
) -> Result<Json<crate::dto::CheckInResponse>, ApiError> {
    let resp = record_service::check_in(&state, req).await;
    Ok(Json(resp))
}
