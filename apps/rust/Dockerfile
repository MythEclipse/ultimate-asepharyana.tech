FROM lukemathwalker/cargo-chef:latest-rust-1.85 AS chef
WORKDIR /app

FROM chef AS planner
COPY apps/rust ./apps/rust
WORKDIR /app/apps/rust
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/apps/rust/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY apps/rust ./apps/rust
WORKDIR /app/apps/rust
RUN cargo build --release

# Final runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/apps/rust/target/release/rustexpress /app/rustexpress

EXPOSE 4091
CMD ["./rustexpress"]
