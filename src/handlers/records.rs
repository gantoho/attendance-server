use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use crate::state::AppState;
use crate::models::{CheckInRequest, CheckInResponse, AttendanceRecord, AttendanceStatus};
use crate::repositories::{records, users, locations};
use crate::utils::geo::calculate_distance;
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
    let recs: Vec<crate::models::AttendanceRecord> = records::get_all(&state.pool).await.unwrap_or_default()
        .into_iter()
        .filter(|r| users_ids.contains(&r.user_id))
        .collect();
    Json(recs)
}

pub async fn check_in(
    State(state): State<AppState>,
    Json(req): Json<CheckInRequest>,
) -> impl IntoResponse {
    let user = match users::get_by_id(&state.pool, &req.user_id).await {
        Ok(Some(u)) => u,
        _ => return Json(CheckInResponse { success: false, record: None, message: Some("用户不存在".into()) }),
    };
    let location_id = match user.location_id {
        Some(id) => id,
        None => return Json(CheckInResponse { success: false, record: None, message: Some("用户未分配打卡位置".into()) }),
    };
    let location = match locations::get_by_id(&state.pool, &location_id).await {
        Ok(Some(l)) => l,
        _ => return Json(CheckInResponse { success: false, record: None, message: Some("打卡位置不存在".into()) }),
    };
    let distance = calculate_distance(req.latitude, req.longitude, location.latitude, location.longitude);
    if distance <= location.radius {
        let record = AttendanceRecord::new(
            req.user_id.clone(), location.id.clone(), req.latitude, req.longitude, AttendanceStatus::Success, None,
        );
        match records::save(&state.pool, &record).await {
            Ok(_) => Json(CheckInResponse { success: true, record: Some(record), message: Some("打卡成功".into()) }),
            Err(e) => Json(CheckInResponse { success: false, record: None, message: Some(format!("保存记录失败: {}", e)) }),
        }
    } else {
        let record = AttendanceRecord::new(
            req.user_id.clone(), location.id.clone(), req.latitude, req.longitude, AttendanceStatus::Failed, Some(format!("距离打卡位置 {:.2} 米，超出范围", distance)),
        );
        let _ = records::save(&state.pool, &record).await;
        Json(CheckInResponse { success: false, record: Some(record), message: Some(format!("不在打卡范围内，距离 {:.2} 米", distance)) })
    }
}

