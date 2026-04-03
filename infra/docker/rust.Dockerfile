# Use cargo-chef for dependency caching
FROM lukemathwalker/cargo-chef:latest-rust-1.89.0 AS chef
WORKDIR /app

# Install Node.js if needed for build scripts
RUN apt-get update && apt-get install -y --no-install-recommends \
    nodejs \
    && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY apps/rust .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Utilize buildkit cache mounts for cargo
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json

# Build application
COPY apps/rust .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/rustexpress /app/rustexpress

# Final runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl3 \
    chromium \
    fonts-liberation \
    fonts-noto-color-emoji \
    && rm -rf /var/lib/apt/lists/*

# Add non-root user
RUN groupadd -g 1001 appgroup && \
    useradd -u 1001 -g appgroup -s /bin/sh appuser

WORKDIR /app
COPY --from=builder /app/rustexpress /app/rustexpress

# Run as non-root
USER appuser

EXPOSE 4091
CMD ["./rustexpress"]
