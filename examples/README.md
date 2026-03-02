# Examples

- client_login: POST /api/v1/login with env APP_USERNAME/APP_PASSWORD
- client_users: login then GET /api/v1/users with Bearer token

Env:
- APP_BASE_URL: default http://127.0.0.1:8000/api/v1
- APP_USERNAME: default admin
- APP_PASSWORD: default admin

Run:
- cargo run --example client_login
- cargo run --example client_users
