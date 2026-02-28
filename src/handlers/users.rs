use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use crate::state::AppState;
use crate::models::{User, CreateUserRequest};
use crate::repositories::{users, locations};
use std::collections::HashMap;

pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let mut list = users::get_all(&state.pool).await.unwrap_or_default();
    if let Some(admin_id) = params.get("adminId") {
        list = list.into_iter().filter(|u| u.admin_id.as_ref() == Some(admin_id)).collect();
    }
    Json(list)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match users::get_by_username(&state.pool, &req.username).await {
        Ok(Some(_)) => (StatusCode::BAD_REQUEST, "用户名已存在".to_string()).into_response(),
        Ok(None) => {
            let user = User::new(req.username, req.password, req.role, req.admin_id);
            if let Err(e) = users::save(&state.pool, &user).await {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            } else {
                (StatusCode::OK, Json(user)).into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match users::delete(&state.pool, &id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
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
) -> impl IntoResponse {
    match users::get_by_id(&state.pool, &id).await {
        Ok(Some(mut user)) => {
            user.location_id = Some(payload.location_id);
            if let Err(e) = users::save(&state.pool, &user).await {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            } else {
                (StatusCode::OK, Json(user)).into_response()
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "用户不存在".to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_user_location(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let user = match users::get_by_id(&state.pool, &id).await {
        Ok(Some(u)) => u,
        _ => return (StatusCode::OK, Json::<Option<crate::models::Location>>(None)).into_response(),
    };
    let loc = match user.location_id {
        Some(loc_id) => locations::get_by_id(&state.pool, &loc_id).await.ok().flatten(),
        None => None,
    };
    (StatusCode::OK, Json(loc)).into_response()
}

