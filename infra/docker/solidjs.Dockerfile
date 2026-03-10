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



# Inline Supervisor config
RUN printf "[supervisord]\n\
    nodaemon=true\n\
    logfile=/dev/null\n\
    logfile_maxbytes=0\n\
    \n\
    [program:bun]\n\
    command=bun run .output/server/index.mjs\n\
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
    stderr_logfile_maxbytes=0\n" > /etc/supervisord.conf

ENV PORT=4090
EXPOSE 80

# run supervisor to manage both processes
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
