FROM oven/bun:alpine AS base
RUN sed -i 's/dl-cdn.alpinelinux.org/mirror.kartolo.sby.datautama.net.id/g' /etc/apk/repositories
WORKDIR /app

# Copy pre-built bundle and package.json
COPY apps/elysia/dist ./dist
COPY apps/elysia/package.json ./

# Expose port
EXPOSE 4092

# Start the application using the pre-built bundle
CMD ["bun", "run", "dist/index.js"]
