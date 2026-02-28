use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use crate::state::AppState;
use crate::models::{CreateLocationRequest, UpdateLocationRequest, Location};
use crate::repositories::locations;
use std::collections::HashMap;

pub async fn get_locations(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let mut locs = locations::get_all(&state.pool).await.unwrap_or_default();
    if let Some(admin_id) = params.get("adminId") {
        locs = locs.into_iter().filter(|l| &l.admin_id == admin_id).collect();
    }
    Json(locs)
}

pub async fn create_location(
    State(state): State<AppState>,
    Json(req): Json<CreateLocationRequest>,
) -> impl IntoResponse {
    let loc = Location::new(req.name, req.latitude, req.longitude, req.radius, req.admin_id);
    match locations::save(&state.pool, &loc).await {
        Ok(_) => (StatusCode::OK, Json(loc)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateLocationRequest>,
) -> impl IntoResponse {
    let mut loc = match locations::get_by_id(&state.pool, &id).await {
        Ok(Some(l)) => l,
        Ok(None) => return (StatusCode::NOT_FOUND, "位置不存在".to_string()).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    if let Some(name) = req.name { loc.name = name; }
    if let Some(lat) = req.latitude { loc.latitude = lat; }
    if let Some(lng) = req.longitude { loc.longitude = lng; }
    if let Some(r) = req.radius { loc.radius = r; }
    match locations::save(&state.pool, &loc).await {
        Ok(_) => (StatusCode::OK, Json(loc)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match locations::delete(&state.pool, &id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

