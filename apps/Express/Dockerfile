# Base image
FROM node:22-alpine AS base
WORKDIR /app

# Builder stage
FROM base AS builder
# Tentukan direktori cache untuk pnpm
ENV PNPM_STORE_PATH=/pnpm-store
RUN apk update && apk add --no-cache libc6-compat
RUN npm install -g pnpm turbo
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
RUN turbo prune @asepharyana/express --docker
RUN pnpm run build

# Production Runner
FROM base AS runner
WORKDIR /app
COPY --from=builder /app/dist /app/dist
COPY --from=builder /app/node_modules /app/node_modules
COPY --from=builder /app/package.json /app/

EXPOSE 4091
CMD ["node", "dist/index.js"]
