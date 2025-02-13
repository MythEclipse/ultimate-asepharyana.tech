# Use the official Node.js image as the base image
FROM node:22-alpine

# Set the working directory
WORKDIR /app

# Copy package.json and pnpm-lock.yaml files
COPY package.json pnpm-lock.yaml ./

# Install pnpm
RUN npm install -g pnpm turbo

# Install dependencies with prefer offline
RUN pnpm install --prefer-offline --frozen-lockfile

# Copy the rest of the application code
COPY . .


# Build the project using Turbo
RUN pnpm run build

# Expose the port the app runs on
EXPOSE 3000

# Start the application
CMD ["pnpm", "start"]