use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::Error as SqlxError;
use std::time::Duration;

pub async fn connect(db_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let attempt = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(db_url)
        .await;

    match attempt {
        Ok(pool) => Ok(pool),
        Err(e) => {
            if is_unknown_database(&e) {
                if let Some((server_url, db_name)) = split_mysql_url(db_url) {
                    let server_pool = MySqlPoolOptions::new()
                        .max_connections(1)
                        .acquire_timeout(Duration::from_secs(10))
                        .connect(&server_url)
                        .await?;
                    let create_sql = format!(
                        "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
                        db_name
                    );
                    sqlx::query(&create_sql).execute(&server_pool).await?;
                    drop(server_pool);
                    return MySqlPoolOptions::new()
                        .max_connections(10)
                        .acquire_timeout(Duration::from_secs(10))
                        .connect(db_url)
                        .await;
                }
            }
            Err(e)
        }
    }
}

fn is_unknown_database(e: &SqlxError) -> bool {
    if let SqlxError::Database(db_err) = e {
        let msg = db_err.message().to_lowercase();
        return msg.contains("unknown database");
    }
    false
}

fn split_mysql_url(url: &str) -> Option<(String, String)> {
    let without_params = url.split('?').next().unwrap_or(url);
    let idx = without_params.rfind('/')?;
    let db_name = without_params.get(idx + 1..)?.to_string();
    let server_url = without_params[..idx].to_string();
    Some((server_url, db_name))
}
