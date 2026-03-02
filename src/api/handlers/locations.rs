use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use crate::state::AppState;
use crate::dto::{CreateLocationRequest, UpdateLocationRequest};
use crate::domain::Location;
use std::collections::HashMap;
use crate::error::ApiError;
use crate::service::location_service;

pub async fn get_locations(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Location>> {
    let admin_id = params.get("adminId").cloned();
    Json(location_service::list_locations(&state, admin_id).await)
}

pub async fn create_location(
    State(state): State<AppState>,
    Json(req): Json<CreateLocationRequest>,
) -> Result<Json<Location>, ApiError> {
    let loc = Location::new(req.name, req.latitude, req.longitude, req.radius, req.admin_id);
    let loc = location_service::create_location(&state, loc).await?;
    Ok(Json(loc))
}

pub async fn update_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateLocationRequest>,
) -> Result<Json<Location>, ApiError> {
    let loc = location_service::update_location(&state, id, req).await?;
    Ok(Json(loc))
}

pub async fn delete_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    location_service::delete_location(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
