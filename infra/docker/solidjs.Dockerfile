# build stage
FROM oven/bun:1-alpine AS builder
WORKDIR /app

# install dependencies with cache mounts
COPY apps/solidjs/package.json apps/solidjs/bun.lock ./
RUN --mount=type=cache,target=/root/.bun/install/cache \
    bun install --frozen-lockfile

# build the application
COPY apps/solidjs ./
RUN bun run build

# runtime stage
FROM oven/bun:1-alpine
WORKDIR /app

# install nginx and supervisor
RUN apk add --no-cache nginx supervisor

# Add non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup && \
    chown -R appuser:appgroup /app /var/lib/nginx /var/log/nginx /run/nginx

# copy build artifacts
COPY --from=builder --chown=appuser:appgroup /app/.output ./.output
COPY --from=builder --chown=appuser:appgroup /app/package.json ./
COPY --chown=appuser:appgroup infra/nginx/solidjs.conf /etc/nginx/http.d/default.conf

# Inline Supervisor config (run as non-root)
RUN printf "[supervisord]\n\
nodaemon=true\n\
user=appuser\n\
logfile=/dev/null\n\
logfile_maxbytes=0\n\
\n\
[program:bun]\n\
command=bun run start\n\
directory=/app\n\
autostart=true\n\
autorestart=true\n\
stdout_logfile=/dev/stdout\n\
stdout_logfile_maxbytes=0\n\
stderr_logfile=/dev/stderr\n\
stderr_logfile_maxbytes=0\n\
\n\
[program:nginx]\n\
command=nginx -g \"daemon off;\"\n\
autostart=true\n\
autorestart=true\n\
stdout_logfile=/dev/stdout\n\
stdout_logfile_maxbytes=0\n\
stderr_logfile=/dev/stderr\n\
stderr_logfile_maxbytes=0\n" > /etc/supervisord.conf && \
    chown appuser:appgroup /etc/supervisord.conf && \
    chmod 644 /etc/supervisord.conf

ENV PORT=4090
EXPOSE 80

# run supervisor
USER appuser
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
