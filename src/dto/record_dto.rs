use serde::{Deserialize, Serialize};
use crate::domain::AttendanceRecord;

#[derive(Debug, Deserialize)]
pub struct CheckInRequest {
    pub user_id: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize)]
pub struct CheckInResponse {
    pub success: bool,
    pub record: Option<AttendanceRecord>,
    pub message: Option<String>,
}
