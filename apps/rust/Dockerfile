# syntax=docker/dockerfile:1

# --- Build Stage ---
FROM rust:1.74 AS builder

WORKDIR /app

# Copy manifest and lock files first for caching
COPY Cargo.toml Cargo.lock ./

# Copy source code and build dependencies
COPY src ./src
COPY build_utils ./build_utils
COPY migrations ./migrations

# Build release binary
RUN cargo build --release --manifest-path Cargo.toml --target-dir target

# --- Runtime Stage ---
FROM debian:stable-slim

WORKDIR /app

# Copy only the built binary
COPY --from=builder /app/target/release/rust /app/rust_app

EXPOSE 4091

CMD ["/app/rust_app"]
