# ElysiaJS Application

A high-performance web server built with [ElysiaJS](https://elysiajs.com/) and Bun runtime.

## Features

- ⚡ Ultra-fast performance with Bun runtime
- 🦊 ElysiaJS framework for building web servers
- 🔥 Hot reload in development mode
- 📦 Built-in TypeScript support

## Prerequisites

Make sure you have [Bun](https://bun.sh/) installed:

```bash
# Windows (PowerShell)
powershell -c "irm bun.sh/install.ps1|iex"

# macOS/Linux
curl -fsSL https://bun.sh/install | bash
```

## Getting Started

### Install dependencies

```bash
bun install
```

### Development

Run the development server with hot reload:

```bash
# Using nx
nx serve elysia

# Or directly with bun
bun run dev
```

### Build

Build the application:

```bash
# Using nx
nx build elysia

# Or directly with bun
bun run build
```

### Production

Start the production server:

```bash
# Using nx
nx start elysia

# Or directly with bun
bun run start
```

## API Endpoints

### General
- `GET /` - Welcome message with API info
- `GET /health` - Health check endpoint with database status

### Demo Endpoints  
- `GET /api/hello/:name` - Personalized greeting
- `POST /api/echo` - Echo back the request body
- `GET /api/users` - List all users (demo)

### Authentication (Full System)
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login with email/username
- `GET /api/auth/me` - Get current user profile (requires auth)
- `POST /api/auth/logout` - Logout and blacklist token
- `GET /api/auth/verify` - Verify email address
- `POST /api/auth/forgot-password` - Request password reset
- `POST /api/auth/reset-password` - Reset password with token
- `POST /api/auth/refresh-token` - Refresh JWT access token

📖 **See [AUTH_README.md](./AUTH_README.md) for complete authentication documentation**

## Example Usage

```bash
# Health check
curl http://localhost:3001/health

# Personalized greeting
curl http://localhost:3001/api/hello/World

# Echo endpoint
curl -X POST http://localhost:3001/api/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello"}'
```

## Testing

Run tests:

```bash
# Using nx
nx test elysia

# Or directly with bun
bun test
```

## Learn More

- [ElysiaJS Documentation](https://elysiajs.com/)
- [Bun Documentation](https://bun.sh/docs)
