pub mod auth;
pub mod user_dto;
pub mod location_dto;
pub mod record_dto;
pub use auth::{LoginRequest, LoginResponse};
pub use user_dto::CreateUserRequest;
pub use location_dto::{CreateLocationRequest, UpdateLocationRequest};
pub use record_dto::{CheckInRequest, CheckInResponse};
