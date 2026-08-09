use attendance_server::dto::{LoginRequest, LoginResponse, RefreshRequest};

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
        println!("login ok, access_token={:?}", body.token);
        println!("refresh_token={:?}", body.refresh_token);

        // 演示通过 refresh token 自动获取新的令牌
        if let Some(refresh_token) = &body.refresh_token {
            let resp2 = client
                .post(format!("{}/refresh", base))
                .json(&RefreshRequest { refresh_token: refresh_token.clone() })
                .send()
                .await
                .expect("refresh request failed");
            let status2 = resp2.status();
            if status2.is_success() {
                let body2: LoginResponse = resp2.json().await.expect("invalid refresh json");
                println!("refresh ok, new access_token={:?}", body2.token);
                println!("new refresh_token={:?}", body2.refresh_token);
            } else {
                let text = resp2.text().await.unwrap_or_default();
                println!("refresh failed: status={} body={}", status2, text);
            }
        }
    } else {
        let text = resp.text().await.unwrap_or_default();
        println!("login failed: status={} body={}", status, text);
    }
}
