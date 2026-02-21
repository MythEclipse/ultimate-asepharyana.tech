# multi-stage build: compile inside a Rust builder image using a modern toolchain
FROM rust:latest AS builder

WORKDIR /app

# copy the entire rust application so cargo can resolve workspace paths if any
COPY apps/rust ./apps/rust

WORKDIR /app/apps/rust

# build the release binary using the default host target (glibc)
RUN cargo build --release

# final runtime image based on a slim Debian distribution
FROM debian:bookworm-slim AS runtime
# install only required runtime dependencies (libssl is critical for most Rust apps)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# copy the compiled binary from the builder stage
COPY --from=builder /app/apps/rust/target/release/rustexpress /app/rustexpress

# expose service port
EXPOSE 4091

# run the binary
CMD ["./rustexpress"]
