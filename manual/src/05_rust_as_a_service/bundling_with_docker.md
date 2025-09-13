# Bundling with Docker

Bundling with Docker is easy. In `code/actix_docker` I've built a simple containerized Actix web service. It's the same as the Diesel example - but listening on `0.0.0.0` and with a Dockerfile.

```dockerfile
## Multi-stage build for a minimal Actix + Diesel (SQLite) service

# 1) Build stage
FROM rust:1.89-bookworm AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

# 2) Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Diesel with SQLite needs libsqlite3 at runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/actix_docker /usr/local/bin/actix_docker

# Optional: default DB path inside container
ENV DATABASE_URL=/app/hello_db.db
ENV RUST_LOG=info

EXPOSE 8080
CMD ["actix_docker"]
```

You can build and run the container with:

```bash
docker build -t actix_diesel .
docker run -p 8080:8080 actix_diesel
```