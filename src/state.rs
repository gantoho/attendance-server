use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

impl AppState {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

