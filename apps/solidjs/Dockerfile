FROM oven/bun:1-slim AS builder
WORKDIR /app

# Copy workspace configuration
COPY package.json bun.lock tsconfig.base.json ./
COPY apps/solidjs/package.json apps/solidjs/

# Install dependencies
RUN bun install

# Copy source code
COPY apps/solidjs apps/solidjs

# Build the application
WORKDIR /app/apps/solidjs
RUN bun run build

# Runtime stage
FROM oven/bun:1-slim
WORKDIR /app

COPY --from=builder /app/apps/solidjs/.output ./output
COPY --from=builder /app/apps/solidjs/package.json ./

EXPOSE 4090

# Vinxi start command
CMD ["bun", "run", "start"]
