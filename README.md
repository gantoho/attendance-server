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