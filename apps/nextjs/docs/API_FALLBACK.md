# API Fallback System Documentation

## Overview

Frontend Next.js menggunakan sistem fallback otomatis dengan 4 prioritas endpoint untuk memastikan aplikasi tetap berjalan meskipun salah satu backend tidak tersedia.

## Fallback Priority

### 1️⃣ **localhost:4091** (Rust Backend - Development)
- **Prioritas**: Primary
- **Teknologi**: Rust + Actix-web
- **Kecepatan**: Paling cepat
- **Use case**: Development lokal, production

### 2️⃣ **localhost:3002** (Elysia Backend - Development)
- **Prioritas**: Secondary fallback
- **Teknologi**: Bun + ElysiaJS
- **Kecepatan**: Cepat
- **Use case**: Development alternatif, testing

### 3️⃣ **ws.asepharyana.tech** (Production Rust)
- **Prioritas**: Tertiary fallback
- **Teknologi**: Rust + Actix-web (Production)
- **Kecepatan**: Cepat (dengan latency network)
- **Use case**: Production deployment Rust

### 4️⃣ **elysia.asepharyana.tech** (Production Elysia)
- **Prioritas**: Quaternary fallback (last resort)
- **Teknologi**: Bun + ElysiaJS (Production)
- **Kecepatan**: Cepat (dengan latency network)
- **Use case**: Production deployment Elysia

---

## Architecture

### Client-Side (Browser)
```
Browser Request → Try Priority 1 → Fail? → Try Priority 2 → Fail? → Try Priority 3 → Fail? → Try Priority 4
```

### Server-Side (Next.js SSR/API)
```
SSR Request → Try Priority 1 → Fail? → Try Priority 2 → Fail? → Try Priority 3 → Fail? → Try Priority 4
```

---

## Configuration

### Environment Variables

```bash
# .env.local atau .env.development

# Priority 1: Rust Development
NEXT_PUBLIC_API_URL_1=http://localhost:4091
API_URL_SERVER_1=http://localhost:4091

# Priority 2: Elysia Development  
NEXT_PUBLIC_API_URL_2=http://localhost:3002
API_URL_SERVER_2=http://localhost:3002

# Priority 3: Production Rust
NEXT_PUBLIC_API_URL_3=https://ws.asepharyana.tech
API_URL_SERVER_3=https://ws.asepharyana.tech

# Priority 4: Production Elysia
NEXT_PUBLIC_API_URL_4=https://elysia.asepharyana.tech
API_URL_SERVER_4=https://elysia.asepharyana.tech
```

### Files Modified

#### 1. `utils/url-utils.ts`
```typescript
export const API_FALLBACK_URLS = {
  client: [
    'http://localhost:4091',      // Rust
    'http://localhost:3002',      // Elysia
    'https://ws.asepharyana.tech', // Production Rust
    'https://elysia.asepharyana.tech', // Production Elysia
  ],
  server: [
    'http://localhost:4091',
    'http://localhost:3002',
    'https://ws.asepharyana.tech',
    'https://elysia.asepharyana.tech',
  ],
};
```

#### 2. `utils/unified-http-client.ts`
- Added `executeRequestWithFallback()` method
- Automatically tries all endpoints in order
- Timeout: 5 seconds per endpoint
- Logs success/failure for each attempt

#### 3. `lib/chat-api.ts`
- All API calls use `fetchWithFallback()`
- WebSocket connections have fallback mechanism
- Auto-reconnect dengan fallback URL

---

## How It Works

### HTTP/REST API Fallback

```typescript
// Example: Fetching posts
const response = await fetchData('/api/sosmed/posts');

// Internal behavior:
// 1. Try http://localhost:4091/api/sosmed/posts (5s timeout)
// 2. If fail, try http://localhost:3002/api/sosmed/posts (5s timeout)
// 3. If fail, try https://ws.asepharyana.tech/api/sosmed/posts (5s timeout)
// 4. If fail, try https://elysia.asepharyana.tech/api/sosmed/posts (5s timeout)
// 5. If all fail, throw error
```

### WebSocket Fallback

```typescript
// Example: Chat connection
const ws = connectWebSocket(handleMessage);

// Internal behavior:
// 1. Try ws://localhost:4091/ws/chat
// 2. On error/close, try ws://localhost:3002/ws/chat (after 2s)
// 3. On error/close, try wss://ws.asepharyana.tech/ws/chat (after 2s)
// 4. On error/close, try wss://elysia.asepharyana.tech/ws/chat (after 2s)
// 5. If all fail, stop reconnecting
```

---

## API Endpoints Affected

### Social Media API (`/api/sosmed`)
- ✅ `GET /api/sosmed/posts` - Get all posts
- ✅ `POST /api/sosmed/posts` - Create post
- ✅ `PUT /api/sosmed/posts/:id` - Update post
- ✅ `DELETE /api/sosmed/posts/:id` - Delete post
- ✅ `POST /api/sosmed/posts/:id/comments` - Add comment
- ✅ `PUT /api/sosmed/comments/:id` - Update comment
- ✅ `DELETE /api/sosmed/comments/:id` - Delete comment
- ✅ `POST /api/sosmed/posts/:id/like` - Like post
- ✅ `DELETE /api/sosmed/posts/:id/like` - Unlike post

### Chat API (`/api/chat`)
- ✅ `GET /api/chat/rooms` - Get all rooms
- ✅ `POST /api/chat/rooms` - Create room
- ✅ `POST /api/chat/rooms/:roomId/join` - Join room
- ✅ `POST /api/chat/rooms/:roomId/leave` - Leave room
- ✅ `GET /api/chat/rooms/:roomId/messages` - Get messages
- ✅ `POST /api/chat/rooms/:roomId/messages` - Send message
- ✅ `DELETE /api/chat/messages/:messageId` - Delete message

### Authentication API (`/api/auth`)
- ✅ All auth endpoints (login, register, etc.)

---

## Monitoring & Debugging

### Console Logs

```javascript
// Success log
[API Fallback] Trying http://localhost:4091 (1/4)
[API Fallback] Success with http://localhost:4091

// Failure with fallback log
[API Fallback] Trying http://localhost:4091 (1/4)
[API Fallback] Failed with http://localhost:4091: Connection refused
[API Fallback] Trying http://localhost:3002 (2/4)
[API Fallback] Success with http://localhost:3002

// WebSocket logs
[WebSocket] Trying ws://localhost:4091/ws/chat (1/4)
[WebSocket] Connected to ws://localhost:4091/ws/chat
```

### Error Handling

Jika semua endpoints gagal:
```javascript
Error: All API endpoints failed
```

---

## Testing Scenarios

### Scenario 1: Rust Backend Only
```bash
# Start Rust backend
cd apps/rust
cargo run

# Next.js will use localhost:4091
# ✅ All requests go to Rust
```

### Scenario 2: Elysia Backend Only
```bash
# Start Elysia backend
cd apps/elysia
pnpm dev

# Rust is not running
# ✅ All requests fallback to localhost:3002 (Elysia)
```

### Scenario 3: Both Running (Development)
```bash
# Start Rust
cd apps/rust
cargo run

# Start Elysia
cd apps/elysia  
pnpm dev

# ✅ Rust gets priority (faster)
# ✅ If Rust fails, automatically switches to Elysia
```

### Scenario 4: Production Only
```bash
# No local backends running
# ✅ Automatically uses ws.asepharyana.tech
# ✅ If fails, uses elysia.asepharyana.tech
```

---

## Performance Metrics

| Endpoint | Timeout | Retry | Total Max Time |
|----------|---------|-------|----------------|
| Priority 1 | 5s | No | 5s |
| Priority 2 | 5s | No | 10s (cumulative) |
| Priority 3 | 5s | No | 15s (cumulative) |
| Priority 4 | 5s | No | 20s (cumulative) |

**Best Case**: 50-200ms (local Rust)  
**Worst Case**: 20s (all endpoints timeout)

---

## Benefits

✅ **High Availability**: Aplikasi tetap berjalan meskipun 1 backend down  
✅ **Zero Configuration**: Fallback otomatis tanpa perlu restart  
✅ **Development Friendly**: Bisa develop dengan Rust atau Elysia  
✅ **Production Ready**: Automatic failover ke production endpoints  
✅ **Performance**: Prioritas ke backend tercepat (Rust)  
✅ **Debugging**: Clear console logs untuk troubleshooting  

---

## Future Improvements

- [ ] Health check endpoint untuk pre-validate availability
- [ ] Circuit breaker pattern untuk skip failed endpoints temporarily
- [ ] Load balancing antar production endpoints
- [ ] Metrics dashboard untuk monitoring fallback frequency
- [ ] Automatic priority reordering based on performance

---

## Related Documentation

- [Backend Elysia API Documentation](../../elysia/SOSMED_CHAT_API.md)
- [Backend Rust API Documentation](../../rust/README.md)
- [Environment Variables](./.env.example)

---

**Last Updated**: November 1, 2025  
**Maintainer**: Development Team
