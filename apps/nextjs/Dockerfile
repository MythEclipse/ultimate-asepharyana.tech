# Use a Node.js base image with Bun
FROM oven/bun:1.2.18-alpine

# Set the working directory in the container
WORKDIR /app

# Copy package.json and bun.lock
COPY package.json bun.lock ./

# Install dependencies
RUN bun install --frozen-lockfile

# Copy the rest of the application code
COPY . .

# Build the Next.js application
RUN bun run nextjs:build

# Expose the port Next.js runs on
EXPOSE 4090

# Command to run the application
CMD ["bun", "start"]
