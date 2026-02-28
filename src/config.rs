use std::env;

pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub default_admin_username: Option<String>,
    pub default_admin_password: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let _ = dotenvy::dotenv(); // 加载 .env（开发环境），缺失时忽略
        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:7982".to_string());
        let database_url = env::var("DATABASE_URL").map_err(|_| "缺少环境变量 DATABASE_URL".to_string())?;
        let default_admin_username = env::var("DEFAULT_ADMIN_USERNAME").ok();
        let default_admin_password = env::var("DEFAULT_ADMIN_PASSWORD").ok();
        Ok(Self {
            bind_address,
            database_url,
            default_admin_username,
            default_admin_password,
        })
    }
}
