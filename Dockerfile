# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release -p api

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/pea-api /usr/local/bin/pea-api
COPY migrations ./migrations
COPY fixtures/sample-app ./fixtures/sample-app

ENV PEA_BIND=0.0.0.0:8080 \
    PEA_REPO_PATH=/app/fixtures/sample-app \
    PEA_ARTIFACTS_DIR=/app/artifacts \
    PEA_MIGRATIONS_DIR=/app/migrations

EXPOSE 8080
VOLUME ["/app/artifacts"]
CMD ["pea-api"]
