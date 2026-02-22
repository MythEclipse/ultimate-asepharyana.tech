# build stage
FROM oven/bun:1-alpine AS builder
WORKDIR /app

# install dependencies
COPY apps/solidjs/package.json apps/solidjs/bun.lock ./
RUN bun install --frozen-lockfile

# build the application
COPY apps/solidjs ./
RUN bun run build

# runtime stage
FROM oven/bun:1-alpine
WORKDIR /app

# install nginx and supervisor
RUN apk add --no-cache nginx supervisor

# copy build artifacts and necessary runtime files
COPY --from=builder /app/.output ./.output
COPY apps/solidjs/package.json ./

# copy nginx config
COPY apps/solidjs/nginx.conf /etc/nginx/http.d/default.conf

# copy supervisor config
COPY apps/solidjs/supervisord.conf /etc/supervisord.conf

EXPOSE 80

# run supervisor to manage both processes
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
