# build the Elysia TypeScript project inside the container so that
# the image does not depend on host-generated artifacts
FROM oven/bun:alpine AS builder
WORKDIR /app

# copy package metadata and source
COPY apps/elysia/package.json apps/elysia/bun.lock ./
COPY apps/elysia/tsconfig.json ./
# include any config or script directories that might be referenced
COPY apps/elysia/scripts ./scripts
COPY apps/elysia/src ./src

# install dependencies then build the bundled output
RUN bun install
RUN bun build src/index.ts --outdir ./dist --target bun

# runtime image
FROM oven/bun:alpine
WORKDIR /app

# copy build artifacts and package metadata for potential runtime requirements
COPY --from=builder /app/dist ./dist
COPY apps/elysia/package.json ./

EXPOSE 4092
CMD ["bun", "run", "dist/index.js"]
