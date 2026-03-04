use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use crate::api::handlers::{
    auth::login,
    health::{health, db_info, stats},
    users::{get_users, create_user, delete_user, update_user_location, get_user_location},
    locations::{get_locations, create_location, update_location, delete_location},
    records::{get_records, get_records_by_admin, check_in},
};
use crate::state::AppState;
use crate::middleware;
use axum::middleware::from_fn;
use axum::{response::{IntoResponse, Html}, Json};

pub fn build_router() -> Router<AppState> {
    let public = Router::new()
        .route("/health", get(health))
        .route("/debug/dbpath", get(db_info))
        .route("/debug/stats", get(stats))
        .route("/login", post(login))
        .route("/openapi-view", get(|| async {
            let page = r#"<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <title>OpenAPI Viewer</title>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <style>
    body { font-family: system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif; margin: 24px; }
    .row { margin-bottom: 12px; }
    input { width: 70%; padding: 8px; }
    button { padding: 8px 14px; }
    pre { background: #f7f7f7; padding: 12px; border-radius: 6px; overflow: auto; max-height: 70vh; }
  </style>
</head>
<body>
  <h2>OpenAPI Viewer</h2>
  <div class="row">
    <label>Token/Bearer：</label>
    <input id="token" placeholder="粘贴 token，或以 Bearer 前缀" />
    <button id="load">加载</button>
  </div>
  <pre id="output">// 等待加载 ...</pre>
  <script>
    const url = '/api/v1/openapi.json';
    document.getElementById('load').onclick = async () => {
      const t = document.getElementById('token').value.trim();
      const headers = { 'Content-Type': 'application/json' };
      if (t) {
        headers['Authorization'] = t.startsWith('Bearer ') ? t : ('Bearer ' + t);
        headers['Token'] = t;
      }
      const out = document.getElementById('output');
      out.textContent = '// 请求中 ...';
      try {
        const resp = await fetch(url, { headers });
        const text = await resp.text();
        out.textContent = text;
      } catch (e) {
        out.textContent = '请求失败: ' + e;
      }
    };
  </script>
</body>
</html>"#;
            Html(page).into_response()
        }));

    let openapi = Router::new()
        .route("/openapi.json", get(|| async {
            let spec = crate::openapi::json();
            let val = serde_json::from_str::<serde_json::Value>(&spec).unwrap();
            let mut resp = Json(val).into_response();
            resp.headers_mut().insert(
                axum::http::header::CACHE_CONTROL,
                axum::http::HeaderValue::from_static("no-store, no-cache, must-revalidate"),
            );
            resp
        }));

    let private = Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", delete(delete_user))
        .route(
            "/users/:id/location",
            get(get_user_location).patch(update_user_location),
        )
        .route(
            "/locations",
            get(get_locations).post(create_location),
        )
        .route(
            "/locations/:id",
            patch(update_location).delete(delete_location),
        )
        .route("/records", get(get_records))
        .route(
            "/records/admin/:admin_id",
            get(get_records_by_admin),
        )
        .route("/checkin", post(check_in))
        .layer(from_fn(middleware::auth::require_auth));

    Router::new()
        .merge(public.clone())
        .merge(openapi.clone())
        .merge(private.clone())
        .nest("/api/v1", public.merge(openapi).merge(private))
}
