# Use stable chef image only to extract the binary
FROM lukemathwalker/cargo-chef:latest-rust-1.85 AS stable-chef

# Build stage using official library stable Rust on Bookworm (required for wgpu-based visuals)
FROM rust:bookworm AS chef
WORKDIR /app
COPY --from=stable-chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef

# install nightly toolchain so wgpu or other nightly features compile
RUN rustup toolchain install nightly && rustup default nightly

# Install Node.js for potential build scripts/hooks
RUN apt-get update && apt-get install -y --no-install-recommends nodejs && rm -rf /var/lib/apt/lists/*
ENV RUSTFLAGS="-C target-feature=+bulk-memory"

FROM chef AS planner
WORKDIR /app
COPY apps/visuals .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add wasm32-unknown-unknown
# Build dependencies using nightly chef
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

# Install trunk (pre-compiled binary)
RUN curl -L https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin

# Install binaryen 121 from GitHub (Debian's is too old for --enable-all)
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_121/binaryen-version_121-x86_64-linux.tar.gz \
    | tar -xzf- --strip-components=2 -C /usr/local/bin binaryen-version_121/bin/wasm-opt

# Build application (trunk's built-in wasm-opt is disabled in Trunk.toml)
COPY apps/visuals .
RUN trunk build --release --public-url "/"

# Run wasm-opt manually with all features enabled
RUN find dist -name '*.wasm' -exec wasm-opt -Oz --all-features {} -o {} \;

# runtime image
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
