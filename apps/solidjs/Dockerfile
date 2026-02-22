# build stage
FROM oven/bun:1-alpine AS builder
WORKDIR /app

# install dependencies
COPY apps/solidjs/package.json apps/solidjs/bun.lock ./
RUN bun install --frozen-lockfile

# build the application
COPY apps/solidjs ./
RUN bun run build

# runtime stage
FROM oven/bun:1-alpine
WORKDIR /app

# copy build artifacts and necessary runtime files
COPY --from=builder /app/.output ./.output
COPY apps/solidjs/package.json ./

EXPOSE 4090

# Vinxi/Nitro start command
CMD ["bun", "run", ".output/server/index.mjs"]
