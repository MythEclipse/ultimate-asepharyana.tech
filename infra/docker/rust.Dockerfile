# cargo-chef, Cargo, and rustc are all from the same image to avoid
# synthetic-Cargo.toml warnings from version mismatches (plugin keys, per-target edition).
FROM lukemathwalker/cargo-chef:latest-rust-1.89.0 AS chef
WORKDIR /app

# Install Node.js for build scripts that invoke node (e.g. build.rs hooks)
RUN apt-get update && apt-get install -y --no-install-recommends nodejs && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY apps/rust .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
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
