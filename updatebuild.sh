#!/bin/bash

# Remove the .next directory to clean up old build files
# sudo rm -rf ./.next/

# Pull the latest changes from the Git repository
git fetch origin
git pull origin main

# Install any new or updated dependencies using pnpm
pnpm install

# Run database migrations if needed (based on your turbo.json tasks)
pnpm run db:push
pnpm run db:migrate:deploy

# Build the Next.js project for production using turbo
pnpm run build

# Check if the build was successful
if [ $? -eq 0 ]; then
    # Restart the PM2 processes named "express" and "nextjs" and update the environment variables
    pm2 restart express --update-env
    pm2 restart nextjs --update-env

    # Execute commit.sh script
    bash commit.sh
else
    echo "Build failed. Skipping commit."
fi