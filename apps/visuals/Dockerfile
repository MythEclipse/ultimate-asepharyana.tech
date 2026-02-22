FROM lukemathwalker/cargo-chef:latest-rust-1.85-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY apps/visuals ./apps/visuals
WORKDIR /app/apps/visuals
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/apps/visuals/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
# Build dependencies
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
