# 📚 API Documentation - Elysia Backend

## Base URL
```
https://elysia.asepharyana.tech
```

## Authentication
Semua endpoint yang memerlukan autentikasi menggunakan **Bearer Token** di header:
```
Authorization: Bearer <JWT_TOKEN>
```

---

## 📁 API Routes

### 1. **Users API** (`/api/users`)

#### 1.1 Get All Users
```http
GET /api/users
```

**Response Success (200):**
```typescript
{
  success: boolean;
  count: number;
  users: Array<{
    id: string;
    email: string | null;
    name: string | null;
    emailVerified: Date | null;
    image: string | null;
    role: string;
  }>;
}
```

**Example:**
```json
{
  "success": true,
  "count": 10,
  "users": [
    {
      "id": "user_123",
      "email": "john@example.com",
      "name": "John Doe",
      "emailVerified": "2024-01-15T10:30:00Z",
      "image": "https://example.com/avatar.jpg",
      "role": "user"
    }
  ]
}
```

---

#### 1.2 Get User by ID
```http
GET /api/users/:id
```

**Parameters:**
- `id` (path) - User ID

**Response Success (200):**
```typescript
{
  success: boolean;
  user: {
    id: string;
    email: string | null;
    name: string | null;
    emailVerified: Date | null;
    image: string | null;
    role: string;
  };
}
```

**Response Error (404):**
```json
{
  "success": false,
  "error": "User not found"
}
```

---

## 📱 Social Media API (`/api/sosmed`)

### 2. **Posts**

#### 2.1 Get All Posts
```http
GET /api/sosmed/posts
```

**Headers:**
```
Authorization: Bearer <token>
```

**Response Success (200):**
```typescript
{
  success: boolean;
  posts: Array<{
    id: string;
    userId: string;
    content: string;
    image_url: string | null;
    created_at: Date;
    user: {
      id: string;
      name: string | null;
      email: string | null;
      image: string | null;
    };
    comments: Array<Comment>;
    likes: Array<Like>;
  }>;
}
```

---

#### 2.2 Create Post
```http
POST /api/sosmed/posts
```

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```typescript
{
  content: string;        // Required if no imageUrl
  imageUrl?: string;      // Optional
}
```

**Example:**
```json
{
  "content": "Hello World! This is my first post.",
  "imageUrl": "https://example.com/image.jpg"
}
```

**Response Success (200):**
```typescript
{
  success: boolean;
  post: {
    id: string;
    userId: string;
    content: string;
    image_url: string | null;
    created_at: Date;
    user: UserInfo;
    comments: Comment[];
    likes: Like[];
  };
}
```

**Response Error (400):**
```json
{
  "success": false,
  "error": "Content or image is required"
}
```

**Response Error (401):**
```json
{
  "success": false,
  "error": "Unauthorized"
}
```

---

#### 2.3 Update Post
```http
PUT /api/sosmed/posts/:id
```

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Parameters:**
- `id` (path) - Post ID

**Request Body:**
```typescript
{
  content: string;
  imageUrl?: string;
}
```

**Response Success (200):**
```typescript
{
  success: boolean;
  post: PostWithRelations;
}
```

**Response Error (403):**
```json
{
  "success": false,
  "error": "Not authorized to edit this post"
}
```

**Response Error (404):**
```json
{
  "success": false,
  "error": "Post not found"
}
```

---

#### 2.4 Delete Post
```http
DELETE /api/sosmed/posts/:id
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `id` (path) - Post ID

**Response Success (200):**
```json
{
  "success": true,
  "message": "Post deleted successfully"
}
```

**Response Error (403):**
```json
{
  "success": false,
  "error": "Not authorized to delete this post"
}
```

---

### 3. **Comments**

#### 3.1 Add Comment to Post
```http
POST /api/sosmed/posts/:id/comments
```

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Parameters:**
- `id` (path) - Post ID

**Request Body:**
```typescript
{
  content: string;  // Required
}
```

**Example:**
```json
{
  "content": "Great post! Thanks for sharing."
}
```

**Response Success (200):**
```typescript
{
  success: boolean;
  comment: {
    id: string;
    postId: string;
    userId: string;
    content: string;
    created_at: Date;
    user: {
      id: string;
      name: string | null;
      email: string | null;
      image: string | null;
    };
  };
}
```

**Response Error (400):**
```json
{
  "success": false,
  "error": "Comment content is required"
}
```

---

#### 3.2 Update Comment
```http
PUT /api/sosmed/comments/:id
```

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Parameters:**
- `id` (path) - Comment ID

**Request Body:**
```typescript
{
  content: string;
}
```

**Response Success (200):**
```typescript
{
  success: boolean;
  comment: CommentWithUser;
}
```

**Response Error (403):**
```json
{
  "success": false,
  "error": "Not authorized to edit this comment"
}
```

---

#### 3.3 Delete Comment
```http
DELETE /api/sosmed/comments/:id
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `id` (path) - Comment ID

**Response Success (200):**
```json
{
  "success": true,
  "message": "Comment deleted successfully"
}
```

---

### 4. **Likes**

#### 4.1 Like a Post
```http
POST /api/sosmed/posts/:id/like
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `id` (path) - Post ID

**Response Success (200):**
```typescript
{
  success: boolean;
  like: {
    postId: string;
    userId: string;
    user: {
      id: string;
      name: string | null;
      email: string | null;
    };
  };
}
```

**Response Error (400):**
```json
{
  "success": false,
  "error": "Post already liked"
}
```

---

#### 4.2 Unlike a Post
```http
DELETE /api/sosmed/posts/:id/like
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `id` (path) - Post ID

**Response Success (200):**
```json
{
  "success": true,
  "message": "Post unliked successfully"
}
```

**Response Error (404):**
```json
{
  "success": false,
  "error": "Like not found"
}
```

---

## 💬 Chat API (`/api/chat`)

### 5. **Chat Rooms**

#### 5.1 Get All Chat Rooms
```http
GET /api/chat/rooms
```

**Headers:**
```
Authorization: Bearer <token>
```

**Response Success (200):**
```typescript
{
  success: boolean;
  rooms: Array<{
    id: string;
    name: string;
    description: string | null;
    isPrivate: number;
    createdAt: Date;
    updatedAt: Date;
    members: Array<{
      id: string;
      roomId: string;
      userId: string;
      role: 'admin' | 'member';
      user: UserInfo;
    }>;
    messages: Array<MessageWithUser>;
  }>;
}
```

---

#### 5.2 Create Chat Room
```http
POST /api/chat/rooms
```

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```typescript
{
  name: string;           // Required
  description?: string;   // Optional
  isPrivate?: boolean;    // Optional, default: false
}
```

**Example:**
```json
{
  "name": "Team Discussion",
  "description": "Our team's main chat room",
  "isPrivate": false
}
```

**Response Success (200):**
```typescript
{
  success: boolean;
  room: {
    id: string;
    name: string;
    description: string | null;
    isPrivate: number;
    createdAt: Date;
    updatedAt: Date;
    members: MemberWithUser[];
  };
}
```

**Response Error (400):**
```json
{
  "success": false,
  "error": "Room name is required"
}
```

---

#### 5.3 Get Room Messages
```http
GET /api/chat/rooms/:roomId/messages
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `roomId` (path) - Room ID

**Query Parameters:**
- `limit` (optional) - Number of messages to fetch (default: 50)
- `before` (optional) - Get messages before this date (ISO string)

**Example:**
```
GET /api/chat/rooms/room_123/messages?limit=20&before=2024-01-15T10:30:00Z
```

**Response Success (200):**
```typescript
{
  success: boolean;
  messages: Array<{
    id: string;
    roomId: string;
    userId: string;
    content: string;
    createdAt: Date;
    user: UserInfo;
  }>;
}
```

**Response Error (403):**
```json
{
  "success": false,
  "error": "Not a member of this chat room"
}
```

---

#### 5.4 Send Message to Room
```http
POST /api/chat/rooms/:roomId/messages
```

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Parameters:**
- `roomId` (path) - Room ID

**Request Body:**
```typescript
{
  content: string;  // Required
}
```

**Example:**
```json
{
  "content": "Hello everyone!"
}
```

**Response Success (200):**
```typescript
{
  success: boolean;
  message: {
    id: string;
    roomId: string;
    userId: string;
    content: string;
    createdAt: Date;
    user: UserInfo;
  };
}
```

**Response Error (400):**
```json
{
  "success": false,
  "error": "Message content is required"
}
```

**Response Error (403):**
```json
{
  "success": false,
  "error": "Not a member of this chat room"
}
```

---

#### 5.5 Join Chat Room
```http
POST /api/chat/rooms/:roomId/join
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `roomId` (path) - Room ID

**Response Success (200):**
```typescript
{
  success: boolean;
  message: string;
  member: {
    id: string;
    roomId: string;
    userId: string;
    role: 'member';
    user: UserInfo;
  };
}
```

**Response Error (404):**
```json
{
  "success": false,
  "error": "Chat room not found"
}
```

---

#### 5.6 Leave Chat Room
```http
POST /api/chat/rooms/:roomId/leave
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `roomId` (path) - Room ID

**Response Success (200):**
```json
{
  "success": true,
  "message": "Left chat room successfully"
}
```

**Response Error (404):**
```json
{
  "success": false,
  "error": "Not a member of this chat room"
}
```

---

#### 5.7 Delete Message
```http
DELETE /api/chat/messages/:messageId
```

**Headers:**
```
Authorization: Bearer <token>
```

**Parameters:**
- `messageId` (path) - Message ID

**Response Success (200):**
```json
{
  "success": true,
  "message": "Message deleted successfully"
}
```

**Response Error (403):**
```json
{
  "success": false,
  "error": "Not authorized to delete this message"
}
```

**Response Error (404):**
```json
{
  "success": false,
  "error": "Message not found"
}
```

---

## 🎮 Quiz Battle WebSocket API (`/api/quiz/battle`)

### WebSocket Connection
```javascript
const ws = new WebSocket('wss://elysia.asepharyana.tech/api/quiz/battle');
```

### Message Format
```typescript
interface WSMessage {
  type: string;
  payload: any;
}
```

### 6. **Authentication & Connection**

#### 6.1 Connect
```typescript
// Send
{
  type: 'auth:connect',
  payload: {
    userId: string;
    token: string;
  }
}

// Receive
{
  type: 'auth:connected',
  payload: {
    sessionId: string;
    userId: string;
    timestamp: number;
  }
}
```

#### 6.2 Update Status
```typescript
// Send
{
  type: 'user.status.update',
  payload: {
    userId: string;
    status: 'online' | 'away' | 'offline';
  }
}
```

#### 6.3 Ping
```typescript
// Send
{
  type: 'connection.ping',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'connection.pong',
  payload: {
    timestamp: number;
  }
}
```

---

### 7. **Matchmaking**

#### 7.1 Find Match
```typescript
// Send
{
  type: 'matchmaking.find',
  payload: {
    userId: string;
    gameMode: 'quick' | 'ranked' | 'custom';
    difficulty?: 'easy' | 'medium' | 'hard';
    category?: string;
  }
}

// Receive - Searching
{
  type: 'matchmaking.searching',
  payload: {
    userId: string;
    queuePosition: number;
    estimatedWaitTime: number;
  }
}

// Receive - Match Found
{
  type: 'matchmaking.match_found',
  payload: {
    matchId: string;
    opponent: {
      userId: string;
      username: string;
      level: number;
      avatar?: string;
    };
    gameSettings: {
      difficulty: string;
      category: string;
      totalQuestions: number;
      timePerQuestion: number;
    };
  }
}
```

#### 7.2 Cancel Matchmaking
```typescript
// Send
{
  type: 'matchmaking.cancel',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'matchmaking.cancelled',
  payload: {
    userId: string;
  }
}
```

---

### 8. **Game**

#### 8.1 Submit Answer
```typescript
// Send
{
  type: 'game.answer.submit',
  payload: {
    userId: string;
    matchId: string;
    questionId: string;
    answer: string;
    timeSpent: number;
  }
}

// Receive
{
  type: 'game.answer.result',
  payload: {
    isCorrect: boolean;
    correctAnswer: string;
    points: number;
    timeBonus: number;
  }
}
```

---

### 9. **Friends System**

#### 9.1 Send Friend Request
```typescript
// Send
{
  type: 'friend.request.send',
  payload: {
    senderId: string;
    targetUsername: string;
  }
}

// Receive
{
  type: 'friend.request.sent',
  payload: {
    requestId: string;
    targetUser: UserInfo;
    timestamp: number;
  }
}
```

#### 9.2 Accept Friend Request
```typescript
// Send
{
  type: 'friend.request.accept',
  payload: {
    userId: string;
    requestId: string;
  }
}

// Receive
{
  type: 'friend.request.accepted',
  payload: {
    friendship: {
      friendshipId: string;
      user1: UserInfo;
      user2: UserInfo;
      createdAt: number;
    };
  }
}
```

#### 9.3 Reject Friend Request
```typescript
// Send
{
  type: 'friend.request.reject',
  payload: {
    userId: string;
    requestId: string;
  }
}

// Receive
{
  type: 'friend.request.rejected',
  payload: {
    requestId: string;
  }
}
```

#### 9.4 Remove Friend
```typescript
// Send
{
  type: 'friend.remove',
  payload: {
    userId: string;
    friendId: string;
  }
}

// Receive
{
  type: 'friend.removed',
  payload: {
    friendId: string;
  }
}
```

#### 9.5 Get Friend List
```typescript
// Send
{
  type: 'friend.list.request',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'friend.list.data',
  payload: {
    friends: Array<{
      userId: string;
      username: string;
      level: number;
      avatar?: string;
      status: 'online' | 'away' | 'offline';
      wins: number;
      losses: number;
    }>;
    pendingRequests: Array<FriendRequest>;
    totalFriends: number;
  }
}
```

#### 9.6 Challenge Friend
```typescript
// Send
{
  type: 'friend.challenge',
  payload: {
    challengerId: string;
    targetFriendId: string;
    gameSettings: GameSettings;
  }
}

// Receive
{
  type: 'friend.challenge.sent',
  payload: {
    challengeId: string;
    targetFriend: UserInfo;
  }
}
```

---

### 10. **Leaderboard**

#### 10.1 Get Global Leaderboard
```typescript
// Send
{
  type: 'leaderboard.global.sync',
  payload: {
    userId: string;
    limit?: number;
    offset?: number;
  }
}

// Receive
{
  type: 'leaderboard.global.data',
  payload: {
    leaderboard: Array<{
      rank: number;
      userId: string;
      username: string;
      level: number;
      points: number;
      wins: number;
      losses: number;
      avatar?: string;
    }>;
    userRank: number;
    totalPlayers: number;
  }
}
```

#### 10.2 Get Friends Leaderboard
```typescript
// Send
{
  type: 'leaderboard.friends.sync',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'leaderboard.friends.data',
  payload: {
    leaderboard: LeaderboardEntry[];
    userRank: number;
    totalFriends: number;
  }
}
```

---

### 11. **Lobby System**

#### 11.1 Create Lobby
```typescript
// Send
{
  type: 'lobby.create',
  payload: {
    hostId: string;
    maxPlayers: number;
    isPrivate: boolean;
    gameSettings: {
      difficulty: 'easy' | 'medium' | 'hard';
      category: string;
      totalQuestions: number;
      timePerQuestion: number;
    };
  }
}

// Receive
{
  type: 'lobby.created',
  payload: {
    lobbyId: string;
    lobbyCode: string;
    hostId: string;
    maxPlayers: number;
    gameSettings: GameSettings;
  }
}
```

#### 11.2 Join Lobby
```typescript
// Send
{
  type: 'lobby.join',
  payload: {
    userId: string;
    lobbyCode: string;
  }
}

// Receive
{
  type: 'lobby.player.joined',
  payload: {
    lobbyId: string;
    player: {
      userId: string;
      username: string;
      level: number;
      isReady: boolean;
    };
    players: LobbyPlayer[];
  }
}
```

#### 11.3 Ready/Unready
```typescript
// Send
{
  type: 'lobby.ready',
  payload: {
    userId: string;
    lobbyId: string;
    isReady: boolean;
  }
}

// Receive
{
  type: 'lobby.player.ready',
  payload: {
    userId: string;
    isReady: boolean;
    allPlayersReady: boolean;
  }
}
```

#### 11.4 Start Game
```typescript
// Send
{
  type: 'lobby.start',
  payload: {
    hostId: string;
    lobbyId: string;
  }
}

// Receive
{
  type: 'lobby.game.starting',
  payload: {
    lobbyId: string;
    countdown: number;
  }
}
```

#### 11.5 Leave Lobby
```typescript
// Send
{
  type: 'lobby.leave',
  payload: {
    userId: string;
    lobbyId: string;
  }
}
```

#### 11.6 Kick Player
```typescript
// Send
{
  type: 'lobby.kick',
  payload: {
    hostId: string;
    lobbyId: string;
    targetUserId: string;
  }
}
```

#### 11.7 Get Lobby List
```typescript
// Send
{
  type: 'lobby.list.sync',
  payload: {}
}

// Receive
{
  type: 'lobby.list.data',
  payload: {
    lobbies: Array<{
      lobbyId: string;
      lobbyCode: string;
      hostName: string;
      currentPlayers: number;
      maxPlayers: number;
      isPrivate: boolean;
      gameSettings: GameSettings;
    }>;
  }
}
```

---

### 12. **Chat (In-Game)**

#### 12.1 Send Global Chat
```typescript
// Send
{
  type: 'chat:global:send',
  payload: {
    userId: string;
    message: string;
  }
}

// Receive
{
  type: 'chat:global:message',
  payload: {
    messageId: string;
    sender: {
      userId: string;
      username: string;
      level: number;
      avatarUrl?: string;
    };
    message: string;
    timestamp: number;
  }
}
```

#### 12.2 Send Private Chat
```typescript
// Send
{
  type: 'chat:private:send',
  payload: {
    senderId: string;
    receiverId: string;
    message: string;
    timestamp: number;
  }
}

// Receive
{
  type: 'chat:private:message',
  payload: {
    messageId: string;
    conversationId: string;
    sender: {
      userId: string;
      username: string;
      status: 'online' | 'away' | 'offline';
    };
    message: string;
    timestamp: number;
    isRead: boolean;
  }
}
```

#### 12.3 Get Chat History
```typescript
// Send
{
  type: 'chat:history:sync',
  payload: {
    userId: string;
    targetUserId?: string;  // For private chat
    limit?: number;
    offset?: number;
  }
}

// Receive
{
  type: 'chat:history:data',
  payload: {
    messages: ChatMessage[];
    totalMessages: number;
    hasMore: boolean;
  }
}
```

#### 12.4 Typing Indicator
```typescript
// Send
{
  type: 'chat:typing',
  payload: {
    userId: string;
    targetUserId?: string;  // For private chat
    isTyping: boolean;
  }
}

// Receive
{
  type: 'chat:typing:indicator',
  payload: {
    userId: string;
    username: string;
    isTyping: boolean;
  }
}
```

#### 12.5 Mark as Read
```typescript
// Send
{
  type: 'chat:mark:read',
  payload: {
    userId: string;
    targetUserId: string;
  }
}
```

---

### 13. **Notifications**

#### 13.1 Get Notification List
```typescript
// Send
{
  type: 'notification.list.sync',
  payload: {
    userId: string;
    unreadOnly?: boolean;
  }
}

// Receive
{
  type: 'notification.list.data',
  payload: {
    notifications: Array<{
      notificationId: string;
      type: 'friend_request' | 'achievement' | 'challenge' | 'system';
      title: string;
      message: string;
      data: object;
      priority: 'low' | 'medium' | 'high';
      isRead: boolean;
      createdAt: number;
    }>;
    unreadCount: number;
  }
}
```

#### 13.2 Mark Notification as Read
```typescript
// Send
{
  type: 'notification.mark.read',
  payload: {
    userId: string;
    notificationId: string;
  }
}

// Receive
{
  type: 'notification.marked.read',
  payload: {
    notificationId: string;
  }
}
```

#### 13.3 Mark All as Read
```typescript
// Send
{
  type: 'notification.mark.all.read',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'notification.all.marked.read',
  payload: {
    count: number;
  }
}
```

#### 13.4 Delete Notification
```typescript
// Send
{
  type: 'notification.delete',
  payload: {
    userId: string;
    notificationId: string;
  }
}

// Receive
{
  type: 'notification.deleted',
  payload: {
    notificationId: string;
  }
}
```

---

### 14. **Achievements**

#### 14.1 Get Achievement List
```typescript
// Send
{
  type: 'achievement.list.sync',
  payload: {
    userId: string;
    unlockedOnly?: boolean;
  }
}

// Receive
{
  type: 'achievement.list.data',
  payload: {
    achievements: Array<{
      achievementId: string;
      name: string;
      description: string;
      icon?: string;
      rarity: 'common' | 'rare' | 'epic' | 'legendary';
      requirement: object;
      rewardPoints: number;
      rewardCoins: number;
      isUnlocked: boolean;
      unlockedAt?: number;
    }>;
    totalAchievements: number;
    unlockedCount: number;
  }
}
```

#### 14.2 Claim Achievement
```typescript
// Send
{
  type: 'achievement.claim',
  payload: {
    userId: string;
    achievementId: string;
  }
}

// Receive
{
  type: 'achievement.claim.success',
  payload: {
    achievementId: string;
    timestamp: string;
  }
}

// Auto-receive when unlocked
{
  type: 'achievement.unlocked',
  payload: {
    achievementId: string;
    name: string;
    description: string;
    rarity: 'common' | 'rare' | 'epic' | 'legendary';
    rewardPoints: number;
    rewardCoins: number;
    timestamp: number;
  }
}
```

---

### 15. **Daily Missions**

#### 15.1 Get Daily Mission List
```typescript
// Send
{
  type: 'daily.mission.list.sync',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'daily.mission.list.data',
  payload: {
    missions: Array<{
      missionId: string;
      name: string;
      description: string;
      type: 'play_games' | 'win_games' | 'answer_questions' | 'login_streak';
      requirement: {
        target: number;
        category?: string;
      };
      reward: {
        coins: number;
        experience: number;
      };
      progress: number;
      isCompleted: boolean;
      isClaimed: boolean;
      expiresAt: number;
    }>;
    completedToday: number;
    totalMissions: number;
  }
}
```

#### 15.2 Claim Daily Mission
```typescript
// Send
{
  type: 'daily.mission.claim',
  payload: {
    userId: string;
    missionId: string;
  }
}

// Receive
{
  type: 'daily.mission.claimed',
  payload: {
    missionId: string;
    rewards: {
      coins: number;
      experience: number;
    };
    newStats: {
      coins: number;
      experience: number;
      level: number;
    };
  }
}
```

---

### 16. **Ranked System**

#### 16.1 Get Ranked Stats
```typescript
// Send
{
  type: 'ranked.stats.sync',
  payload: {
    userId: string;
  }
}

// Receive
{
  type: 'ranked.stats.data',
  payload: {
    userId: string;
    tier: 'bronze' | 'silver' | 'gold' | 'platinum' | 'diamond' | 'master' | 'grandmaster';
    division: 1 | 2 | 3 | 4;
    mmr: number;
    rankedPoints: number;
    wins: number;
    losses: number;
    winRate: number;
    rank: number;
    topPercentage: number;
  }
}
```

#### 16.2 Get Ranked Leaderboard
```typescript
// Send
{
  type: 'ranked.leaderboard.sync',
  payload: {
    userId: string;
    tier?: string;
    limit?: number;
    offset?: number;
  }
}

// Receive
{
  type: 'ranked.leaderboard.data',
  payload: {
    leaderboard: Array<{
      rank: number;
      userId: string;
      username: string;
      tier: string;
      division: number;
      mmr: number;
      rankedPoints: number;
      wins: number;
      losses: number;
      winRate: number;
    }>;
    userRank: number;
    totalPlayers: number;
  }
}
```

---

## 📊 WebSocket Stats Endpoint

### Get WebSocket Statistics
```http
GET /api/quiz/stats
```

**Response Success (200):**
```typescript
{
  success: boolean;
  stats: {
    activeConnections: number;
    activeMatches: number;
    activeLobbies: number;
    matchmakingQueue: number;
  };
  timestamp: number;
}
```

**Example:**
```json
{
  "success": true,
  "stats": {
    "activeConnections": 150,
    "activeMatches": 42,
    "activeLobbies": 15,
    "matchmakingQueue": 8
  },
  "timestamp": 1705320000000
}
```

---

## 🔒 Error Responses

### Common Error Codes

#### 400 - Bad Request
```json
{
  "success": false,
  "error": "Invalid request data"
}
```

#### 401 - Unauthorized
```json
{
  "success": false,
  "error": "Unauthorized"
}
```

#### 403 - Forbidden
```json
{
  "success": false,
  "error": "Not authorized to perform this action"
}
```

#### 404 - Not Found
```json
{
  "success": false,
  "error": "Resource not found"
}
```

#### 500 - Internal Server Error
```json
{
  "success": false,
  "error": "Internal server error"
}
```

---

## 📝 Type Definitions

### Common Types

```typescript
interface UserInfo {
  id: string;
  name: string | null;
  email: string | null;
  image: string | null;
}

interface GameSettings {
  difficulty: 'easy' | 'medium' | 'hard';
  category: string;
  totalQuestions: number;
  timePerQuestion: number;
}

interface LobbyPlayer {
  userId: string;
  username: string;
  level: number;
  avatar?: string;
  isReady: boolean;
  isHost: boolean;
}

interface LeaderboardEntry {
  rank: number;
  userId: string;
  username: string;
  level: number;
  points: number;
  wins: number;
  losses: number;
  winRate: number;
  avatar?: string;
}

interface ChatMessage {
  messageId: string;
  userId: string;
  username: string;
  message: string;
  timestamp: number;
}

interface Notification {
  notificationId: string;
  type: 'friend_request' | 'achievement' | 'challenge' | 'system';
  title: string;
  message: string;
  data: object;
  priority: 'low' | 'medium' | 'high';
  isRead: boolean;
  createdAt: number;
}
```

---

## 🚀 Getting Started

### 1. Install Dependencies
```bash
bun install
```

### 2. Start Development Server
```bash
bun run dev
```

### 3. Connect to WebSocket
```javascript
const ws = new WebSocket('wss://elysia.asepharyana.tech/api/quiz/battle');

ws.onopen = () => {
  // Send authentication
  ws.send(JSON.stringify({
    type: 'auth:connect',
    payload: {
      userId: 'user_123',
      token: 'your_jwt_token'
    }
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

### 4. Make REST API Call
```javascript
// Get all posts
const response = await fetch('https://elysia.asepharyana.tech/api/sosmed/posts', {
  headers: {
    'Authorization': 'Bearer your_jwt_token'
  }
});

const data = await response.json();
console.log(data);
```

---

## 📌 Notes

- **Authentication**: Semua endpoint (kecuali `/api/users`) memerlukan JWT token
- **WebSocket**: Real-time communication untuk Quiz Battle features
- **Rate Limiting**: Akan diimplementasikan untuk production
- **CORS**: Sudah dikonfigurasi untuk development

---

## 🔧 Environment Variables

```env
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
JWT_SECRET=your_jwt_secret_key
REDIS_URL=redis://localhost:6379
PORT=3000
```

---

**Last Updated**: November 13, 2025
**Version**: 1.0.0
**API Base**: `https://elysia.asepharyana.tech`
