use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use super::status::AttendanceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceRecord {
    pub id: String,
    pub user_id: String,
    pub location_id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
    pub status: AttendanceStatus,
    pub error_message: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl AttendanceRecord {
    pub fn new(
        user_id: String,
        location_id: String,
        latitude: f64,
        longitude: f64,
        status: AttendanceStatus,
        error_message: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            location_id,
            latitude,
            longitude,
            timestamp: chrono::Utc::now().timestamp(),
            status,
            error_message,
            create_time: None,
            update_time: None,
        }
    }
}
