# Use stable chef image only to extract the binary
FROM lukemathwalker/cargo-chef:latest-rust-1.85 AS stable-chef

# Build stage using official library stable Rust on Bookworm
FROM rust:bookworm AS chef
WORKDIR /app
COPY --from=stable-chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef

# install nightly toolchain in case crates require unstable features
RUN rustup toolchain install nightly && rustup default nightly

# Install Node.js for potential build scripts/hooks
RUN apt-get update && apt-get install -y --no-install-recommends nodejs && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
WORKDIR /app
COPY apps/rust .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies using nightly chef
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY apps/rust .
RUN cargo build --release

# Final runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl libssl3 chromium fonts-liberation fonts-noto-color-emoji && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/rustexpress /app/rustexpress

EXPOSE 4091
CMD ["./rustexpress"]
