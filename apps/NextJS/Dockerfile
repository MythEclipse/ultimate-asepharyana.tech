# Stage 1: Base image
FROM node:18-alpine AS base
WORKDIR /app

# Create a 'nextjs' user explicitly in the base image
RUN adduser -D nextjs

# Stage 2: Builder
FROM base AS builder
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apk update && apk add --no-cache libc6-compat
RUN npm install -g pnpm
COPY . .
RUN pnpm turbo prune web --docker

# Stage 3: Installer
FROM base AS installer
RUN apk update && apk add --no-cache libc6-compat
WORKDIR /app

# First install the dependencies (as they change less often)
COPY --from=builder /app/out/json/ . 
RUN pnpm install

# Build the project
COPY --from=builder /app/out/full/ .

# Uncomment and use build args to enable remote caching
# ARG TURBO_TEAM
# ENV TURBO_TEAM=$TURBO_TEAM

# ARG TURBO_TOKEN
# ENV TURBO_TOKEN=$TURBO_TOKEN

RUN pnpm turbo build

# Stage 4: Runner
FROM base AS runner
WORKDIR /app

# Don't run production as root
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
USER nextjs

# Automatically leverage output traces to reduce image size
# https://nextjs.org/docs/advanced-features/output-file-tracing
COPY --from=installer --chown=nextjs:nodejs /app/apps/NextJS/.next/standalone ./
COPY --from=installer --chown=nextjs:nodejs /app/apps/NextJS/.next/static ./apps/NextJS/.next/static
COPY --from=installer --chown=nextjs:nodejs /app/apps/NextJS/public ./apps/NextJS/public

EXPOSE 3090
CMD ["node", "apps/NextJS/server.js"]
