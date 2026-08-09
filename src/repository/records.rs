use crate::domain::{AttendanceRecord, AttendanceStatus, RecordType};
use sqlx::{mysql::{MySqlPool, MySqlRow}, Row};

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

fn record_type_to_db(record_type: &RecordType) -> &'static str {
    match record_type {
        RecordType::In => "in",
        RecordType::Out => "out",
    }
}

fn record_type_from_db(s: &str) -> RecordType {
    match s {
        "out" => RecordType::Out,
        _ => RecordType::In,
    }
}

const SELECT_COLUMNS: &str = "id, user_id, location_id, latitude, longitude, timestamp, status, record_type, record_date, error_message";

fn from_row(row: &MySqlRow) -> AttendanceRecord {
    AttendanceRecord {
        id: row.get::<String, _>("id"),
        user_id: row.get::<String, _>("user_id"),
        location_id: row.get::<String, _>("location_id"),
        latitude: row.get::<f64, _>("latitude"),
        longitude: row.get::<f64, _>("longitude"),
        timestamp: row.get::<i64, _>("timestamp"),
        status: status_from_db(&row.get::<String, _>("status")),
        record_type: record_type_from_db(&row.get::<String, _>("record_type")),
        record_date: row.try_get::<Option<chrono::NaiveDate>, _>("record_date").ok().flatten(),
        error_message: row.try_get::<Option<String>, _>("error_message").ok().flatten(),
        create_time: row.try_get::<Option<chrono::NaiveDateTime>, _>("create_time").ok().flatten(),
        update_time: row.try_get::<Option<chrono::NaiveDateTime>, _>("update_time").ok().flatten(),
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
    let rows = sqlx::query(&format!(
        "SELECT {SELECT_COLUMNS} FROM records WHERE delete_time IS NULL"
    ))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(|row| from_row(row)).collect())
}

pub async fn get_by_user(pool: &MySqlPool, user_id: &str) -> Result<Vec<AttendanceRecord>, sqlx::Error> {
    let rows = sqlx::query(&format!(
        "SELECT {SELECT_COLUMNS} FROM records WHERE user_id=? AND delete_time IS NULL"
    ))
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(|row| from_row(row)).collect())
}

pub async fn find_by_user_date_type(
    pool: &MySqlPool,
    user_id: &str,
    record_date: chrono::NaiveDate,
    record_type: &RecordType,
) -> Result<Option<AttendanceRecord>, sqlx::Error> {
    let rows = sqlx::query(&format!(
        "SELECT {SELECT_COLUMNS} FROM records WHERE user_id=? AND record_date=? AND record_type=? AND delete_time IS NULL"
    ))
        .bind(user_id)
        .bind(record_date)
        .bind(record_type_to_db(record_type))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().next().map(|row| from_row(row)))
}

pub async fn save(pool: &MySqlPool, record: &AttendanceRecord) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO records (id, user_id, location_id, latitude, longitude, timestamp, status, record_type, record_date, error_message)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            user_id=VALUES(user_id),
            location_id=VALUES(location_id),
            latitude=VALUES(latitude),
            longitude=VALUES(longitude),
            timestamp=VALUES(timestamp),
            status=VALUES(status),
            record_type=VALUES(record_type),
            record_date=VALUES(record_date),
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
        .bind(record_type_to_db(&record.record_type))
        .bind(record.record_date)
        .bind(&record.error_message)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as c FROM records WHERE delete_time IS NULL").fetch_one(pool).await?;
    Ok(row.get::<i64, _>("c"))
}
