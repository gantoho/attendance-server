use crate::models::{User, UserRole};
use sqlx::{mysql::MySqlPool, Row};

fn role_to_db(role: &UserRole) -> &'static str {
    match role {
        UserRole::Admin => "admin",
        UserRole::User => "user",
    }
}

fn role_from_db(s: &str) -> UserRole {
    match s {
        "admin" => UserRole::Admin,
        _ => UserRole::User,
    }
}

pub async fn create_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(64) PRIMARY KEY,
            username VARCHAR(191) NOT NULL UNIQUE,
            password VARCHAR(191) NOT NULL,
            role VARCHAR(16) NOT NULL,
            admin_id VARCHAR(64) NULL,
            location_id VARCHAR(64) NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
    "#;
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub async fn get_by_id(pool: &MySqlPool, id: &str) -> Result<Option<User>, sqlx::Error> {
    let row_opt = sqlx::query(
        "SELECT id, username, password, role, admin_id, location_id FROM users WHERE id=?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row_opt.map(|row| User {
        id: row.get::<String, _>("id"),
        username: row.get::<String, _>("username"),
        password: row.get::<String, _>("password"),
        role: role_from_db(&row.get::<String, _>("role")),
        admin_id: row.try_get::<Option<String>, _>("admin_id").ok().flatten(),
        location_id: row.try_get::<Option<String>, _>("location_id").ok().flatten(),
    }))
}

pub async fn get_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let row_opt = sqlx::query(
        "SELECT id, username, password, role, admin_id, location_id FROM users WHERE username=?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(row_opt.map(|row| User {
        id: row.get::<String, _>("id"),
        username: row.get::<String, _>("username"),
        password: row.get::<String, _>("password"),
        role: role_from_db(&row.get::<String, _>("role")),
        admin_id: row.try_get::<Option<String>, _>("admin_id").ok().flatten(),
        location_id: row.try_get::<Option<String>, _>("location_id").ok().flatten(),
    }))
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, username, password, role, admin_id, location_id FROM users")
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| User {
            id: row.get::<String, _>("id"),
            username: row.get::<String, _>("username"),
            password: row.get::<String, _>("password"),
            role: role_from_db(&row.get::<String, _>("role")),
            admin_id: row.try_get::<Option<String>, _>("admin_id").ok().flatten(),
            location_id: row.try_get::<Option<String>, _>("location_id").ok().flatten(),
        })
        .collect())
}

pub async fn save(pool: &MySqlPool, user: &User) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO users (id, username, password, role, admin_id, location_id)
        VALUES (?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            username=VALUES(username),
            password=VALUES(password),
            role=VALUES(role),
            admin_id=VALUES(admin_id),
            location_id=VALUES(location_id)
    "#;
    sqlx::query(sql)
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password)
        .bind(role_to_db(&user.role))
        .bind(&user.admin_id)
        .bind(&user.location_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &MySqlPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id=?").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as c FROM users").fetch_one(pool).await?;
    Ok(row.get::<i64, _>("c"))
}

pub async fn ensure_default_admin(pool: &MySqlPool, username: &str, password: &str) -> Result<(), sqlx::Error> {
    if get_by_username(pool, username).await?.is_none() {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            password: password.to_string(),
            role: UserRole::Admin,
            admin_id: None,
            location_id: None,
        };
        save(pool, &user).await?;
    }
    Ok(())
}

pub async fn migrate_assign_admin(pool: &MySqlPool, admin_username: &str) -> Result<(), sqlx::Error> {
    if let Some(admin) = get_by_username(pool, admin_username).await? {
        let admin_id = admin.id.clone();
        let sql = r#"
            UPDATE users
            SET admin_id=?
            WHERE role='user' AND (admin_id IS NULL OR admin_id='')
        "#;
        sqlx::query(sql).bind(admin_id).execute(pool).await?;
    }
    Ok(())
}

