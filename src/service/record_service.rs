use crate::state::AppState;
use crate::dto::{CheckInRequest, CheckInResponse};
use crate::domain::{AttendanceRecord, AttendanceStatus, RecordType};
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

    // 每日打卡限制：先校验地理范围
    let distance = calculate_distance(req.latitude, req.longitude, location.latitude, location.longitude);
    if distance > location.radius {
        let record = AttendanceRecord::new(
            req.user_id.clone(), location.id.clone(), req.latitude, req.longitude,
            AttendanceStatus::Failed, req.record_type.clone(),
            Some(format!("距离打卡位置 {:.2} 米，超出范围", distance)),
        );
        let _ = records::save(&state.pool, &record).await;
        return CheckInResponse { success: false, record: Some(record), message: Some(format!("不在打卡范围内，距离 {:.2} 米", distance)) };
    }

    // 每日两次打卡校验（失败记录不计入次数，仅成功记录生效）
    let today = chrono::Local::now().date_naive();
    let today_in = records::find_by_user_date_type(&state.pool, &req.user_id, today, &RecordType::In).await.ok().flatten();
    match &req.record_type {
        RecordType::In => {
            if matches!(today_in.as_ref().map(|r| &r.status), Some(AttendanceStatus::Success)) {
                return CheckInResponse { success: false, record: None, message: Some("今天已打过上班卡".into()) };
            }
        }
        RecordType::Out => {
            if !matches!(today_in.as_ref().map(|r| &r.status), Some(AttendanceStatus::Success)) {
                return CheckInResponse { success: false, record: None, message: Some("请先打上班卡".into()) };
            }
            let today_out = records::find_by_user_date_type(&state.pool, &req.user_id, today, &RecordType::Out).await.ok().flatten();
            if matches!(today_out.as_ref().map(|r| &r.status), Some(AttendanceStatus::Success)) {
                return CheckInResponse { success: false, record: None, message: Some("今天已打过下班卡".into()) };
            }
        }
    }

    let record = AttendanceRecord::new(
        req.user_id.clone(), location.id.clone(), req.latitude, req.longitude,
        AttendanceStatus::Success, req.record_type.clone(), None,
    );
    let success_msg = match &record.record_type {
        RecordType::In => "上班卡打卡成功".to_string(),
        RecordType::Out => "下班卡打卡成功".to_string(),
    };
    match records::save(&state.pool, &record).await {
        Ok(_) => CheckInResponse { success: true, record: Some(record), message: Some(success_msg) },
        Err(e) => CheckInResponse { success: false, record: None, message: Some(format!("保存记录失败: {}", e)) },
    }
}
