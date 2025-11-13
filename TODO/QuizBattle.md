# 🌐 QuizBattle - WebSocket API List

Daftar lengkap API WebSocket yang dibutuhkan untuk membuat QuizBattle menjadi aplikasi online multiplayer realtime.

---

## 📋 Daftar Isi

1. [Authentication & User Management](#1-authentication--user-management)
2. [Matchmaking & Lobby](#2-matchmaking--lobby)
3. [Real-time Battle](#3-real-time-battle)
4. [Friend System](#4-friend-system)
5. [Leaderboard & Statistics](#5-leaderboard--statistics)
6. [Chat & Social](#6-chat--social)
7. [Notifications](#7-notifications)
8. [Connection Management](#8-connection-management)

---

## 1. Authentication & User Management

### 1.1 User Authentication

#### `ws://server/auth/connect`
**Description**: Koneksi awal dan autentikasi user

**Client → Server**
```json
{
  "type": "auth.connect",
  "payload": {
    "token": "jwt_token_here",
    "userId": "user_123",
    "username": "player_name",
    "deviceId": "device_unique_id"
  }
}
```

**Server → Client (Success)**
```json
{
  "type": "auth.connected",
  "payload": {
    "userId": "user_123",
    "sessionId": "session_abc123",
    "serverTime": 1699890000000,
    "status": "online"
  }
}
```

**Server → Client (Error)**
```json
{
  "type": "auth.error",
  "payload": {
    "code": "INVALID_TOKEN",
    "message": "Token tidak valid atau expired"
  }
}
```

---

### 1.2 User Status

#### `user.status.update`
**Description**: Update status online/offline user

**Client → Server**
```json
{
  "type": "user.status.update",
  "payload": {
    "userId": "user_123",
    "status": "online" // online, offline, in_game, away
  }
}
```

**Server → Client (Broadcast ke friends)**
```json
{
  "type": "user.status.changed",
  "payload": {
    "userId": "user_123",
    "username": "player_name",
    "status": "online",
    "timestamp": 1699890000000
  }
}
```

---

### 1.3 User Profile Update

#### `user.profile.sync`
**Description**: Sinkronisasi data profile user

**Client → Server**
```json
{
  "type": "user.profile.sync",
  "payload": {
    "userId": "user_123"
  }
}
```

**Server → Client**
```json
{
  "type": "user.profile.data",
  "payload": {
    "userId": "user_123",
    "username": "player_name",
    "email": "player@example.com",
    "points": 1500,
    "wins": 45,
    "losses": 20,
    "totalGames": 65,
    "rank": 15,
    "avatarUrl": "https://...",
    "level": 12,
    "experience": 3400
  }
}
```

---

## 2. Matchmaking & Lobby

### 2.1 Find Match

#### `matchmaking.find`
**Description**: Mencari lawan untuk battle

**Client → Server**
```json
{
  "type": "matchmaking.find",
  "payload": {
    "userId": "user_123",
    "gameMode": "ranked", // ranked, casual, friend
    "difficulty": "medium", // easy, medium, hard
    "category": "all" // all, science, history, technology, etc.
  }
}
```

**Server → Client (Searching)**
```json
{
  "type": "matchmaking.searching",
  "payload": {
    "estimatedWaitTime": 30, // seconds
    "playersInQueue": 5
  }
}
```

**Server → Client (Match Found)**
```json
{
  "type": "matchmaking.found",
  "payload": {
    "matchId": "match_xyz789",
    "opponent": {
      "userId": "user_456",
      "username": "opponent_name",
      "points": 1450,
      "wins": 40,
      "losses": 25,
      "avatarUrl": "https://..."
    },
    "gameSettings": {
      "totalQuestions": 5,
      "timePerQuestion": 10,
      "difficulty": "medium",
      "category": "all"
    },
    "startIn": 5 // countdown seconds
  }
}
```

---

### 2.2 Cancel Matchmaking

#### `matchmaking.cancel`
**Description**: Batalkan pencarian match

**Client → Server**
```json
{
  "type": "matchmaking.cancel",
  "payload": {
    "userId": "user_123"
  }
}
```

**Server → Client**
```json
{
  "type": "matchmaking.cancelled",
  "payload": {
    "reason": "user_cancelled"
  }
}
```

---

### 2.3 Private Lobby

#### `lobby.create`
**Description**: Buat lobby private untuk challenge friend

**Client → Server**
```json
{
  "type": "lobby.create",
  "payload": {
    "hostId": "user_123",
    "hostUsername": "player_name",
    "isPrivate": true,
    "maxPlayers": 2,
    "gameSettings": {
      "totalQuestions": 5,
      "timePerQuestion": 10,
      "difficulty": "medium",
      "category": "science"
    }
  }
}
```

**Server → Client**
```json
{
  "type": "lobby.created",
  "payload": {
    "lobbyId": "lobby_abc123",
    "lobbyCode": "QUIZ4567", // 8 character code
    "hostId": "user_123",
    "createdAt": 1699890000000
  }
}
```

---

#### `lobby.join`
**Description**: Join lobby dengan kode

**Client → Server**
```json
{
  "type": "lobby.join",
  "payload": {
    "userId": "user_456",
    "username": "opponent_name",
    "lobbyCode": "QUIZ4567"
  }
}
```

**Server → Client (Broadcast to lobby)**
```json
{
  "type": "lobby.player_joined",
  "payload": {
    "lobbyId": "lobby_abc123",
    "player": {
      "userId": "user_456",
      "username": "opponent_name",
      "points": 1300,
      "avatarUrl": "https://..."
    },
    "players": [
      {
        "userId": "user_123",
        "username": "player_name",
        "isHost": true,
        "isReady": false
      },
      {
        "userId": "user_456",
        "username": "opponent_name",
        "isHost": false,
        "isReady": false
      }
    ]
  }
}
```

---

#### `lobby.ready`
**Description**: Player ready untuk start game

**Client → Server**
```json
{
  "type": "lobby.ready",
  "payload": {
    "lobbyId": "lobby_abc123",
    "userId": "user_456",
    "isReady": true
  }
}
```

**Server → Client (Broadcast)**
```json
{
  "type": "lobby.player_ready",
  "payload": {
    "userId": "user_456",
    "isReady": true,
    "allReady": true // semua player ready
  }
}
```

---

#### `lobby.start`
**Description**: Host start game (semua player ready)

**Client → Server**
```json
{
  "type": "lobby.start",
  "payload": {
    "lobbyId": "lobby_abc123",
    "hostId": "user_123"
  }
}
```

**Server → Client (Broadcast)**
```json
{
  "type": "lobby.game_starting",
  "payload": {
    "matchId": "match_xyz789",
    "countdown": 3 // seconds
  }
}
```

---

## 3. Real-time Battle

### 3.1 Game Start

#### `game.start`
**Description**: Game dimulai, kirim data awal

**Server → Client (Broadcast to both players)**
```json
{
  "type": "game.started",
  "payload": {
    "matchId": "match_xyz789",
    "gameState": {
      "totalQuestions": 5,
      "currentQuestionIndex": 0,
      "timePerQuestion": 10,
      "playerHealth": 100,
      "opponentHealth": 100
    },
    "players": [
      {
        "userId": "user_123",
        "username": "player_name",
        "position": "left" // left or right
      },
      {
        "userId": "user_456",
        "username": "opponent_name",
        "position": "right"
      }
    ],
    "serverTime": 1699890000000
  }
}
```

---

### 3.2 Question Sync

#### `game.question.sync`
**Description**: Sinkronisasi pertanyaan untuk kedua player

**Server → Client (Broadcast)**
```json
{
  "type": "game.question.new",
  "payload": {
    "matchId": "match_xyz789",
    "questionIndex": 0,
    "question": {
      "id": "q_12345",
      "text": "Apa ibu kota Indonesia?",
      "answers": [
        "Jakarta",
        "Bandung",
        "Surabaya",
        "Medan"
      ],
      // Correct answer TIDAK dikirim ke client untuk anti-cheat
      "category": "Geography",
      "difficulty": "Easy"
    },
    "timeLimit": 10,
    "startTime": 1699890000000
  }
}
```

---

### 3.3 Answer Submission

#### `game.answer.submit`
**Description**: Player submit jawaban

**Client → Server**
```json
{
  "type": "game.answer.submit",
  "payload": {
    "matchId": "match_xyz789",
    "userId": "user_123",
    "questionId": "q_12345",
    "questionIndex": 0,
    "answerIndex": 0, // index 0-3
    "answerTime": 6.5, // berapa detik untuk jawab
    "timestamp": 1699890006500
  }
}
```

**Server → Client (To submitter)**
```json
{
  "type": "game.answer.received",
  "payload": {
    "questionIndex": 0,
    "isCorrect": true,
    "correctAnswerIndex": 0,
    "points": 20, // points earned
    "answerTime": 6.5
  }
}
```

---

### 3.4 Battle Update (Real-time)

#### `game.battle.update`
**Description**: Update status battle real-time

**Server → Client (Broadcast)**
```json
{
  "type": "game.battle.update",
  "payload": {
    "matchId": "match_xyz789",
    "questionIndex": 0,
    "event": "player_answered", // player_answered, player_attacked, player_hurt, timeout
    "actor": {
      "userId": "user_123",
      "action": "attack" // attack, hurt, miss
    },
    "gameState": {
      "playerHealth": 100,
      "opponentHealth": 80, // berkurang 20
      "playerScore": 20,
      "opponentScore": 0
    },
    "animation": {
      "type": "attack", // attack, hurt, defend, miss
      "target": "opponent",
      "damage": 20
    }
  }
}
```

---

### 3.5 Opponent Answer Notification

#### `game.opponent.answered`
**Description**: Notifikasi ketika opponent menjawab

**Server → Client**
```json
{
  "type": "game.opponent.answered",
  "payload": {
    "opponentId": "user_456",
    "questionIndex": 0,
    "answerTime": 5.2,
    "isCorrect": false, // bisa diketahui dari health opponent
    "animation": "hurt" // opponent salah, player menyerang
  }
}
```

---

### 3.6 Question Timeout

#### `game.question.timeout`
**Description**: Waktu habis untuk pertanyaan

**Server → Client (Broadcast)**
```json
{
  "type": "game.question.timeout",
  "payload": {
    "matchId": "match_xyz789",
    "questionIndex": 0,
    "correctAnswerIndex": 0,
    "players": [
      {
        "userId": "user_123",
        "answered": true,
        "isCorrect": true
      },
      {
        "userId": "user_456",
        "answered": false,
        "tookDamage": 20
      }
    ]
  }
}
```

---

### 3.7 Next Question

#### `game.question.next`
**Description**: Lanjut ke pertanyaan berikutnya

**Server → Client (Broadcast)**
```json
{
  "type": "game.question.next",
  "payload": {
    "matchId": "match_xyz789",
    "questionIndex": 1,
    "delay": 2 // delay 2 detik sebelum pertanyaan muncul
  }
}
```

---

### 3.8 Game Over

#### `game.over`
**Description**: Game selesai, kirim hasil

**Server → Client (Broadcast)**
```json
{
  "type": "game.over",
  "payload": {
    "matchId": "match_xyz789",
    "reason": "health_depleted", // health_depleted, all_questions_answered
    "winner": {
      "userId": "user_123",
      "username": "player_name",
      "finalHealth": 60,
      "finalScore": 100,
      "correctAnswers": 5,
      "totalAnswers": 5,
      "averageTime": 7.3
    },
    "loser": {
      "userId": "user_456",
      "username": "opponent_name",
      "finalHealth": 0,
      "finalScore": 60,
      "correctAnswers": 3,
      "totalAnswers": 5,
      "averageTime": 8.1
    },
    "rewards": {
      "winner": {
        "points": 100,
        "experience": 150,
        "coins": 50
      },
      "loser": {
        "points": 30,
        "experience": 50,
        "coins": 10
      }
    },
    "gameHistory": {
      "historyId": "history_abc123",
      "playedAt": 1699890000000,
      "duration": 65 // seconds
    }
  }
}
```

---

### 3.9 Player Disconnected

#### `game.player.disconnected`
**Description**: Player disconnect di tengah game

**Server → Client (To remaining player)**
```json
{
  "type": "game.player.disconnected",
  "payload": {
    "matchId": "match_xyz789",
    "disconnectedPlayer": {
      "userId": "user_456",
      "username": "opponent_name"
    },
    "waitTime": 30, // tunggu 30 detik untuk reconnect
    "autoWin": true // jika tidak reconnect, auto win
  }
}
```

---

### 3.10 Player Reconnected

#### `game.player.reconnected`
**Description**: Player berhasil reconnect

**Server → Client (Broadcast)**
```json
{
  "type": "game.player.reconnected",
  "payload": {
    "matchId": "match_xyz789",
    "reconnectedPlayer": {
      "userId": "user_456",
      "username": "opponent_name"
    },
    "gameState": {
      "currentQuestionIndex": 2,
      "playerHealth": 60,
      "opponentHealth": 40,
      "timeRemaining": 7
    },
    "resumeIn": 3 // countdown untuk resume
  }
}
```

---

## 4. Friend System

### 4.1 Friend Request

#### `friend.request.send`
**Description**: Kirim friend request

**Client → Server**
```json
{
  "type": "friend.request.send",
  "payload": {
    "senderId": "user_123",
    "senderUsername": "player_name",
    "targetUsername": "friend_name", // or targetUserId
    "message": "Ayo main bareng!"
  }
}
```

**Server → Client (To target user)**
```json
{
  "type": "friend.request.received",
  "payload": {
    "requestId": "req_abc123",
    "sender": {
      "userId": "user_123",
      "username": "player_name",
      "points": 1500,
      "wins": 45,
      "avatarUrl": "https://..."
    },
    "message": "Ayo main bareng!",
    "timestamp": 1699890000000
  }
}
```

---

#### `friend.request.accept`
**Description**: Accept friend request

**Client → Server**
```json
{
  "type": "friend.request.accept",
  "payload": {
    "requestId": "req_abc123",
    "userId": "user_456"
  }
}
```

**Server → Client (To both users)**
```json
{
  "type": "friend.request.accepted",
  "payload": {
    "friendship": {
      "friendshipId": "friendship_xyz789",
      "user1": {
        "userId": "user_123",
        "username": "player_name"
      },
      "user2": {
        "userId": "user_456",
        "username": "friend_name"
      },
      "createdAt": 1699890000000
    }
  }
}
```

---

#### `friend.request.reject`
**Description**: Reject friend request

**Client → Server**
```json
{
  "type": "friend.request.reject",
  "payload": {
    "requestId": "req_abc123",
    "userId": "user_456"
  }
}
```

**Server → Client (To sender)**
```json
{
  "type": "friend.request.rejected",
  "payload": {
    "requestId": "req_abc123",
    "rejectedBy": "user_456"
  }
}
```

---

### 4.2 Friend List

#### `friend.list.sync`
**Description**: Sinkronisasi daftar teman

**Client → Server**
```json
{
  "type": "friend.list.sync",
  "payload": {
    "userId": "user_123"
  }
}
```

**Server → Client**
```json
{
  "type": "friend.list.data",
  "payload": {
    "friends": [
      {
        "userId": "user_456",
        "username": "friend_1",
        "status": "online", // online, offline, in_game
        "points": 1300,
        "wins": 40,
        "lastSeen": 1699890000000,
        "avatarUrl": "https://..."
      },
      {
        "userId": "user_789",
        "username": "friend_2",
        "status": "in_game",
        "points": 1800,
        "wins": 60,
        "lastSeen": 1699890000000,
        "avatarUrl": "https://..."
      }
    ],
    "pendingRequests": 2,
    "totalFriends": 15
  }
}
```

---

### 4.3 Challenge Friend

#### `friend.challenge.send`
**Description**: Challenge friend untuk battle

**Client → Server**
```json
{
  "type": "friend.challenge.send",
  "payload": {
    "challengerId": "user_123",
    "challengerUsername": "player_name",
    "targetFriendId": "user_456",
    "gameSettings": {
      "difficulty": "hard",
      "category": "science",
      "totalQuestions": 10
    },
    "message": "Berani lawan gw?"
  }
}
```

**Server → Client (To target friend)**
```json
{
  "type": "friend.challenge.received",
  "payload": {
    "challengeId": "challenge_abc123",
    "challenger": {
      "userId": "user_123",
      "username": "player_name",
      "points": 1500,
      "wins": 45
    },
    "gameSettings": {
      "difficulty": "hard",
      "category": "science",
      "totalQuestions": 10
    },
    "message": "Berani lawan gw?",
    "expiresIn": 60 // detik
  }
}
```

---

#### `friend.challenge.accept`
**Description**: Accept challenge

**Client → Server**
```json
{
  "type": "friend.challenge.accept",
  "payload": {
    "challengeId": "challenge_abc123",
    "acceptorId": "user_456"
  }
}
```

**Server → Client (To both users)**
```json
{
  "type": "friend.challenge.accepted",
  "payload": {
    "challengeId": "challenge_abc123",
    "matchId": "match_xyz789",
    "lobbyId": "lobby_abc123",
    "startIn": 5
  }
}
```

---

#### `friend.challenge.reject`
**Description**: Reject challenge

**Client → Server**
```json
{
  "type": "friend.challenge.reject",
  "payload": {
    "challengeId": "challenge_abc123",
    "rejecterId": "user_456"
  }
}
```

**Server → Client (To challenger)**
```json
{
  "type": "friend.challenge.rejected",
  "payload": {
    "challengeId": "challenge_abc123",
    "rejectedBy": {
      "userId": "user_456",
      "username": "friend_name"
    }
  }
}
```

---

### 4.4 Remove Friend

#### `friend.remove`
**Description**: Hapus teman dari list

**Client → Server**
```json
{
  "type": "friend.remove",
  "payload": {
    "userId": "user_123",
    "friendId": "user_456"
  }
}
```

**Server → Client**
```json
{
  "type": "friend.removed",
  "payload": {
    "friendId": "user_456",
    "success": true
  }
}
```

---

## 5. Leaderboard & Statistics

### 5.1 Global Leaderboard

#### `leaderboard.global.sync`
**Description**: Get global leaderboard

**Client → Server**
```json
{
  "type": "leaderboard.global.sync",
  "payload": {
    "limit": 100, // top 100
    "offset": 0,
    "timeframe": "weekly" // all_time, weekly, monthly, daily
  }
}
```

**Server → Client**
```json
{
  "type": "leaderboard.global.data",
  "payload": {
    "leaderboard": [
      {
        "rank": 1,
        "userId": "user_001",
        "username": "pro_player",
        "points": 5000,
        "wins": 150,
        "losses": 30,
        "winRate": 0.833,
        "avatarUrl": "https://...",
        "level": 25
      },
      {
        "rank": 2,
        "userId": "user_002",
        "username": "quiz_master",
        "points": 4800,
        "wins": 140,
        "losses": 35,
        "winRate": 0.8,
        "avatarUrl": "https://...",
        "level": 24
      }
      // ... more entries
    ],
    "userRank": {
      "rank": 45,
      "points": 1500,
      "percentile": 0.95 // top 5%
    },
    "totalPlayers": 10000,
    "updatedAt": 1699890000000
  }
}
```

---

### 5.2 Friends Leaderboard

#### `leaderboard.friends.sync`
**Description**: Leaderboard hanya teman

**Client → Server**
```json
{
  "type": "leaderboard.friends.sync",
  "payload": {
    "userId": "user_123",
    "timeframe": "weekly"
  }
}
```

**Server → Client**
```json
{
  "type": "leaderboard.friends.data",
  "payload": {
    "leaderboard": [
      {
        "rank": 1,
        "userId": "user_456",
        "username": "friend_1",
        "points": 1800,
        "wins": 60,
        "status": "online"
      },
      {
        "rank": 2,
        "userId": "user_123",
        "username": "player_name",
        "points": 1500,
        "wins": 45,
        "status": "online"
      }
      // ... more friends
    ]
  }
}
```

---

### 5.3 Real-time Leaderboard Update

#### `leaderboard.update`
**Description**: Update leaderboard secara real-time

**Server → Client (Broadcast ke relevant users)**
```json
{
  "type": "leaderboard.updated",
  "payload": {
    "userId": "user_123",
    "oldRank": 46,
    "newRank": 45,
    "pointsGained": 100,
    "newPoints": 1500,
    "rankChange": 1 // naik 1 peringkat
  }
}
```

---

### 5.4 User Statistics

#### `stats.user.sync`
**Description**: Get detailed user statistics

**Client → Server**
```json
{
  "type": "stats.user.sync",
  "payload": {
    "userId": "user_123",
    "period": "all_time" // all_time, weekly, monthly
  }
}
```

**Server → Client**
```json
{
  "type": "stats.user.data",
  "payload": {
    "userId": "user_123",
    "statistics": {
      "totalGames": 65,
      "wins": 45,
      "losses": 20,
      "winRate": 0.692,
      "totalPoints": 1500,
      "averageScore": 80,
      "bestStreak": 8,
      "currentStreak": 3,
      "averageAnswerTime": 7.5,
      "totalCorrectAnswers": 280,
      "totalQuestions": 325,
      "accuracy": 0.862,
      "favoriteCategory": "Science",
      "categoryStats": [
        {
          "category": "Science",
          "gamesPlayed": 20,
          "accuracy": 0.90
        },
        {
          "category": "History",
          "gamesPlayed": 15,
          "accuracy": 0.85
        }
      ],
      "recentGames": [
        {
          "matchId": "match_001",
          "opponent": "bot_easy",
          "isVictory": true,
          "score": 100,
          "playedAt": 1699890000000
        }
      ]
    }
  }
}
```

---

## 6. Chat & Social

### 6.1 Global Chat

#### `chat.global.send`
**Description**: Kirim pesan ke global chat

**Client → Server**
```json
{
  "type": "chat.global.send",
  "payload": {
    "userId": "user_123",
    "username": "player_name",
    "message": "Halo semua!",
    "timestamp": 1699890000000
  }
}
```

**Server → Client (Broadcast)**
```json
{
  "type": "chat.global.message",
  "payload": {
    "messageId": "msg_abc123",
    "sender": {
      "userId": "user_123",
      "username": "player_name",
      "level": 12,
      "avatarUrl": "https://..."
    },
    "message": "Halo semua!",
    "timestamp": 1699890000000
  }
}
```

---

### 6.2 Private Chat (DM)

#### `chat.private.send`
**Description**: Kirim private message ke friend

**Client → Server**
```json
{
  "type": "chat.private.send",
  "payload": {
    "senderId": "user_123",
    "receiverId": "user_456",
    "message": "Main yuk!",
    "timestamp": 1699890000000
  }
}
```

**Server → Client (To receiver)**
```json
{
  "type": "chat.private.message",
  "payload": {
    "messageId": "msg_abc123",
    "conversationId": "conv_123_456",
    "sender": {
      "userId": "user_123",
      "username": "player_name",
      "status": "online"
    },
    "message": "Main yuk!",
    "timestamp": 1699890000000,
    "isRead": false
  }
}
```

---

### 6.3 In-Game Chat

#### `chat.game.send`
**Description**: Chat di dalam battle/match

**Client → Server**
```json
{
  "type": "chat.game.send",
  "payload": {
    "matchId": "match_xyz789",
    "senderId": "user_123",
    "message": "GG!", // atau quick chat emoji
    "messageType": "text" // text, emoji, quick_chat
  }
}
```

**Server → Client (To opponent)**
```json
{
  "type": "chat.game.message",
  "payload": {
    "matchId": "match_xyz789",
    "sender": {
      "userId": "user_123",
      "username": "player_name"
    },
    "message": "GG!",
    "messageType": "text",
    "timestamp": 1699890000000
  }
}
```

---

### 6.4 Chat History

#### `chat.history.sync`
**Description**: Get chat history dengan friend

**Client → Server**
```json
{
  "type": "chat.history.sync",
  "payload": {
    "userId": "user_123",
    "conversationId": "conv_123_456",
    "limit": 50,
    "offset": 0
  }
}
```

**Server → Client**
```json
{
  "type": "chat.history.data",
  "payload": {
    "conversationId": "conv_123_456",
    "messages": [
      {
        "messageId": "msg_001",
        "senderId": "user_123",
        "message": "Halo!",
        "timestamp": 1699880000000,
        "isRead": true
      },
      {
        "messageId": "msg_002",
        "senderId": "user_456",
        "message": "Hi! Main yuk",
        "timestamp": 1699880100000,
        "isRead": true
      }
    ],
    "hasMore": true
  }
}
```

---

## 7. Notifications

### 7.1 General Notification

#### `notification.send`
**Description**: Server mengirim notifikasi

**Server → Client**
```json
{
  "type": "notification.received",
  "payload": {
    "notificationId": "notif_abc123",
    "notificationType": "achievement", // achievement, friend_request, challenge, system
    "title": "Achievement Unlocked!",
    "message": "Kamu telah memenangkan 50 game!",
    "icon": "trophy",
    "data": {
      "achievementId": "ach_50_wins",
      "reward": {
        "points": 500,
        "coins": 100
      }
    },
    "priority": "high", // low, medium, high
    "timestamp": 1699890000000,
    "expiresAt": 1699893600000
  }
}
```

---

### 7.2 Achievement Notification

#### `notification.achievement`
**Description**: Notifikasi achievement unlock

**Server → Client**
```json
{
  "type": "notification.achievement",
  "payload": {
    "achievementId": "ach_first_win",
    "title": "First Blood",
    "description": "Menangkan pertama kalinya!",
    "icon": "🏆",
    "rarity": "common", // common, rare, epic, legendary
    "reward": {
      "points": 100,
      "coins": 50,
      "title": "Newbie Champion"
    },
    "unlockedAt": 1699890000000
  }
}
```

---

### 7.3 System Notification

#### `notification.system`
**Description**: System notification (maintenance, update, event)

**Server → Client (Broadcast)**
```json
{
  "type": "notification.system",
  "payload": {
    "notificationType": "maintenance", // maintenance, update, event, announcement
    "title": "Server Maintenance",
    "message": "Server akan maintenance pada 15 Nov 2024, 02:00 - 04:00 WIB",
    "priority": "high",
    "action": {
      "type": "open_url",
      "url": "https://quizbattle.com/maintenance"
    },
    "timestamp": 1699890000000
  }
}
```

---

### 7.4 Notification Read

#### `notification.read`
**Description**: Mark notification sebagai dibaca

**Client → Server**
```json
{
  "type": "notification.read",
  "payload": {
    "userId": "user_123",
    "notificationId": "notif_abc123"
  }
}
```

**Server → Client**
```json
{
  "type": "notification.read.confirmed",
  "payload": {
    "notificationId": "notif_abc123",
    "success": true
  }
}
```

---

## 8. Connection Management

### 8.1 Heartbeat / Ping

#### `connection.ping`
**Description**: Keep-alive ping untuk maintain connection

**Client → Server**
```json
{
  "type": "connection.ping",
  "payload": {
    "timestamp": 1699890000000
  }
}
```

**Server → Client**
```json
{
  "type": "connection.pong",
  "payload": {
    "timestamp": 1699890000100,
    "latency": 100 // ms
  }
}
```

---

### 8.2 Connection Error

#### `connection.error`
**Description**: Error pada koneksi

**Server → Client**
```json
{
  "type": "connection.error",
  "payload": {
    "code": "CONNECTION_UNSTABLE",
    "message": "Koneksi tidak stabil, mencoba reconnect...",
    "severity": "warning", // info, warning, critical
    "retryIn": 5 // seconds
  }
}
```

---

### 8.3 Force Disconnect

#### `connection.disconnect`
**Description**: Server force disconnect client

**Server → Client**
```json
{
  "type": "connection.disconnect",
  "payload": {
    "reason": "duplicate_session", // duplicate_session, banned, server_shutdown
    "message": "Anda login dari device lain",
    "canReconnect": false
  }
}
```

---

### 8.4 Reconnection

#### `connection.reconnect`
**Description**: Client reconnect setelah disconnect

**Client → Server**
```json
{
  "type": "connection.reconnect",
  "payload": {
    "userId": "user_123",
    "sessionId": "session_abc123",
    "lastEventId": "event_12345" // untuk sync missed events
  }
}
```

**Server → Client**
```json
{
  "type": "connection.reconnected",
  "payload": {
    "sessionId": "session_abc123",
    "missedEvents": [
      {
        "type": "chat.private.message",
        "payload": { /* ... */ },
        "timestamp": 1699890050000
      },
      {
        "type": "friend.request.received",
        "payload": { /* ... */ },
        "timestamp": 1699890100000
      }
    ],
    "serverTime": 1699890200000
  }
}
```

---

## 9. Additional Features (Optional)

### 9.1 Tournament System

#### `tournament.list`
**Description**: Get list tournament yang sedang berjalan

**Client → Server**
```json
{
  "type": "tournament.list",
  "payload": {
    "status": "open", // open, ongoing, finished
    "limit": 10
  }
}
```

**Server → Client**
```json
{
  "type": "tournament.list.data",
  "payload": {
    "tournaments": [
      {
        "tournamentId": "tour_001",
        "name": "Weekly Championship",
        "description": "Turnamen mingguan dengan hadiah besar!",
        "startTime": 1699900000000,
        "endTime": 1700000000000,
        "entryFee": 100, // coins
        "prizePool": 10000,
        "maxParticipants": 128,
        "currentParticipants": 87,
        "status": "open",
        "format": "single_elimination",
        "rules": {
          "questionsPerMatch": 10,
          "difficulty": "hard",
          "category": "all"
        }
      }
    ]
  }
}
```

---

#### `tournament.join`
**Description**: Join tournament

**Client → Server**
```json
{
  "type": "tournament.join",
  "payload": {
    "userId": "user_123",
    "tournamentId": "tour_001"
  }
}
```

**Server → Client**
```json
{
  "type": "tournament.joined",
  "payload": {
    "tournamentId": "tour_001",
    "bracketPosition": 87,
    "nextMatch": {
      "matchId": "tmatch_001",
      "opponent": "TBD",
      "scheduledTime": 1699910000000
    }
  }
}
```

---

### 9.2 Daily Missions

#### `missions.daily.sync`
**Description**: Get daily missions

**Client → Server**
```json
{
  "type": "missions.daily.sync",
  "payload": {
    "userId": "user_123"
  }
}
```

**Server → Client**
```json
{
  "type": "missions.daily.data",
  "payload": {
    "missions": [
      {
        "missionId": "mission_001",
        "title": "Win 3 Games",
        "description": "Menangkan 3 permainan hari ini",
        "type": "daily",
        "progress": {
          "current": 1,
          "target": 3
        },
        "reward": {
          "points": 150,
          "coins": 50
        },
        "expiresAt": 1699916400000, // end of day
        "isCompleted": false
      },
      {
        "missionId": "mission_002",
        "title": "Answer 20 Questions Correctly",
        "description": "Jawab 20 pertanyaan dengan benar",
        "type": "daily",
        "progress": {
          "current": 15,
          "target": 20
        },
        "reward": {
          "points": 100,
          "coins": 30
        },
        "expiresAt": 1699916400000,
        "isCompleted": false
      }
    ],
    "refreshIn": 28800 // seconds until reset (8 hours)
  }
}
```

---

#### `missions.progress.update`
**Description**: Update mission progress real-time

**Server → Client**
```json
{
  "type": "missions.progress.updated",
  "payload": {
    "missionId": "mission_001",
    "progress": {
      "current": 2,
      "target": 3
    },
    "isCompleted": false
  }
}
```

---

#### `missions.completed`
**Description**: Mission completed notification

**Server → Client**
```json
{
  "type": "missions.completed",
  "payload": {
    "missionId": "mission_001",
    "title": "Win 3 Games",
    "reward": {
      "points": 150,
      "coins": 50
    },
    "completedAt": 1699890000000
  }
}
```

---

### 9.3 Shop & Inventory

#### `shop.items.sync`
**Description**: Get shop items

**Client → Server**
```json
{
  "type": "shop.items.sync",
  "payload": {
    "category": "all" // all, avatars, emotes, titles, power_ups
  }
}
```

**Server → Client**
```json
{
  "type": "shop.items.data",
  "payload": {
    "items": [
      {
        "itemId": "item_avatar_001",
        "name": "Knight Avatar",
        "description": "Avatar kesatria legendaris",
        "category": "avatar",
        "price": {
          "coins": 500,
          "points": 0
        },
        "rarity": "epic",
        "imageUrl": "https://...",
        "isOwned": false,
        "isLimited": false
      }
    ],
    "featuredItems": ["item_avatar_001"],
    "refreshIn": 86400 // daily refresh
  }
}
```

---

#### `shop.purchase`
**Description**: Purchase item dari shop

**Client → Server**
```json
{
  "type": "shop.purchase",
  "payload": {
    "userId": "user_123",
    "itemId": "item_avatar_001",
    "quantity": 1
  }
}
```

**Server → Client**
```json
{
  "type": "shop.purchase.success",
  "payload": {
    "itemId": "item_avatar_001",
    "name": "Knight Avatar",
    "newBalance": {
      "coins": 500,
      "points": 1500
    },
    "addedToInventory": true
  }
}
```

---

## 📊 WebSocket Event Priority

### High Priority (Real-time critical)
- `game.*` - Semua event game
- `matchmaking.found`
- `connection.ping` / `connection.pong`
- `game.player.disconnected`

### Medium Priority
- `friend.*` - Friend system events
- `chat.private.*` - Private messages
- `notification.*` - Notifications
- `leaderboard.updated`

### Low Priority
- `chat.global.*` - Global chat
- `shop.*` - Shop events
- `missions.*` - Mission updates

---

## 🔒 Security Considerations

### Anti-Cheat Measures
1. **Server-side validation**: Semua jawaban divalidasi di server
2. **Timestamp verification**: Cek waktu jawaban tidak lebih cepat dari time limit
3. **Question randomization**: Pertanyaan diacak server-side
4. **Encrypted answers**: Jawaban benar tidak dikirim ke client sampai dijawab
5. **Rate limiting**: Limit jumlah request per detik

### Authentication
1. JWT Token untuk autentikasi
2. Session management
3. Device fingerprinting untuk detect multi-account
4. IP rate limiting

---

## 📈 Scaling Considerations

### Load Balancing
- Gunakan Redis untuk pub/sub antar server instances
- Sticky sessions untuk WebSocket connections
- Horizontal scaling dengan multiple server nodes

### Database
- Read replicas untuk leaderboard & statistics
- Caching dengan Redis untuk frequently accessed data
- Database sharding untuk user data

### Message Queue
- RabbitMQ/Redis untuk queue system
- Async processing untuk non-critical events
- Background jobs untuk statistics calculation

---

## 🧪 Testing WebSocket

### Example Test Cases

```javascript
// Connect to WebSocket
const ws = new WebSocket('ws://server/auth/connect');

// Authentication
ws.send(JSON.stringify({
  type: 'auth.connect',
  payload: {
    token: 'your_jwt_token',
    userId: 'user_123',
    username: 'test_player',
    deviceId: 'device_abc'
  }
}));

// Listen for messages
ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
  
  // Handle different message types
  switch(message.type) {
    case 'auth.connected':
      console.log('Connected successfully');
      break;
    case 'game.question.new':
      console.log('New question:', message.payload.question);
      break;
    // ... handle other events
  }
};

// Find match
ws.send(JSON.stringify({
  type: 'matchmaking.find',
  payload: {
    userId: 'user_123',
    gameMode: 'ranked',
    difficulty: 'medium',
    category: 'all'
  }
}));
```

---

## 📝 Implementation Checklist

### Phase 1: Core Features
- [ ] Authentication & Connection Management
- [ ] Matchmaking System
- [ ] Real-time Battle (Question Sync, Answer Submit, Battle Updates)
- [ ] Game Over & Results

### Phase 2: Social Features
- [ ] Friend System (Request, Accept, List)
- [ ] Private Lobby & Challenge
- [ ] Chat System (Global, Private, In-Game)

### Phase 3: Progression
- [ ] Leaderboard (Global, Friends)
- [ ] User Statistics
- [ ] Achievements
- [ ] Daily Missions

### Phase 4: Monetization (Optional)
- [ ] Shop System
- [ ] Inventory Management
- [ ] Tournament System

---

## 🔗 Client Libraries

### Android (Kotlin)
```kotlin
// OkHttp WebSocket atau Scarlet library
implementation("com.squareup.okhttp3:okhttp:4.12.0")
// atau
implementation("com.tinder.scarlet:scarlet:0.1.12")
```

### Backend Options
- **Node.js**: Socket.io, ws library
- **Python**: FastAPI with WebSockets, Socket.io
- **Java/Kotlin**: Spring Boot WebSocket, Ktor
- **Go**: Gorilla WebSocket

---

## 🎯 Kesimpulan

Dokumen ini mencakup semua API WebSocket yang dibutuhkan untuk:
- ✅ Multiplayer real-time gameplay
- ✅ Matchmaking & lobby system
- ✅ Friend & social features
- ✅ Leaderboard & statistics
- ✅ Chat system
- ✅ Notifications
- ✅ Connection management & anti-cheat

**Total Events**: ~80+ event types untuk sistem lengkap

Prioritaskan implementasi Phase 1 terlebih dahulu untuk MVP (Minimum Viable Product), kemudian lanjut ke phase selanjutnya sesuai kebutuhan.
