use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use crate::api::handlers as handlers;
use crate::state::AppState;
use crate::middleware;
use axum::middleware::from_fn;

pub fn build_router() -> Router<AppState> {
    let public = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/debug/dbpath", get(handlers::health::db_info))
        .route("/debug/stats", get(handlers::health::stats))
        .route("/login", post(handlers::auth::login));

    let private = Router::new()
        .route("/users", get(handlers::users::get_users).post(handlers::users::create_user))
        .route("/users/:id", delete(handlers::users::delete_user))
        .route(
            "/users/:id/location",
            get(handlers::users::get_user_location).patch(handlers::users::update_user_location),
        )
        .route(
            "/locations",
            get(handlers::locations::get_locations).post(handlers::locations::create_location),
        )
        .route(
            "/locations/:id",
            patch(handlers::locations::update_location).delete(handlers::locations::delete_location),
        )
        .route("/records", get(handlers::records::get_records))
        .route(
            "/records/admin/:admin_id",
            get(handlers::records::get_records_by_admin),
        )
        .route("/checkin", post(handlers::records::check_in))
        .layer(from_fn(middleware::auth::require_auth));

    Router::new()
        .merge(public.clone())
        .merge(private.clone())
        .nest("/api/v1", public.merge(private))
}
