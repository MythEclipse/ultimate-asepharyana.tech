FROM oven/bun:1-slim AS base
WORKDIR /app

# Copy workspace configuration
COPY package.json bun.lock tsconfig.base.json ./
COPY apps/elysia/package.json apps/elysia/
COPY packages/services/package.json packages/services/

# Install dependencies
RUN bun install

# Copy source code
COPY apps/elysia apps/elysia
COPY packages/services packages/services

# Work from elysia app directory
WORKDIR /app/apps/elysia

# Expose port
EXPOSE 4092

# Start the application
CMD ["bun", "run", "start"]
