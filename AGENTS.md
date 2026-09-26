# qnsvoice - Project AGENTS.md

**Purpose:** Local project context for qnsvoice.

## Ports
- Assigned port block: **72xx** (see shared registry `L:\Verdent\PORTS.md`)
- Backend API: `7200`
- PostgreSQL (host): `7204` → 5432 (container)
- Redis (host): `7205` → 6379 (container)

## Startup
1. Start Docker services (if any): `docker-compose up -d`
2. Run backend: `cargo run` (listens on port `7200`)

## Environment
- `config/app.toml` → `[server] port = 7200`
- `.env.example` → `PORT=7200`, `DATABASE_URL` on `localhost:7204`, `REDIS_URL` on `localhost:7205`
