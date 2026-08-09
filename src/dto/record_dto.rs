use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::{AttendanceRecord, RecordType};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckInRequest {
    pub user_id: String,
    pub record_type: RecordType,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckInResponse {
    pub success: bool,
    pub record: Option<AttendanceRecord>,
    pub message: Option<String>,
}
