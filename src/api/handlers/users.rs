use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use crate::state::AppState;
use crate::domain::User;
use crate::dto::CreateUserRequest;
use std::collections::HashMap;
use crate::error::ApiError;
use crate::service::user_service;

pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<User>> {
    let admin_id = params.get("adminId").cloned();
    Json(user_service::list_users(&state, admin_id).await)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    let user = user_service::create_user(&state, req).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    user_service::delete_user(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Deserialize)]
pub struct UpdateUserLocationPayload {
    #[serde(rename = "locationId")]
    pub location_id: String,
}

pub async fn update_user_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserLocationPayload>,
) -> Result<Json<User>, ApiError> {
    let user = user_service::update_user_location(&state, id, payload.location_id).await?;
    Ok(Json(user))
}

pub async fn get_user_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Option<crate::domain::Location>>, ApiError> {
    let loc = user_service::get_user_location(&state, id).await?;
    Ok(Json(loc))
}
