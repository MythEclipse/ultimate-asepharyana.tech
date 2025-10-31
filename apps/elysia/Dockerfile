# Use Bun's official image
FROM oven/bun:1 AS base
WORKDIR /app

# Install dependencies
FROM base AS install
COPY package.json bun.lockb ./
RUN bun install --frozen-lockfile

# Build the application
FROM base AS build
COPY --from=install /app/node_modules ./node_modules
COPY . .
RUN bun build src/index.ts --outdir ./dist --target bun

# Production image
FROM base AS release
COPY --from=build /app/dist ./dist
COPY --from=build /app/package.json ./

# Expose the port
EXPOSE 3001

# Run the application
ENTRYPOINT ["bun", "run", "dist/index.js"]
