use attendance_server::state::AppState;
use attendance_server::{config, db, api};

use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env().expect("加载配置失败：请设置 DATABASE_URL 或 .env");

    let pool = db::connect(&cfg.database_url).await.expect("connect mysql");
    db::init_schema(&pool).await.expect("init schema");
    db::init_data(&pool, &cfg).await.expect("init data");

    let state = AppState::new(pool, cfg.jwt_secret.clone(), cfg.token_exp_hours);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = api::router::build_router()
        .layer(cors)
        .with_state(state);

    attendance_server::openapi::write_to_file("openapi.json");

    let bind: SocketAddr = cfg.bind_address.parse().expect("Invalid BIND_ADDRESS");
    println!("Attendance server listening on http://{}", bind);
    axum::serve(tokio::net::TcpListener::bind(bind).await.unwrap(), app)
        .await
        .unwrap();
}
