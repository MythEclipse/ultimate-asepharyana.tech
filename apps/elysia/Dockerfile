# build stage
FROM oven/bun:alpine AS builder
WORKDIR /app

# install dependencies
COPY apps/elysia/package.json apps/elysia/bun.lock ./
RUN bun install

# build the application
COPY apps/elysia ./
RUN bun run build

# runtime stage
FROM oven/bun:alpine
WORKDIR /app

# copy build artifacts and necessary runtime files
COPY --from=builder /app/dist ./dist
COPY apps/elysia/package.json ./

EXPOSE 4092
CMD ["bun", "run", "dist/index.js"]
