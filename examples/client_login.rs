use attendance_server::dto::{LoginRequest, LoginResponse};

#[tokio::main]
async fn main() {
    let base = std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000/api/v1".to_string());
    let username = std::env::var("APP_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("APP_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    let req = LoginRequest { username, password };
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/login", base))
        .json(&req)
        .send()
        .await
        .expect("request failed");

    let status = resp.status();
    if status.is_success() {
        let body: LoginResponse = resp.json().await.expect("invalid json");
        println!("login ok, token={:?}", body.token);
    } else {
        let text = resp.text().await.unwrap_or_default();
        println!("login failed: status={} body={}", status, text);
    }
}
