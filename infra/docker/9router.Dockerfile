FROM node:20-bookworm-slim

# Use tini as init - using absolute path for robustness
ENTRYPOINT ["/usr/bin/tini", "--"]

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

# Start 9router
# We use --no-browser and --log
# No shell redirection to avoid potential issues with tini/sh interaction
CMD ["9router", "--no-browser", "--log", "--skip-update"]
