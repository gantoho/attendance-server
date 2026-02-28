use crate::models::Location;
use sqlx::{mysql::MySqlPool, Row};

pub async fn create_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS locations (
            id VARCHAR(64) PRIMARY KEY,
            name VARCHAR(191) NOT NULL,
            latitude DOUBLE NOT NULL,
            longitude DOUBLE NOT NULL,
            radius DOUBLE NOT NULL,
            admin_id VARCHAR(64) NOT NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
    "#;
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub async fn get_by_id(pool: &MySqlPool, id: &str) -> Result<Option<Location>, sqlx::Error> {
    let row_opt = sqlx::query(
        "SELECT id, name, latitude, longitude, radius, admin_id FROM locations WHERE id=?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row_opt.map(|row| Location {
        id: row.get::<String, _>("id"),
        name: row.get::<String, _>("name"),
        latitude: row.get::<f64, _>("latitude"),
        longitude: row.get::<f64, _>("longitude"),
        radius: row.get::<f64, _>("radius"),
        admin_id: row.get::<String, _>("admin_id"),
    }))
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Location>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name, latitude, longitude, radius, admin_id FROM locations")
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| Location {
            id: row.get::<String, _>("id"),
            name: row.get::<String, _>("name"),
            latitude: row.get::<f64, _>("latitude"),
            longitude: row.get::<f64, _>("longitude"),
            radius: row.get::<f64, _>("radius"),
            admin_id: row.get::<String, _>("admin_id"),
        })
        .collect())
}

pub async fn save(pool: &MySqlPool, location: &Location) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO locations (id, name, latitude, longitude, radius, admin_id)
        VALUES (?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            name=VALUES(name),
            latitude=VALUES(latitude),
            longitude=VALUES(longitude),
            radius=VALUES(radius),
            admin_id=VALUES(admin_id)
    "#;
    sqlx::query(sql)
        .bind(&location.id)
        .bind(&location.name)
        .bind(location.latitude)
        .bind(location.longitude)
        .bind(location.radius)
        .bind(&location.admin_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &MySqlPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM locations WHERE id=?").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as c FROM locations").fetch_one(pool).await?;
    Ok(row.get::<i64, _>("c"))
}

