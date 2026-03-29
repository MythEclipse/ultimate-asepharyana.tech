# ─── Stage 1: Install dependencies ──────────────────────────────────────────
FROM oven/bun:1-alpine AS deps
# Add libc6-compat for native binary support in Alpine/musl
RUN apk add --no-cache libc6-compat
WORKDIR /app

COPY apps/nextjs/package.json apps/nextjs/bun.lock* ./
RUN bun install --frozen-lockfile

# ─── Stage 2: Build ──────────────────────────────────────────────────────────
FROM node:22-alpine AS builder
# Add compatibility layers for native Next.js/Turbopack binaries
RUN apk add --no-cache libc6-compat gcompat curl bash
WORKDIR /app

# Ensure we have Bun available for running scripts if needed
RUN curl -fsSL https://bun.sh/install | bash && \
    cp /root/.bun/bin/bun /usr/local/bin/bun

# Copy deps from previous stage
COPY --from=deps /app/node_modules ./node_modules

# Copy full app source
COPY apps/nextjs .

# Environment stabilization for Turbopack in containers
ENV NEXT_CPU_COUNT=1
ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production

# next build output: standalone (copies only what's needed to run)
# We use node directly for the build engine to ensure maximum compatibility in Alpine/musl
RUN npx next build --no-lint

# ─── Stage 3: Runtime (slim) ─────────────────────────────────────────────────
FROM node:22-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1
ENV PORT=3000
ENV HOSTNAME=0.0.0.0

RUN addgroup --system --gid 1001 nodejs && \
    adduser  --system --uid 1001 nextjs

# standalone bundle includes its own node_modules subset
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
COPY --from=builder --chown=nextjs:nodejs /app/public ./public

USER nextjs

EXPOSE 3000
CMD ["node", "server.js"]
