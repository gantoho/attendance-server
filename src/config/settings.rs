use std::env;

pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub default_admin_username: Option<String>,
    pub default_admin_password: Option<String>,
    pub jwt_secret: String,
    pub token_exp_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let _ = dotenvy::dotenv();
        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:7982".to_string());
        let database_url = env::var("DATABASE_URL").map_err(|_| "缺少环境变量 DATABASE_URL".to_string())?;
        let default_admin_username = env::var("DEFAULT_ADMIN_USERNAME").ok();
        let default_admin_password = env::var("DEFAULT_ADMIN_PASSWORD").ok();
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| "缺少环境变量 JWT_SECRET".to_string())?;
        let token_exp_hours = env::var("TOKEN_EXP_HOURS").ok().and_then(|s| s.parse::<i64>().ok()).unwrap_or(24);
        Ok(Self {
            bind_address,
            database_url,
            default_admin_username,
            default_admin_password,
            jwt_secret,
            token_exp_hours,
        })
    }
}
