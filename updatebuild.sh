#!/bin/bash

# Remove the .next directory to clean up old build files
# sudo rm -rf ./.next/
# Check if nvm is installed
# if command -v nvm > /dev/null 2>&1; then
#     # Use Node.js version 22 with nvm
#     nvm use 22
# else
#     echo "nvm is not installed."
# fi
nvm use 22
bash sqlitereset.sh
bash cleanmodule.sh
# Pull the latest changes from the Git repository
git fetch origin
git pull origin main
# Load environment variables from .env file
# export $(grep -v '^#' .env | xargs)

# # Check if DATABASE_URL is present in /etc/environment, if not, add it
# if ! grep -q "DATABASE_URL=\"$DATABASE_URL\"" /etc/environment; then
#     echo "DATABASE_URL=\"$DATABASE_URL\"" | sudo tee -a /etc/environment
#     echo "DATABASE_URL added to /etc/environment"
# else
#     echo "DATABASE_URL already exists in /etc/environment"
# fi

# Remove any carriage return characters from /etc/environment
# sudo sed -i 's/\r$//' /etc/environment
# Install any new or updated dependencies using pnpm
bun install

# Run database migrations if needed (based on your turbo.json tasks)
bun run generate
# pnpm run db:push
# pnpm run db:migrate:deploy

# Build the Next.js project for production using turbo
bun run buildnc

# Check if the build was successful
if [ $? -eq 0 ]; then
    # Restart the PM2 processes named "express" and "nextjs" and update the environment variables
    pm2 restart express --update-env
    pm2 restart nextjs --update-env

    # Execute commit.sh script
    bash commit.sh
    bash rsync.sh
else
    echo "Build failed. Skipping commit."
fi
nvm use default
# if command -v nvm > /dev/null 2>&1; then
#     # Use Node.js version 22 with nvm
#     nvm use default
# else
#     echo "nvm is not installed."
# fi