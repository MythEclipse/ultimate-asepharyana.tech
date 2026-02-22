# Build stage using nightly Rust
FROM lukemathwalker/cargo-chef:latest-rust-nightly-bookworm AS chef
WORKDIR /app

# Install Bun for TailwindCSS
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

FROM chef AS planner
COPY apps/leptos ./apps/leptos
WORKDIR /app/apps/leptos
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/apps/leptos/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
# Build dependencies
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

# Install trunk (pre-compiled binary to save time)
RUN curl -L https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin

# Build application
COPY apps/leptos ./apps/leptos
WORKDIR /app/apps/leptos
RUN bun install
RUN trunk build --release --public-url "/"

# runtime stage
FROM nginx:alpine
COPY --from=builder /app/apps/leptos/dist /usr/share/nginx/html
COPY apps/leptos/docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

EXPOSE 8081
ENTRYPOINT ["/docker-entrypoint.sh"]
