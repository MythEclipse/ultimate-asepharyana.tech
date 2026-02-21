# build stage using trunk to produce dist
FROM rust:latest AS builder
WORKDIR /app

COPY apps/leptos/Cargo.toml apps/leptos/Trunk.toml ./
COPY apps/leptos/index.html ./
COPY apps/leptos/style ./style
COPY apps/leptos/src ./src

# install Node.js and npm for tailwind build hook
RUN apt-get update && apt-get install -y nodejs npm && rm -rf /var/lib/apt/lists/*

# copy package files so we can install frontend dependencies
COPY apps/leptos/package.json ./
# use npm install since we may not have a lockfile
RUN npm install

# allow passing visuals URL at build time (defaults to local nginx host)
ARG VISUALS_URL="http://visuals.localhost"
ENV VISUALS_URL=${VISUALS_URL}

RUN cargo install trunk --locked
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
