FROM node:20-bookworm-slim

# Use tini as init
ENTRYPOINT ["tini", "--"]

# Install dependencies
RUN apt-get update && apt-get install -y \
    procps \
    curl \
    tini \
    && rm -rf /var/lib/apt/lists/*

# Install 9router
RUN npm install -g 9router

# Create necessary directories
RUN mkdir -p /root/.9router

# Expose 9router default port
EXPOSE 20128

VOLUME ["/root/.9router"]

# Start 9router in non-interactive mode
# Redirecting stdin from /dev/null prevents interactive prompt hangs
CMD ["/bin/sh", "-c", "9router --no-browser --log --skip-update < /dev/null"]
