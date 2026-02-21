FROM oven/bun:alpine
WORKDIR /app

# Copy Nitro output from the host
COPY apps/solidjs/.output ./output
COPY apps/solidjs/package.json ./

EXPOSE 4090

# Vinxi/Nitro start command
CMD ["bun", "run", "output/server/index.mjs"]
