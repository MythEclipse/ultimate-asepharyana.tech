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

# Inline Nginx config
RUN printf "server {\n\
    listen 80;\n\
    server_name _;\n\
    \n\
    location / {\n\
    proxy_pass http://127.0.0.1:4090;\n\
    proxy_set_header Host \$host;\n\
    proxy_set_header X-Real-IP \$remote_addr;\n\
    proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;\n\
    proxy_set_header X-Forwarded-Proto \$scheme;\n\
    proxy_set_header X-Forwarded-Host \$host;\n\
    proxy_set_header X-Forwarded-Port \$server_port;\n\
    }\n\
    \n\
    location /api/rust/ {\n\
    proxy_pass http://rust-api:4091/api/;\n\
    proxy_set_header Host \$host;\n\
    proxy_set_header X-Real-IP \$remote_addr;\n\
    proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;\n\
    proxy_set_header X-Forwarded-Proto \$scheme;\n\
    }\n\
    \n\
    location /api/elysia/ {\n\
    proxy_pass http://elysia-api:4092/api/;\n\
    proxy_set_header Host \$host;\n\
    proxy_set_header X-Real-IP \$remote_addr;\n\
    proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;\n\
    proxy_set_header X-Forwarded-Proto \$scheme;\n\
    }\n\
    }\n" > /etc/nginx/http.d/default.conf

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
