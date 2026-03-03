use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use crate::state::AppState;
use crate::domain::User;
use crate::dto::CreateUserRequest;
use std::collections::HashMap;
use crate::error::ApiError;
use crate::service::user_service;
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/api/v1/users",
    responses(
        (status = 200, body = [User]),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "users"
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<User>>, ApiError> {
    let admin_id = params.get("adminId").cloned();
    let list = user_service::list_users(&state, admin_id).await?;
    Ok(Json(list))
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, body = User),
        (status = 400, body = crate::error::ErrorBody),
        (status = 401, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "users"
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    let user = user_service::create_user(&state, req).await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    responses(
        (status = 204),
        (status = 404, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "users"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    user_service::delete_user(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Deserialize)]
#[derive(ToSchema)]
pub struct UpdateUserLocationPayload {
    #[serde(rename = "locationId")]
    pub location_id: String,
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}/location",
    request_body = UpdateUserLocationPayload,
    responses(
        (status = 200, body = User),
        (status = 404, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "users"
)]
pub async fn update_user_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserLocationPayload>,
) -> Result<Json<User>, ApiError> {
    let user = user_service::update_user_location(&state, id, payload.location_id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/location",
    responses(
        (status = 200, body = Option<crate::domain::Location>),
        (status = 404, body = crate::error::ErrorBody),
        (status = 500, body = crate::error::ErrorBody)
    ),
    tag = "users"
)]
pub async fn get_user_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Option<crate::domain::Location>>, ApiError> {
    let loc = user_service::get_user_location(&state, id).await?;
    Ok(Json(loc))
}
