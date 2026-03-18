# Stage 1: Build
FROM rust:1.85-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY style/ style/
COPY public/ public/
COPY migrations/ migrations/

# Set Tailwind version for cargo-leptos to download
ENV LEPTOS_TAILWIND_VERSION=v4.1.8
ENV LEPTOS_OUTPUT_NAME=physics-tree
ENV LEPTOS_SITE_ROOT=target/site

# Build release
RUN cargo leptos build --release

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy server binary
COPY --from=builder /app/target/release/server ./server
# Copy site assets (WASM, CSS, static files)
COPY --from=builder /app/target/site ./site
# Copy migrations for runtime migration support
COPY --from=builder /app/migrations ./migrations

ENV LEPTOS_OUTPUT_NAME=physics-tree
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
  CMD curl -f http://localhost:3000/api/health || exit 1

CMD ["./server"]
