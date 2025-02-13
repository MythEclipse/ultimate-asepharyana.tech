# Use a specific Node.js version based on Alpine for a smaller image size
FROM node:22-alpine AS builder

# Set working directory inside the container
WORKDIR /app

# Copy only the files needed for dependency installation
COPY package.json ./

# Install pnpm globally
RUN npm install -g pnpm

# Install dependencies, ensuring the lockfile is used for consistent versions
RUN pnpm install 

# Copy all files after dependencies are installed
COPY . .

# Build the Next.js application
RUN pnpm run build

# Use a lighter Node.js image for production
FROM node:22-alpine AS production

# Set working directory inside the container
WORKDIR /app

# Copy build files from the builder stage
COPY --from=builder /app/.next/ .next/
COPY --from=builder /app/public/ public/

# Copy only the files needed to run the application
COPY --from=builder /app/package.json ./

# Install pnpm and production dependencies only
RUN npm install -g pnpm && pnpm install --prod 

# Specify the port the application will use
EXPOSE 4090

# Run the application
CMD ["pnpm", "run", "start"]
