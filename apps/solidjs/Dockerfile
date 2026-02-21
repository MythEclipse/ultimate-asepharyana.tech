FROM oven/bun:alpine
RUN sed -i 's/dl-cdn.alpinelinux.org/mirror.kartolo.sby.datautama.net.id/g' /etc/apk/repositories
WORKDIR /app

# Copy Nitro output from the host
COPY apps/solidjs/.output ./output
COPY apps/solidjs/package.json ./

EXPOSE 4090

# Vinxi/Nitro start command
CMD ["bun", "run", "output/server/index.mjs"]
