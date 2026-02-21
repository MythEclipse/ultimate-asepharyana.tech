# builder stage compiles the Leptos/Trunk project
FROM rust:latest AS builder

WORKDIR /app

# copy manifest and assets first for caching
COPY apps/visuals/Cargo.toml apps/visuals/Trunk.toml ./
# include any static files needed for trunk build
COPY apps/visuals/index.html ./
COPY apps/visuals/assets ./assets
COPY apps/visuals/src ./src

# install trunk and build
RUN cargo install trunk --locked
RUN trunk build --release --public-url "/"

# runtime image serving static files
FROM nginx:alpine

# copy output of trunk build
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
