# Base image
FROM oven/bun:latest AS base
WORKDIR /app
RUN addgroup -S -g 1001 nodejs
RUN adduser -S -u 1001 -G nodejs nextjs

# Builder stage
FROM base AS builder
RUN bun install -g turbo
WORKDIR /app
COPY package.json bun.lockb ./ 
COPY . . 
RUN bun install --filter @asepharyana/web
RUN bun run turbo build
# RUN turbo prune @asepharyana/web --docker

# Production Runner
FROM base AS runner
WORKDIR /app
USER nextjs
COPY --from=builder --chown=nextjs:nodejs /app/apps/NextJS/.next/standalone ./ 
COPY --from=builder --chown=nextjs:nodejs /app/apps/NextJS/.next/static ./apps/NextJS/.next/static
COPY --from=builder --chown=nextjs:nodejs /app/apps/NextJS/public ./apps/NextJS/public

EXPOSE 3000
CMD ["bun", "run", "start"]