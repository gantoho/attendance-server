use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use crate::state::AppState;
use crate::dto::{CreateLocationRequest, UpdateLocationRequest};
use crate::domain::Location;
use std::collections::HashMap;
use crate::error::ApiError;
use crate::service::location_service;

#[utoipa::path(
    get,
    path = "/api/v1/locations",
    responses(
        (status = 200, body = [Location]),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "locations"
)]
pub async fn get_locations(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Location>>, ApiError> {
    let admin_id = params.get("adminId").cloned();
    let list = location_service::list_locations(&state, admin_id).await?;
    Ok(Json(list))
}

#[utoipa::path(
    post,
    path = "/api/v1/locations",
    request_body = CreateLocationRequest,
    responses(
        (status = 200, body = Location),
        (status = 401, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "locations"
)]
pub async fn create_location(
    State(state): State<AppState>,
    Json(req): Json<CreateLocationRequest>,
) -> Result<Json<Location>, ApiError> {
    let loc = Location::new(req.name, req.latitude, req.longitude, req.radius, req.admin_id);
    let loc = location_service::create_location(&state, loc).await?;
    Ok(Json(loc))
}

#[utoipa::path(
    patch,
    path = "/api/v1/locations/{id}",
    request_body = UpdateLocationRequest,
    responses(
        (status = 200, body = Location),
        (status = 404, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "locations"
)]
pub async fn update_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateLocationRequest>,
) -> Result<Json<Location>, ApiError> {
    let loc = location_service::update_location(&state, id, req).await?;
    Ok(Json(loc))
}

#[utoipa::path(
    delete,
    path = "/api/v1/locations/{id}",
    responses(
        (status = 204),
        (status = 404, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "locations"
)]
pub async fn delete_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    location_service::delete_location(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
