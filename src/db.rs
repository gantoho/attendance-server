use crate::config::Config;
use crate::repositories::{locations, records, users};
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
                    // 尝试连接到服务器级别并创建数据库
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
                    // 再次尝试连接目标数据库
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

pub async fn init_schema(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    users::create_table(pool).await?;
    locations::create_table(pool).await?;
    records::create_table(pool).await?;
    Ok(())
}

pub async fn init_data(pool: &MySqlPool, cfg: &Config) -> Result<(), sqlx::Error> {
    if let (Some(ref username), Some(ref password)) = (&cfg.default_admin_username, &cfg.default_admin_password) {
        users::ensure_default_admin(pool, username, password).await?;
        users::migrate_assign_admin(pool, username).await?;
    }
    Ok(())
}

fn is_unknown_database(e: &SqlxError) -> bool {
    if let SqlxError::Database(db_err) = e {
        let msg = db_err.message().to_lowercase();
        return msg.contains("unknown database");
    }
    false
}

fn split_mysql_url(url: &str) -> Option<(String, String)> {
    // 形如 mysql://user:pass@host:port/dbname?params
    // 拆出 server_url: mysql://user:pass@host:port/ 与 dbname
    // 简单解析：找到最后一个 '/'（在 ? 之前）
    let without_params = url.split('?').next().unwrap_or(url);
    let idx = without_params.rfind('/')?;
    let db_name = without_params.get(idx + 1..)?.to_string();
    // 去掉数据库名与末尾的斜杠，得到不含数据库名的连接串
    let server_url = without_params[..idx].to_string();
    Some((server_url, db_name))
}

