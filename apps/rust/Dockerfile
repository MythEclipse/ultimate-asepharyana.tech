# Build stage
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./
COPY apps/rust/Cargo.toml apps/rust/Cargo.toml

# Dummy build to cache dependencies
RUN mkdir -p apps/rust/src && echo "fn main() {}" > apps/rust/src/main.rs
RUN cargo build --release --manifest-path apps/rust/Cargo.toml

# Copy actual source code
COPY apps/rust apps/rust
# Copy shared packages if any (based on workspace structure)
COPY packages packages

# Build final binary
RUN touch apps/rust/src/main.rs
RUN cargo build --release --manifest-path apps/rust/Cargo.toml

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rustexpress /app/rustexpress

# Expose port
EXPOSE 4091

# Run the binary
CMD ["./rustexpress"]
