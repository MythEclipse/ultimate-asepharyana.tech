# Base image
FROM node:22-alpine AS base
WORKDIR /app
RUN addgroup -S -g 1001 nodejs
RUN adduser -S -u 1001 -G nodejs nextjs

# Builder stage
FROM base AS builder
RUN apk update && apk add --no-cache libc6-compat
RUN npm install -g turbo
RUN corepack enable
RUN pnpm set version latest
WORKDIR /app
COPY package.json pnpm-lock.yaml ./ 
COPY . . 
RUN pnpm install --frozen-lockfile
RUN pnpm turbo build
# RUN turbo prune @asepharyana/web --docker

# Production Runner
FROM base AS runner
WORKDIR /app
USER nextjs
COPY --from=builder --chown=nextjs:nodejs /app/apps/NextJS/.next/standalone ./ 
COPY --from=builder --chown=nextjs:nodejs /app/apps/NextJS/.next/static ./apps/NextJS/.next/static
COPY --from=builder --chown=nextjs:nodejs /app/apps/NextJS/public ./apps/NextJS/public

EXPOSE 3000
CMD ["node", "/app/apps/NextJS/server.js"]
