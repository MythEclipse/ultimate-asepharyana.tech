FROM node:20-alpine

# Set working directory
WORKDIR /app

# Install 9router and dependencies
RUN npm install -g 9router && \
    apk update && \
    apk add --no-cache nginx apache2-utils procps

# Create necessary directories
RUN mkdir -p /run/nginx /root/.9router

# Configure Nginx
RUN cat <<'EOF' > /etc/nginx/nginx.conf
worker_processes auto;
events { 
    worker_connections 1024; 
}
http {
    include mime.types;
    default_type application/octet-stream;
    sendfile on;
    keepalive_timeout 65;

    # Gzip settings
    gzip on;
    gzip_disable "msie6";
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    server {
        listen 80;
        server_name _;

        # Dashboard location
        location / {
            auth_basic "Restricted 9Router Dashboard";
            auth_basic_user_file /etc/nginx/.htpasswd;

            proxy_pass http://127.0.0.1:20128;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # API location
        location /v1/ {
            if ($http_authorization != "Bearer API_KEY_PLACEHOLDER") {
                return 401 '{"error": {"message": "Unauthorized: Invalid API Key", "type": "invalid_request_error"}}';
            }

            proxy_pass http://127.0.0.1:20128/v1/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            
            # OpenAI streaming support
            proxy_buffering off;
            proxy_cache off;
            chunked_transfer_encoding on;
            proxy_set_header Connection '';
            proxy_http_version 1.1;
            
            # Long-running requests
            proxy_read_timeout 600s;
        }
    }
}
EOF

# Improved entrypoint with process monitoring
RUN cat <<'EOF' > /entrypoint.sh
#!/bin/sh
set -e

ADMIN_USER=${ADMIN_USER:-admin}
ADMIN_PASS=${ADMIN_PASS:-password}
API_KEY=${API_KEY:-sk-rahasia-super-aman}

# Setup Basic Auth
htpasswd -cb /etc/nginx/.htpasswd "$ADMIN_USER" "$ADMIN_PASS"

# Inject API Key into Nginx config
sed -i "s|API_KEY_PLACEHOLDER|$API_KEY|g" /etc/nginx/nginx.conf

# Start 9router
echo "Starting 9router..."
9router --no-browser --log &
NINEROUTER_PID=$!

# Wait for 9router to start
sleep 5

# Start Nginx
echo "Starting Nginx..."
nginx -g "daemon off;" &
NGINX_PID=$!

# Monitoring loop
while true; do
  if ! kill -0 $NINEROUTER_PID 2>/dev/null; then
    echo "9router process died. Exiting..."
    exit 1
  fi
  if ! kill -0 $NGINX_PID 2>/dev/null; then
    echo "Nginx process died. Exiting..."
    exit 1
  fi
  sleep 10
done
EOF

RUN chmod +x /entrypoint.sh

EXPOSE 80
VOLUME ["/root/.9router"]
CMD ["/entrypoint.sh"]
