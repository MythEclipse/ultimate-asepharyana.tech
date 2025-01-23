# Remove the .next directory to clean up old build files
Remove-Item -Recurse -Force .\.next\ -ErrorAction SilentlyContinue

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
if ($LASTEXITCODE -eq 0) {
    # Restart the PM2 processes named "express" and "nextjs" and update the environment variables
    pm2 restart express --update-env
    pm2 restart nextjs --update-env

    # Execute commit.ps1 script
    .\commit.ps1
} else {
    Write-Host "Build failed. Skipping commit." -ForegroundColor Red
}