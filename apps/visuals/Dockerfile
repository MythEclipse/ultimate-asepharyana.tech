# builder stage compiles the Leptos/Trunk project
FROM rust:latest AS builder
WORKDIR /app

# ensure WASM target is available for building
RUN rustup target add wasm32-unknown-unknown

# copy manifest and assets first for caching
COPY apps/visuals/Cargo.toml apps/visuals/Cargo.lock apps/visuals/Trunk.toml ./

# install trunk
RUN cargo install trunk --locked

# copy source and build
COPY apps/visuals ./
RUN trunk build --release --public-url "/"

# runtime image serving static files
FROM nginx:alpine

# copy output of trunk build
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
