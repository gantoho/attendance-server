use crate::state::AppState;
use crate::domain::Location;
use crate::dto::UpdateLocationRequest;
use crate::repository::locations;
use crate::error::ApiError;

pub async fn list_locations(state: &AppState, admin_id: Option<String>) -> Vec<Location> {
    let mut locs = locations::get_all(&state.pool).await.unwrap_or_default();
    if let Some(aid) = admin_id {
        locs = locs.into_iter().filter(|l| l.admin_id == aid).collect();
    }
    locs
}

pub async fn create_location(state: &AppState, loc: Location) -> Result<Location, ApiError> {
    match locations::save(&state.pool, &loc).await {
        Ok(_) => Ok(loc),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}

pub async fn update_location(state: &AppState, id: String, req: UpdateLocationRequest) -> Result<Location, ApiError> {
    let mut loc = match locations::get_by_id(&state.pool, &id).await {
        Ok(Some(l)) => l,
        Ok(None) => return Err(ApiError::NotFound("位置不存在".into())),
        Err(e) => return Err(ApiError::Internal(e.to_string())),
    };
    if let Some(name) = req.name { loc.name = name; }
    if let Some(lat) = req.latitude { loc.latitude = lat; }
    if let Some(lng) = req.longitude { loc.longitude = lng; }
    if let Some(r) = req.radius { loc.radius = r; }
    match locations::save(&state.pool, &loc).await {
        Ok(_) => Ok(loc),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}

pub async fn delete_location(state: &AppState, id: String) -> Result<(), ApiError> {
    match locations::delete(&state.pool, &id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::Internal(e.to_string())),
    }
}
