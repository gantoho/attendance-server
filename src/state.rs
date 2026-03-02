use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub jwt_secret: String,
    pub token_exp_hours: i64,
}

impl AppState {
    pub fn new(pool: MySqlPool, jwt_secret: String, token_exp_hours: i64) -> Self {
        Self { pool, jwt_secret, token_exp_hours }
    }
}
