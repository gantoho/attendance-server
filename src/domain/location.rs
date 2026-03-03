use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub admin_id: String,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl Location {
    pub fn new(name: String, latitude: f64, longitude: f64, radius: f64, admin_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            latitude,
            longitude,
            radius,
            admin_id,
            create_time: None,
            update_time: None,
        }
    }
}
