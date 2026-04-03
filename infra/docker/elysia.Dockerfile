# build stage
FROM oven/bun:1-alpine AS builder
WORKDIR /app

# install dependencies with cache mounts
COPY apps/elysia/package.json apps/elysia/bun.lock ./
RUN --mount=type=cache,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

# build the application
COPY apps/elysia ./
RUN bun run build

# runtime stage
FROM oven/bun:1-distroless
WORKDIR /app

# copy build artifacts
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./

# distroless uses nonroot user (UID 65532) by default, or we can use it
USER nonroot

EXPOSE 4092
CMD ["run", "dist/index.js"]
