# build stage using trunk to produce dist
FROM rust:latest AS builder
WORKDIR /app

# install Bun for faster frontend dependency management
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

# ensure WASM target is available for building
RUN rustup target add wasm32-unknown-unknown

# copy manifest and lockfiles first for caching
COPY apps/leptos/Cargo.toml apps/leptos/Cargo.lock apps/leptos/Trunk.toml ./
COPY apps/leptos/package.json apps/leptos/bun.lock ./

# install frontend dependencies using bun
RUN bun install

# install trunk
RUN cargo install trunk --locked

# copy source and build
COPY apps/leptos ./
RUN trunk build --release --public-url "/"

# runtime stage
FROM nginx:alpine

# Copy generated files
COPY --from=builder /app/dist /usr/share/nginx/html

# Add entrypoint script to adjust port at runtime
COPY apps/leptos/docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

# default exposed port (can be changed via PORT env at runtime)
EXPOSE 8081

ENTRYPOINT ["/docker-entrypoint.sh"]
