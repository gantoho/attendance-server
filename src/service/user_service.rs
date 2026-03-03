use crate::state::AppState;
use crate::domain::User;
use crate::dto::CreateUserRequest;
use crate::repository::users;
use crate::security::crypto;
use crate::error::ApiError;

pub async fn list_users(state: &AppState, admin_id: Option<String>) -> Result<Vec<User>, ApiError> {
    let mut list = users::get_all(&state.pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    if let Some(aid) = admin_id {
        list = list.into_iter().filter(|u| u.admin_id.as_ref() == Some(&aid)).collect();
    }
    Ok(list)
}

pub async fn create_user(state: &AppState, req: CreateUserRequest) -> Result<User, ApiError> {
    match users::get_by_username(&state.pool, &req.username).await {
        Ok(Some(_)) => Err(ApiError::BadRequest("用户名已存在".into())),
        Ok(None) => {
            let hashed = match crypto::hash_password(&req.password) {
                Ok(h) => h,
                Err(_) => return Err(ApiError::Internal("加密失败".into())),
            };
            let user = User::new(req.username, hashed, req.role, req.admin_id);
            if let Err(e) = users::save(&state.pool, &user).await {
                Err(ApiError::Internal(e.to_string()))
            } else {
                Ok(user)
            }
        }
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}

pub async fn delete_user(state: &AppState, id: String) -> Result<(), ApiError> {
    match users::delete(&state.pool, &id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}

pub async fn update_user_location(state: &AppState, id: String, location_id: String) -> Result<User, ApiError> {
    match users::get_by_id(&state.pool, &id).await {
        Ok(Some(mut user)) => {
            user.location_id = Some(location_id);
            if let Err(e) = users::save(&state.pool, &user).await {
                Err(ApiError::Internal(e.to_string()))
            } else {
                Ok(user)
            }
        }
        Ok(None) => Err(ApiError::NotFound("用户不存在".into())),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}

pub async fn get_user_location(state: &AppState, id: String) -> Result<Option<crate::domain::Location>, ApiError> {
    let user = match users::get_by_id(&state.pool, &id).await {
        Ok(Some(u)) => u,
        _ => return Ok(None),
    };
    let loc = match user.location_id {
        Some(loc_id) => crate::repository::locations::get_by_id(&state.pool, &loc_id).await.ok().flatten(),
        None => None,
    };
    Ok(loc)
}
