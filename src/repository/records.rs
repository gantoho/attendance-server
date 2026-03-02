use crate::domain::{AttendanceRecord, AttendanceStatus};
use sqlx::{mysql::MySqlPool, Row};

fn status_to_db(status: &AttendanceStatus) -> &'static str {
    match status {
        AttendanceStatus::Success => "success",
        AttendanceStatus::Failed => "failed",
    }
}

fn status_from_db(s: &str) -> AttendanceStatus {
    match s {
        "success" => AttendanceStatus::Success,
        _ => AttendanceStatus::Failed,
    }
}

pub async fn create_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS records (
            id VARCHAR(64) PRIMARY KEY,
            user_id VARCHAR(64) NOT NULL,
            location_id VARCHAR(64) NOT NULL,
            latitude DOUBLE NOT NULL,
            longitude DOUBLE NOT NULL,
            timestamp BIGINT NOT NULL,
            status VARCHAR(16) NOT NULL,
            error_message TEXT NULL,
            INDEX idx_user_id (user_id),
            INDEX idx_location_id (location_id)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
    "#;
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<AttendanceRecord>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, user_id, location_id, latitude, longitude, timestamp, status, error_message FROM records WHERE delete_time IS NULL")
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| AttendanceRecord {
            id: row.get::<String, _>("id"),
            user_id: row.get::<String, _>("user_id"),
            location_id: row.get::<String, _>("location_id"),
            latitude: row.get::<f64, _>("latitude"),
            longitude: row.get::<f64, _>("longitude"),
            timestamp: row.get::<i64, _>("timestamp"),
            status: status_from_db(&row.get::<String, _>("status")),
            error_message: row.try_get::<Option<String>, _>("error_message").ok().flatten(),
            create_time: row.try_get::<Option<chrono::NaiveDateTime>, _>("create_time").ok().flatten(),
            update_time: row.try_get::<Option<chrono::NaiveDateTime>, _>("update_time").ok().flatten(),
        })
        .collect())
}

pub async fn get_by_user(pool: &MySqlPool, user_id: &str) -> Result<Vec<AttendanceRecord>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, user_id, location_id, latitude, longitude, timestamp, status, error_message FROM records WHERE user_id=? AND delete_time IS NULL")
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| AttendanceRecord {
            id: row.get::<String, _>("id"),
            user_id: row.get::<String, _>("user_id"),
            location_id: row.get::<String, _>("location_id"),
            latitude: row.get::<f64, _>("latitude"),
            longitude: row.get::<f64, _>("longitude"),
            timestamp: row.get::<i64, _>("timestamp"),
            status: status_from_db(&row.get::<String, _>("status")),
            error_message: row.try_get::<Option<String>, _>("error_message").ok().flatten(),
            create_time: row.try_get::<Option<chrono::NaiveDateTime>, _>("create_time").ok().flatten(),
            update_time: row.try_get::<Option<chrono::NaiveDateTime>, _>("update_time").ok().flatten(),
        })
        .collect())
}

pub async fn save(pool: &MySqlPool, record: &AttendanceRecord) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO records (id, user_id, location_id, latitude, longitude, timestamp, status, error_message)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            user_id=VALUES(user_id),
            location_id=VALUES(location_id),
            latitude=VALUES(latitude),
            longitude=VALUES(longitude),
            timestamp=VALUES(timestamp),
            status=VALUES(status),
            error_message=VALUES(error_message),
            delete_time=NULL
    "#;
    sqlx::query(sql)
        .bind(&record.id)
        .bind(&record.user_id)
        .bind(&record.location_id)
        .bind(record.latitude)
        .bind(record.longitude)
        .bind(record.timestamp)
        .bind(status_to_db(&record.status))
        .bind(&record.error_message)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as c FROM records WHERE delete_time IS NULL").fetch_one(pool).await?;
    Ok(row.get::<i64, _>("c"))
}
