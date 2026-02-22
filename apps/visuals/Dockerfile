# Use stable chef image only to extract the binary
FROM lukemathwalker/cargo-chef:latest-rust-1.85 AS stable-chef

# Build stage using official nightly Rust (required for wgpu-based visuals)
FROM rustlang/rust:nightly-bookworm AS chef
WORKDIR /app
COPY --from=stable-chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef

FROM chef AS planner
COPY apps/visuals ./apps/visuals
WORKDIR /app/apps/visuals
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/apps/visuals/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
# Build dependencies using nightly chef
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

# Install trunk (pre-compiled binary)
RUN curl -L https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin

# Build application
COPY apps/visuals ./apps/visuals
WORKDIR /app/apps/visuals
RUN trunk build --release --public-url "/"

# runtime image
FROM nginx:alpine
COPY --from=builder /app/apps/visuals/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
