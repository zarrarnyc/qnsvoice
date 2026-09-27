# qnsvoice-api — Rust/Axum API image.
#
# Multi-stage build that produces a linux/amd64 (and arm64-compatible) runtime
# image. The binary serves the QNS Voice platform API and, for now, a static
# "coming soon" landing page while the Astro/Alpine/HTMX frontend is built.
#
# Contract notes (follows gothicans/nychaparty Coolify patterns):
#   - Non-root runtime on an unprivileged port; Traefik owns 80/443.
#   - Config is baked into the image and read from runtime env vars.
#   - Healthcheck is answered directly by the container.

# ---------- build ----------
FROM rust:1.88-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy core/ and the manifests first so the dependency layer is cached.
# core/ is a path dependency and workspace member, so it must be present before
# the stub build runs.
COPY core ./core
COPY Cargo.toml Cargo.lock ./

# Build dependencies only (cached unless Cargo.toml/lock change).
RUN mkdir -p src && echo "fn main() {}" > src/main.rs \
    && cargo build --release --locked \
    && rm -rf src

# Copy the real sources.
COPY src ./src
COPY config ./config

# Final build (only the app crate recompiles).
RUN touch src/main.rs && cargo build --release --locked

# ---------- runtime ----------
FROM debian:bookworm-slim

# ca-certificates for outbound TLS, libssl3 because the toolchain links against
# it, and curl because the container healthcheck uses it.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Non-root with a fixed uid/gid so volume ownership is deterministic.
RUN groupadd --gid 10001 app \
    && useradd --uid 10001 --gid 10001 --no-create-home --shell /usr/sbin/nologin app

WORKDIR /app

COPY --from=builder /app/target/release/qnsvoice /usr/local/bin/qnsvoice

# config/ is baked in. Do not bind-mount over it in production.
COPY --from=builder /app/config ./config

# Runtime expects these paths to exist even while they are empty.
RUN mkdir -p /app/plugins /app/migrations

# Writable path for future uploads, owned by the runtime user.
RUN mkdir -p /app/uploads && chown -R app:app /app

USER app

EXPOSE 7200

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl -fsS http://127.0.0.1:7200/health || exit 1

CMD ["qnsvoice"]
