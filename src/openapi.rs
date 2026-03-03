use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::auth::login,
        crate::api::handlers::users::get_users,
        crate::api::handlers::users::create_user,
        crate::api::handlers::users::delete_user,
        crate::api::handlers::users::update_user_location,
        crate::api::handlers::users::get_user_location,
        crate::api::handlers::locations::get_locations,
        crate::api::handlers::locations::create_location,
        crate::api::handlers::locations::update_location,
        crate::api::handlers::locations::delete_location,
        crate::api::handlers::records::get_records,
        crate::api::handlers::records::get_records_by_admin,
        crate::api::handlers::records::check_in,
        crate::api::handlers::health::health,
        crate::api::handlers::health::db_info,
        crate::api::handlers::health::stats
    ),
    components(schemas(
        crate::domain::User,
        crate::domain::UserRole,
        crate::domain::Location,
        crate::domain::AttendanceRecord,
        crate::domain::AttendanceStatus,
        crate::dto::LoginRequest,
        crate::dto::LoginResponse,
        crate::dto::CreateUserRequest,
        crate::dto::CreateLocationRequest,
        crate::dto::UpdateLocationRequest,
        crate::dto::CheckInRequest,
        crate::dto::CheckInResponse,
        crate::error::api_error::ErrorBody
    )),
    tags(
        (name = "auth", description = "authentication"),
        (name = "users", description = "user management"),
        (name = "locations", description = "location management"),
        (name = "records", description = "attendance records"),
        (name = "health", description = "health endpoints")
    )
)]
pub struct ApiDoc;

pub fn json() -> String {
    ApiDoc::openapi().to_json().unwrap()
}

pub fn write_to_file(path: &str) {
    let _ = std::fs::write(path, json());
}
