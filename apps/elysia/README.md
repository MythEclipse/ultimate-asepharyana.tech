# ElysiaJS Auth API with Prisma

A high-performance authentication API built with [ElysiaJS](https://elysiajs.com/), Bun runtime, and Prisma ORM.

## Features

- ⚡ Ultra-fast performance with Bun runtime
- 🦊 ElysiaJS framework for building web servers
- 🗄️ Prisma ORM for type-safe database access
- 🔐 JWT authentication with refresh tokens
- 📧 Email verification system
- 🔄 Password reset functionality
- 🚀 Redis caching support
- 📚 Swagger/OpenAPI documentation
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
pnpm install
```

### Setup Database

1. **Configure environment variables:**

Create `.env` file (copy from `.env.example`):

```env
PORT=4092
NODE_ENV=development
DATABASE_URL="mysql://root:password@localhost:3306/elysia_auth"
JWT_SECRET=your-super-secret-jwt-key-minimum-32-characters-long
REDIS_URL=redis://localhost:6379
MINIO_ENDPOINT=localhost
MINIO_PORT=9000
MINIO_USE_SSL=false
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
MINIO_BUCKET=avatars
MINIO_PUBLIC_URL=http://localhost:9000
MINIO_AVATAR_PREFIX=avatars
```

2. **Create MySQL database:**

```sql
CREATE DATABASE elysia_auth;
```

3. **Generate Prisma Client:**

```bash
pnpm prisma:generate
```

4. **Run database migrations:**

```bash
pnpm prisma:migrate
```

5. **(Optional) Seed database:**

```bash
pnpm prisma:seed
```

📖 **See [PRISMA_SETUP.md](./PRISMA_SETUP.md) for detailed setup instructions**

### Development

Run the development server with hot reload:

```bash
# Using turbo
turbo run dev --filter=@asepharyana/elysia

# Or directly with bun
bun run dev
# or
pnpm dev
```

The server will start at `http://localhost:4092`

### View API Documentation

Open Swagger UI at:

```
http://localhost:4092/docs
```

### Prisma Studio (Database GUI)

```bash
pnpm prisma:studio
```

### Build

Build the application:

```bash
# Using turbo
turbo run build --filter=@asepharyana/elysia

# Or directly with bun
bun run build
# or
pnpm build
```

### Production

Start the production server:

```bash
bun run start
# or
pnpm start
```

## API Endpoints

### General

- `GET /` - Welcome message with API info
- `GET /health` - Health check endpoint with database status

### Demo Endpoints

- `GET /api/hello/:name` - Personalized greeting
- `POST /api/echo` - Echo back the request body
- `GET /api/users` - List all users (demo)
  - `POST /api/users/avatar` - Upload user avatar (multipart/form-data; requires auth; stores in MinIO)

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
curl http://localhost:4092/health

# Personalized greeting
curl http://localhost:4092/api/hello/World

# Echo endpoint
curl -X POST http://localhost:4092/api/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello"}'
```

## Testing

### Unit Tests

Run unit tests:

```bash
# Using turbo
turbo run test --filter=@asepharyana/elysia

# Or directly with bun
bun test
```

### API Integration Tests

Test all API endpoints automatically:

```bash
# Bash (Linux/Mac/Git Bash) - with running server
chmod +x test-all-api.sh
./test-all-api.sh

# Bash - auto-start server for testing
./test-all-api.sh -s

# PowerShell (Windows) - with running server
.\test-all-api.ps1

# PowerShell - auto-start server for testing
.\test-all-api.ps1 -s
```

**Options:**

- Without `-s`: Requires server to be already running on port 4092
- With `-s`: Automatically starts server, runs tests, then stops server

**Prerequisites for `-s` option:**

- Redis must be running (start with `docker run -d -p 6379:6379 redis` or local redis-server)
- Database must be accessible and migrated
- Port 4092 must be available

The test script will automatically:

- Test all health and basic endpoints
- Register a new user and login
- Test all authentication endpoints
- Test social media features (posts, comments, likes)
- Test chat features (rooms, messages)
- Clean up test data

For detailed testing documentation, see the test script comments.

## Learn More

- [ElysiaJS Documentation](https://elysiajs.com/)
- [Bun Documentation](https://bun.sh/docs)
