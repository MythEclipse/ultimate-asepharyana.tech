# ─── Stage 1: Build Base ──────────────────────────────────────────────────────
FROM rust:bookworm AS base

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    nodejs \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Cargo Chef
RUN cargo install cargo-chef --locked

# Install Trunk and Wasm-Opt in a single layer
RUN curl -L https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin && \
    curl -L https://github.com/WebAssembly/binaryen/releases/download/version_121/binaryen-version_121-x86_64-linux.tar.gz \
    | tar -xzf- --strip-components=2 -C /usr/local/bin binaryen-version_121/bin/wasm-opt

# Setup toolchain
RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app
ENV RUSTFLAGS="-C target-feature=+bulk-memory"

# ─── Stage 2: Planner ────────────────────────────────────────────────────────
FROM base AS planner
COPY apps/visuals .
RUN cargo chef prepare --recipe-path recipe.json

# ─── Stage 3: Builder ────────────────────────────────────────────────────────
FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies with cache mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

# Build the actual application
COPY apps/visuals .
RUN trunk build --release --public-url "/"

# Run wasm-opt manually with all features enabled (-Oz for size)
RUN find dist -name '*.wasm' -exec wasm-opt -Oz --all-features {} -o {} \;

# ─── Stage 4: Runtime ────────────────────────────────────────────────────────
FROM nginx:alpine AS runner
COPY --from=builder /app/dist /usr/share/nginx/html
COPY infra/nginx/visuals.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
