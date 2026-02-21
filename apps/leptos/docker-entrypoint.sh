#!/bin/sh
# Adjust nginx listen port based on PORT environment variable (default 8081)
PORT=${PORT:-8081}

# Update default.conf only if the port is not 80
if [ "$PORT" != "80" ]; then
    sed -i "s/listen \(.*\)80;/listen ${PORT};/" /etc/nginx/conf.d/default.conf
fi

exec nginx -g 'daemon off;'
