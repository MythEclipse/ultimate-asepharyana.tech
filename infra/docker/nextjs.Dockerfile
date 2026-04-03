# ─── Stage 1: Build Base ──────────────────────────────────────────────────────
FROM node:22-alpine AS base

# Install Bun for faster dependency management
RUN apk add --no-cache libc6-compat gcompat curl bash && \
    curl -fsSL https://bun.sh/install | bash && \
    cp /root/.bun/bin/bun /usr/local/bin/bun

WORKDIR /app

# ─── Stage 2: Install dependencies ──────────────────────────────────────────
FROM base AS deps
COPY apps/nextjs/package.json apps/nextjs/bun.lock* ./
# Use cache mount for bun install
RUN --mount=type=cache,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

# ─── Stage 3: Build ──────────────────────────────────────────────────────────
FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY apps/nextjs .

ENV NEXT_CPU_COUNT=1
ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production

# next build output: standalone
RUN npx next build

# ─── Stage 4: Runtime (slim) ─────────────────────────────────────────────────
FROM node:22-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1
ENV PORT=3000
ENV HOSTNAME=0.0.0.0

# Add non-root user
RUN addgroup --system --gid 1001 nodejs && \
    adduser  --system --uid 1001 nextjs

# standalone bundle includes its own node_modules subset
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
COPY --from=builder --chown=nextjs:nodejs /app/public ./public

USER nextjs

EXPOSE 3000
CMD ["node", "server.js"]
