use attendance_server::domain::User;
use attendance_server::dto::{LoginRequest, LoginResponse};

#[tokio::main]
async fn main() {
    let base = std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000/api/v1".to_string());
    let username = std::env::var("APP_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("APP_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    let client = reqwest::Client::new();
    let login = client
        .post(format!("{}/login", base))
        .json(&LoginRequest { username, password })
        .send()
        .await
        .expect("login failed");

    let login_status = login.status();
    if !login_status.is_success() {
        let text = login.text().await.unwrap_or_default();
        println!("login failed: status={} body={}", login_status, text);
        return;
    }
    let lr: LoginResponse = login.json().await.expect("invalid login json");
    let token = lr.token.expect("no token");

    let resp = client
        .get(format!("{}/users", base))
        .bearer_auth(token)
        .send()
        .await
        .expect("request failed");

    let status = resp.status();
    if status.is_success() {
        let users: Vec<User> = resp.json().await.expect("invalid json");
        println!("users count={} first={:?}", users.len(), users.get(0));
    } else {
        let text = resp.text().await.unwrap_or_default();
        println!("fetch users failed: status={} body={}", status, text);
    }
}
