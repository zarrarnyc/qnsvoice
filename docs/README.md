# qnsvoice

Queens voice/news platform

## Architecture

This project uses the **resightings-core** framework for:
- Authentication and authorization
- Multi-tenant isolation
- Database migrations
- Plugin system
- Configuration management

## Getting Started

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+
- Redis 6+

### Setup

1. Copy environment file:
   ```bash
   cp .env.example .env
   ```

2. Update `.env` with your database credentials and secrets

3. Build the project:
   ```bash
   cargo build
   ```

4. Run migrations:
   ```bash
   cargo run --bin qnsvoice
   ```

## Development

### Updating Core
```bash
git submodule update --remote core
```

### Adding Plugins
Create a new directory in `plugins/` with:
- `manifest.toml` - Plugin metadata
- `migrations/` - Database migrations
- Source code

## Project Structure
```
qnsvoice/
  core/              # Git submodule to resightings-core
  src/               # Application source
  config/            # Configuration files
  plugins/           # Project-specific plugins
  migrations/        # Database migrations
  templates/         # HTML templates
  static/            # Static assets (CSS, JS)
  docs/              # Documentation
  Cargo.toml
```
