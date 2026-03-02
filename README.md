复制 .env.example 到 .env .env.development .env.production
```bash
cp .env.example .env
cp .env.example .env.development
cp .env.example .env.production
```
并修改 .env .env.development .env.production 中的数据库连接信息为实际值

打包 Release 版本
```bash
cargo build --release
```
打包完注入变量运行
```bash
DATABASE_URL='mysql://root:pass@127.0.0.1:3306/attendance' \
BIND_ADDRESS='0.0.0.0:7982' \
DEFAULT_ADMIN_USERNAME="admin" \
DEFAULT_ADMIN_PASSWORD="your_password" \
./target/release/attendance-server
```
Run Release Linux
/usr/local/bin/start-attendance.sh（权限 700，归属部署用户）
```bash
#!/usr/bin/env bash
set -euo pipefail
export DATABASE_URL="mysql://user:pass@127.0.0.1:3306/attendance"
export BIND_ADDRESS="0.0.0.0:7982"
export DEFAULT_ADMIN_USERNAME="admin"
export DEFAULT_ADMIN_PASSWORD="your_password"
exec /opt/attendance/attendance-server
```

# 开发启动指南

## 环境变量(.env)
BIND_ADDRESS=127.0.0.1:8000
DATABASE_URL=mysql://root:password@127.0.0.1:3306/attendance
DEFAULT_ADMIN_USERNAME=admin
DEFAULT_ADMIN_PASSWORD=admin123
JWT_SECRET=dev-secret-please-change
TOKEN_EXP_HOURS=24

## 启动数据库
Docker:
docker compose up -d

## 启动服务
cargo run

## 示例客户端
cargo run --example client_login
cargo run --example client_users

## 故障排查
- 迁移版本缺失 VersionMissing(20260302)
  - 原因：旧迁移版本 20260302 已写入 _sqlx_migrations，但当前磁盘迁移集不包含该版本
  - 处理：
    - 登录 MySQL，删除旧记录：DELETE FROM _sqlx_migrations WHERE version = 20260302;
    - 或清空迁移表（仅在无数据或可重建时）：DROP TABLE IF EXISTS _sqlx_migrations;
    - 然后重新 cargo run，按新的 14 位时间戳迁移执行
