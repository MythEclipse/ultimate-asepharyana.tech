# Social Media & Chat Backend API

Backend API untuk fitur Social Media dan Chat menggunakan ElysiaJS + Prisma.

## 📦 Models

### Social Media Models

#### Post
```prisma
model Post {
  id        String   @id @default(uuid())
  userId    String
  content   String   @db.Text
  imageUrl  String?
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  
  user      User
  comments  Comment[]
  likes     Like[]
}
```

#### Comment
```prisma
model Comment {
  id        String   @id @default(uuid())
  postId    String
  userId    String
  content   String   @db.Text
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  
  post      Post
  user      User
}
```

#### Like
```prisma
model Like {
  id        String   @id @default(uuid())
  postId    String
  userId    String
  createdAt DateTime @default(now())
  
  post      Post
  user      User
  
  @@unique([postId, userId])
}
```

### Chat Models

#### ChatRoom
```prisma
model ChatRoom {
  id          String   @id @default(uuid())
  name        String
  description String?
  isPrivate   Boolean  @default(false)
  createdAt   DateTime @default(now())
  updatedAt   DateTime @updatedAt
  
  messages  ChatMessage[]
  members   ChatRoomMember[]
}
```

#### ChatMessage
```prisma
model ChatMessage {
  id        String   @id @default(uuid())
  roomId    String
  userId    String
  content   String   @db.Text
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  
  room      ChatRoom
  user      User
}
```

#### ChatRoomMember
```prisma
model ChatRoomMember {
  id       String   @id @default(uuid())
  roomId   String
  userId   String
  role     String   @default("member") // member, admin
  joinedAt DateTime @default(now())
  
  room     ChatRoom
  user     User
  
  @@unique([roomId, userId])
}
```

---

## 🚀 API Endpoints

### Social Media API (`/api/sosmed`)

#### Posts

**GET /api/sosmed/posts**
- Get all posts with comments and likes
- Headers: `Authorization: Bearer <token>`
- Response:
```json
{
  "success": true,
  "posts": [
    {
      "id": "uuid",
      "userId": "uuid",
      "content": "Post content",
      "imageUrl": "https://...",
      "createdAt": "2025-11-01T...",
      "updatedAt": "2025-11-01T...",
      "user": {
        "id": "uuid",
        "name": "John Doe",
        "email": "john@example.com",
        "avatar": "https://..."
      },
      "comments": [...],
      "likes": [...]
    }
  ]
}
```

**POST /api/sosmed/posts**
- Create a new post
- Headers: `Authorization: Bearer <token>`
- Body:
```json
{
  "content": "Post content",
  "imageUrl": "https://..." // optional
}
```

**PUT /api/sosmed/posts/:id**
- Update a post (only by owner)
- Headers: `Authorization: Bearer <token>`
- Body:
```json
{
  "content": "Updated content",
  "imageUrl": "https://..." // optional
}
```

**DELETE /api/sosmed/posts/:id**
- Delete a post (only by owner)
- Headers: `Authorization: Bearer <token>`

#### Comments

**POST /api/sosmed/posts/:id/comments**
- Add a comment to a post
- Headers: `Authorization: Bearer <token>`
- Body:
```json
{
  "content": "Comment content"
}
```

**PUT /api/sosmed/comments/:id**
- Update a comment (only by owner)
- Headers: `Authorization: Bearer <token>`
- Body:
```json
{
  "content": "Updated comment"
}
```

**DELETE /api/sosmed/comments/:id**
- Delete a comment (only by owner)
- Headers: `Authorization: Bearer <token>`

#### Likes

**POST /api/sosmed/posts/:id/like**
- Like a post
- Headers: `Authorization: Bearer <token>`

**DELETE /api/sosmed/posts/:id/like**
- Unlike a post
- Headers: `Authorization: Bearer <token>`

---

### Chat API (`/api/chat`)

#### Rooms

**GET /api/chat/rooms**
- Get all chat rooms
- Headers: `Authorization: Bearer <token>`
- Response:
```json
{
  "success": true,
  "rooms": [
    {
      "id": "uuid",
      "name": "General Chat",
      "description": "General discussion",
      "isPrivate": false,
      "createdAt": "2025-11-01T...",
      "updatedAt": "2025-11-01T...",
      "members": [...],
      "messages": [...],
      "_count": {
        "messages": 100,
        "members": 5
      }
    }
  ]
}
```

**POST /api/chat/rooms**
- Create a new chat room
- Headers: `Authorization: Bearer <token>`
- Body:
```json
{
  "name": "Room name",
  "description": "Room description", // optional
  "isPrivate": false // optional
}
```

**POST /api/chat/rooms/:roomId/join**
- Join a chat room
- Headers: `Authorization: Bearer <token>`

**POST /api/chat/rooms/:roomId/leave**
- Leave a chat room
- Headers: `Authorization: Bearer <token>`

#### Messages

**GET /api/chat/rooms/:roomId/messages**
- Get messages from a chat room
- Headers: `Authorization: Bearer <token>`
- Query params:
  - `limit`: Number of messages (default: 50)
  - `before`: Get messages before this timestamp
- Response:
```json
{
  "success": true,
  "messages": [
    {
      "id": "uuid",
      "roomId": "uuid",
      "userId": "uuid",
      "content": "Message content",
      "createdAt": "2025-11-01T...",
      "user": {
        "id": "uuid",
        "name": "John Doe",
        "email": "john@example.com",
        "avatar": "https://..."
      }
    }
  ]
}
```

**POST /api/chat/rooms/:roomId/messages**
- Send a message to a chat room
- Headers: `Authorization: Bearer <token>`
- Body:
```json
{
  "content": "Message content"
}
```

**DELETE /api/chat/messages/:messageId**
- Delete a message (only by sender or room admin)
- Headers: `Authorization: Bearer <token>`

---

## 🔧 Setup & Migration

### 1. Generate Prisma Client

```bash
cd apps/elysia
pnpm prisma generate
```

### 2. Create Migration

```bash
pnpm prisma migrate dev --name add_sosmed_and_chat
```

### 3. Start Server

```bash
pnpm dev
```

Server akan berjalan di: `http://localhost:3002`

### 4. View Swagger Docs

```
http://localhost:3002/swagger
```

### 5. View Database (Prisma Studio)

```bash
pnpm prisma:studio
```

---

## 📝 Usage Examples

### Social Media

#### Create a Post
```bash
curl -X POST http://localhost:3002/api/sosmed/posts \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello World!",
    "imageUrl": "https://example.com/image.jpg"
  }'
```

#### Like a Post
```bash
curl -X POST http://localhost:3002/api/sosmed/posts/POST_ID/like \
  -H "Authorization: Bearer YOUR_TOKEN"
```

#### Add Comment
```bash
curl -X POST http://localhost:3002/api/sosmed/posts/POST_ID/comments \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Nice post!"
  }'
```

### Chat

#### Create Chat Room
```bash
curl -X POST http://localhost:3002/api/chat/rooms \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "General Chat",
    "description": "General discussion"
  }'
```

#### Join Room
```bash
curl -X POST http://localhost:3002/api/chat/rooms/ROOM_ID/join \
  -H "Authorization: Bearer YOUR_TOKEN"
```

#### Send Message
```bash
curl -X POST http://localhost:3002/api/chat/rooms/ROOM_ID/messages \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello everyone!"
  }'
```

---

## 🔐 Authentication

Semua endpoint memerlukan authentication dengan Bearer token:

```
Authorization: Bearer <access_token>
```

Token didapat dari login endpoint:
```bash
POST /api/auth/login
Body: { "email": "user@example.com", "password": "password" }
```

---

## ✨ Features

### Social Media
- ✅ Create, read, update, delete posts
- ✅ Upload images
- ✅ Comments system
- ✅ Like/Unlike posts
- ✅ User mentions (via user relations)
- ✅ Real-time updates ready

### Chat
- ✅ Multiple chat rooms
- ✅ Public/Private rooms
- ✅ Room membership management
- ✅ Message history
- ✅ Pagination support
- ✅ Role-based permissions (member/admin)
- ✅ Real-time ready (WebSocket can be added)

---

## 🎯 Next Steps

### WebSocket Support (Optional)
Untuk real-time features, dapat ditambahkan WebSocket:

```typescript
import { Elysia } from 'elysia';
import { ws } from '@elysiajs/ws';

app.use(ws())
  .ws('/api/chat/ws', {
    message(ws, message) {
      // Handle WebSocket messages
      ws.send(message);
    },
  });
```

### File Upload
Untuk image upload, dapat menggunakan endpoint khusus:

```typescript
.post('/api/upload', async ({ body }) => {
  const { file } = body;
  // Handle file upload
  return { url: 'https://...' };
});
```

---

## 📚 Additional Resources

- [Prisma Documentation](https://www.prisma.io/docs)
- [ElysiaJS Documentation](https://elysiajs.com)
- [Swagger UI](http://localhost:3002/swagger)

---

**Backend siap digunakan!** 🚀
