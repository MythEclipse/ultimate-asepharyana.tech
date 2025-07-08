# Base image
FROM oven/bun:latest AS base
WORKDIR /app

# Builder stage
FROM base AS builder
RUN bun install -g turbo
WORKDIR /app

# Copy package files first to install dependencies
COPY package.json bun.lockb ./ 

# Copy the rest of the application files
COPY . . 

# Run the turbo prune command to prune only Express dependencies
RUN turbo prune @asepharyana/express --docker

# Install dependencies focused on Express
RUN bun install --filter @asepharyana/express

# Build the application
RUN bun run build:express

# Production Runner
FROM base AS runner
WORKDIR /app

# Copy built dist and node_modules from the builder stage
COPY --from=builder /app/apps/Express/dist /app/dist
COPY --from=builder /app/apps/Express/node_modules /app/node_modules
COPY --from=builder /app/apps/Express/package.json /app/

# List files in /app to verify the files have been copied
RUN ls -la . 
RUN ls -la /app 

# List files in /app/dist to verify the built files exist
RUN ls -la /app/dist

# List files in /app/node_modules to verify node_modules exist
RUN ls -la /app/node_modules

# Expose the port and define the entrypoint
EXPOSE 4091
CMD ["bun", "start"]