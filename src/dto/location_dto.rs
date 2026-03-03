use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateLocationRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub admin_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>,
}
