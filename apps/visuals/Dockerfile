# Build stage
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install trunk and wasm-target
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

# Copy source code
COPY . .

# Build the application
WORKDIR /app/apps/visuals
RUN trunk build --release

# Runtime stage (Nginx to serve static files)
FROM nginx:alpine

# Copy built files from builder
COPY --from=builder /app/apps/visuals/dist /usr/share/nginx/html

# Expose port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
