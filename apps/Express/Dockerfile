# Base image
FROM node:22-alpine AS base
WORKDIR /app

# Builder stage
FROM base AS builder
RUN apk update && apk add --no-cache libc6-compat
RUN npm install -g turbo
RUN corepack enable
RUN yarn set version stable
WORKDIR /app

# Copy package files first to install dependencies
COPY package.json yarn.lock ./ 

# Copy the rest of the application files
COPY . . 

# Run the turbo prune command to prune only Express dependencies
RUN turbo prune @asepharyana/express --docker

# Install dependencies focused on Express
RUN yarn workspaces focus @asepharyana/express

# Build the application
RUN yarn build:express

# Production Runner
FROM base AS runner
WORKDIR /app

# Copy built dist and node_modules from the builder stage
COPY --from=builder /app/apps/Express/dist /app/dist
COPY --from=builder /app/node_modules /app/node_modules
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
CMD ["npm", "run", "start"]