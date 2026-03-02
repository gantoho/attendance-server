use crate::state::AppState;
use crate::dto::{CheckInRequest, CheckInResponse};
use crate::domain::{AttendanceRecord, AttendanceStatus};
use crate::repository::{records, users, locations};
use crate::utils::geo::calculate_distance;

pub async fn check_in(state: &AppState, req: CheckInRequest) -> CheckInResponse {
    let user = match users::get_by_id(&state.pool, &req.user_id).await {
        Ok(Some(u)) => u,
        _ => return CheckInResponse { success: false, record: None, message: Some("用户不存在".into()) },
    };
    let location_id = match user.location_id {
        Some(id) => id,
        None => return CheckInResponse { success: false, record: None, message: Some("用户未分配打卡位置".into()) },
    };
    let location = match locations::get_by_id(&state.pool, &location_id).await {
        Ok(Some(l)) => l,
        _ => return CheckInResponse { success: false, record: None, message: Some("打卡位置不存在".into()) },
    };
    let distance = calculate_distance(req.latitude, req.longitude, location.latitude, location.longitude);
    if distance <= location.radius {
        let record = AttendanceRecord::new(
            req.user_id.clone(), location.id.clone(), req.latitude, req.longitude, AttendanceStatus::Success, None,
        );
        match records::save(&state.pool, &record).await {
            Ok(_) => CheckInResponse { success: true, record: Some(record), message: Some("打卡成功".into()) },
            Err(e) => CheckInResponse { success: false, record: None, message: Some(format!("保存记录失败: {}", e)) },
        }
    } else {
        let record = AttendanceRecord::new(
            req.user_id.clone(), location.id.clone(), req.latitude, req.longitude, AttendanceStatus::Failed, Some(format!("距离打卡位置 {:.2} 米，超出范围", distance)),
        );
        let _ = records::save(&state.pool, &record).await;
        CheckInResponse { success: false, record: Some(record), message: Some(format!("不在打卡范围内，距离 {:.2} 米", distance)) }
    }
}
