mod models;
mod config;
mod state;
mod db;
mod repositories;
mod handlers;
mod utils;

use axum::{routing::{delete, get, patch, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env().expect("加载配置失败：请设置 DATABASE_URL 或 .env");

    let pool = db::connect(&cfg.database_url).await.expect("connect mysql");
    db::init_schema(&pool).await.expect("init schema");
    db::init_data(&pool, &cfg).await.expect("init data");

    let state = AppState::new(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/debug/dbpath", get(handlers::health::db_info))
        .route("/debug/stats", get(handlers::health::stats))
        .route("/login", post(handlers::auth::login))
        .route("/users", get(handlers::users::get_users).post(handlers::users::create_user))
        .route("/users/:id", delete(handlers::users::delete_user))
        .route("/users/:id/location", get(handlers::users::get_user_location).patch(handlers::users::update_user_location))
        .route("/locations", get(handlers::locations::get_locations).post(handlers::locations::create_location))
        .route("/locations/:id", patch(handlers::locations::update_location).delete(handlers::locations::delete_location))
        .route("/records", get(handlers::records::get_records))
        .route("/records/admin/:admin_id", get(handlers::records::get_records_by_admin))
        .route("/checkin", post(handlers::records::check_in))
        .layer(cors)
        .with_state(state);

    let bind: SocketAddr = cfg.bind_address.parse().expect("Invalid BIND_ADDRESS");
    println!("Attendance server listening on http://{}", bind);
    axum::serve(tokio::net::TcpListener::bind(bind).await.unwrap(), app)
        .await
        .unwrap();
}
