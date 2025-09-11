# Use a Node.js base image
FROM node:22-alpine

# Set the working directory in the container
WORKDIR /app

# Copy package.json and pnpm-lock.yaml
COPY package.json pnpm-lock.yaml ./

# Install pnpm globally
RUN npm install -g pnpm

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy the rest of the application code
COPY . .

# Build the Next.js application
RUN pnpm run nextjs:build

# Expose the port Next.js runs on
EXPOSE 4090

# Command to run the application
CMD ["pnpm", "start"]
