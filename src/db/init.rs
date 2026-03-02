use crate::config::Config;
use sqlx::mysql::MySqlPool;

pub async fn init_data(pool: &MySqlPool, cfg: &Config) -> Result<(), sqlx::Error> {
    if let (Some(ref username), Some(ref password)) = (&cfg.default_admin_username, &cfg.default_admin_password) {
        crate::repository::users::ensure_default_admin(pool, username, password).await?;
        crate::repository::users::migrate_assign_admin(pool, username).await?;
    }
    Ok(())
}

pub async fn init_schema(pool: &MySqlPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
