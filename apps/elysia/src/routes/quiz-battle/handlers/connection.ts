// Connection & Authentication Handlers

import type { ServerWebSocket } from 'bun';
import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  AuthConnectPayload,
  AuthConnectedPayload,
  AuthErrorPayload,
  UserStatusUpdatePayload,
  UserStatusChangedPayload,
  ConnectionPingPayload,
  ConnectionPongPayload,
  WSConnection,
  WSData,
} from '../types';
import {
  getDb,
  users,
  quizUserStats,
  quizFriendships,
  eq,
  and,
} from '../../../services';
import { wsLogger } from '../../../utils/logger';

// Reconnect payload type
interface ReconnectPayload {
  sessionId: string;
  userId: string;
  lastEventId?: string;
}

// Verify JWT token (simple version, you should use proper JWT verification)
async function verifyToken(
  token: string,
): Promise<{ userId: string; username: string } | null> {
  try {
    // TODO: Implement proper JWT verification
    // For now, we'll do a basic check
    // In production, use jwt.verify() with your secret

    // Decode token (basic example)
    const parts = token.split('.');
    if (parts.length !== 3) return null;

    const payload = JSON.parse(atob(parts[1]));
    return {
      userId: payload.userId || payload.sub,
      username: payload.username || payload.name,
    };
  } catch (error) {
    console.error('[Auth] Token verification failed:', error);
    return null;
  }
}

// Generate unique session ID
function generateSessionId(): string {
  return `session_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
}

export async function handleAuthConnect(
  ws: ServerWebSocket<WSData>,
  payload: AuthConnectPayload,
): Promise<void> {
  try {
    // Verify token
    const tokenData = await verifyToken(payload.token);

    if (!tokenData) {
      const errorMsg: WSMessage<AuthErrorPayload> = {
        type: 'auth.error',
        payload: {
          code: 'INVALID_TOKEN',
          message: 'Token tidak valid atau expired',
        },
      };
      ws.send(JSON.stringify(errorMsg));
      ws.close();
      return;
    }

    // Check if user exists in database
    const db = getDb();
    const [user] = await db
      .select()
      .from(users)
      .where(eq(users.id, payload.userId))
      .limit(1);

    if (!user) {
      const errorMsg: WSMessage<AuthErrorPayload> = {
        type: 'auth.error',
        payload: {
          code: 'USER_NOT_FOUND',
          message: 'User tidak ditemukan',
        },
      };
      ws.send(JSON.stringify(errorMsg));
      ws.close();
      return;
    }

    // Initialize user stats if not exists
    const [existingStats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, payload.userId))
      .limit(1);

    if (!existingStats) {
      await db.insert(quizUserStats).values({
        id: `stats_${payload.userId}`,
        userId: payload.userId,
        points: 0,
        wins: 0,
        losses: 0,
        draws: 0,
        totalGames: 0,
        currentStreak: 0,
        bestStreak: 0,
        totalCorrectAnswers: 0,
        totalQuestions: 0,
        level: 1,
        experience: 0,
        coins: 0,
      });
    }

    // Check for duplicate session
    const existingConnection = wsManager.getConnectionByUserId(payload.userId);
    if (existingConnection) {
      // Disconnect old session
      const disconnectMsg: WSMessage = {
        type: 'connection.disconnect',
        payload: {
          reason: 'duplicate_session',
          message: 'Anda login dari device lain',
          canReconnect: false,
        },
      };
      wsManager.sendToUser(payload.userId, disconnectMsg);
      wsManager.removeConnection(existingConnection.sessionId);
    }

    // Create new session
    const sessionId = generateSessionId();
    const connection: WSConnection = {
      userId: payload.userId,
      username: payload.username,
      sessionId,
      ws,
      status: 'online',
      lastPing: Date.now(),
    };

    wsManager.addConnection(sessionId, connection);

    // Send success response
    const successMsg: WSMessage<AuthConnectedPayload> = {
      type: 'auth.connected',
      payload: {
        userId: payload.userId,
        sessionId,
        serverTime: Date.now(),
        status: 'online',
      },
    };
    ws.send(JSON.stringify(successMsg));

    wsLogger.authenticated(sessionId, payload.userId, payload.username);
  } catch (error) {
    console.error('[Auth] Error during authentication:', error);
    const errorMsg: WSMessage<AuthErrorPayload> = {
      type: 'auth.error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Terjadi kesalahan server',
      },
    };
    ws.send(JSON.stringify(errorMsg));
  }
}

export async function handleUserStatusUpdate(
  sessionId: string,
  payload: UserStatusUpdatePayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    // Update status
    wsManager.updateUserStatus(payload.userId, payload.status);

    // Get user's friends to broadcast status change
    const db = getDb();

    const friendships = await db
      .select()
      .from(quizFriendships)
      .where(
        and(
          eq(quizFriendships.userId, payload.userId),
          eq(quizFriendships.status, 'accepted'),
        ),
      );

    const friendIds = friendships.map((f: { friendId: string }) => f.friendId);

    // Broadcast status change to online friends
    const statusMsg: WSMessage<UserStatusChangedPayload> = {
      type: 'user.status.changed',
      payload: {
        userId: payload.userId,
        username: connection.username,
        status: payload.status,
        timestamp: Date.now(),
      },
    };

    wsManager.broadcastToFriends(payload.userId, statusMsg, friendIds);

    console.log(
      `[Status] User ${connection.username} status changed to ${payload.status}`,
    );
  } catch (error) {
    console.error('[Status] Error updating user status:', error);
  }
}

export function handleConnectionPing(
  sessionId: string,
  payload: ConnectionPingPayload,
): void {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    // Update last ping time
    connection.lastPing = Date.now();

    // Send pong response
    const pongMsg: WSMessage<ConnectionPongPayload> = {
      type: 'connection.pong',
      payload: {
        timestamp: Date.now(),
        latency: Date.now() - payload.timestamp,
      },
    };

    wsManager.sendToSession(sessionId, pongMsg);
  } catch (error) {
    console.error('[Ping] Error handling ping:', error);
  }
}

export async function handleDisconnect(sessionId: string): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    wsLogger.disconnected(sessionId, connection.userId, 'user initiated');

    // Update status to offline
    await handleUserStatusUpdate(sessionId, {
      userId: connection.userId,
      status: 'offline',
    });

    // Check if user is in a match
    if (connection.currentMatchId) {
      const match = wsManager.getMatch(connection.currentMatchId);
      if (match && match.status !== 'finished') {
        // Import and call endGameByForfeit
        const { endGameByForfeit } = await import('./game');

        // Notify opponent first
        const opponentId =
          match.player1Id === connection.userId
            ? match.player2Id
            : match.player1Id;
        const opponent = wsManager.getConnectionByUserId(opponentId);

        if (opponent) {
          const disconnectMsg: WSMessage = {
            type: 'game.player.disconnected',
            payload: {
              matchId: connection.currentMatchId,
              disconnectedPlayer: {
                userId: connection.userId,
                username: connection.username,
              },
              waitTime: 0, // No wait time, game ends immediately
              autoWin: true,
            },
          };
          wsManager.sendToUser(opponentId, disconnectMsg);
        }

        // End the game with forfeit - disconnected player loses
        await endGameByForfeit(connection.currentMatchId, connection.userId);
        console.log(
          `[Disconnect] Player ${connection.username} forfeited match ${connection.currentMatchId}`,
        );
      }
    }

    // Check if user is in a lobby
    if (connection.currentLobbyId) {
      const lobby = wsManager.getLobby(connection.currentLobbyId);
      if (lobby) {
        wsManager.removeLobbyMember(
          connection.currentLobbyId,
          connection.userId,
        );

        // Notify other lobby members
        const leaveMsg: WSMessage = {
          type: 'lobby.player_left',
          payload: {
            lobbyId: connection.currentLobbyId,
            userId: connection.userId,
            username: connection.username,
          },
        };
        wsManager.broadcastToLobby(connection.currentLobbyId, leaveMsg);

        // If host left, close lobby
        if (lobby.hostId === connection.userId) {
          wsManager.removeLobby(connection.currentLobbyId);
        }
      }
    }

    // Remove from connection manager
    wsManager.removeConnection(sessionId);
  } catch (error) {
    console.error('[Disconnect] Error handling disconnect:', error);
  }
}

export async function handleReconnect(
  ws: ServerWebSocket<WSData>,
  payload: ReconnectPayload,
): Promise<void> {
  try {
    // Verify session
    const oldConnection = wsManager.getConnection(payload.sessionId);

    if (!oldConnection) {
      const errorMsg: WSMessage<AuthErrorPayload> = {
        type: 'auth.error',
        payload: {
          code: 'SESSION_NOT_FOUND',
          message: 'Session tidak ditemukan, silakan login ulang',
        },
      };
      ws.send(JSON.stringify(errorMsg));
      return;
    }

    // Update WebSocket instance
    oldConnection.ws = ws;
    oldConnection.lastPing = Date.now();
    oldConnection.status = 'online';

    // Send reconnected response with missed events
    const reconnectMsg: WSMessage = {
      type: 'connection.reconnected',
      payload: {
        sessionId: payload.sessionId,
        missedEvents: [], // TODO: Implement missed events queue
        serverTime: Date.now(),
      },
    };
    ws.send(JSON.stringify(reconnectMsg));

    // If user was in a match, notify opponent
    if (oldConnection.currentMatchId) {
      const match = wsManager.getMatch(oldConnection.currentMatchId);
      if (match) {
        const opponentId =
          match.player1Id === oldConnection.userId
            ? match.player2Id
            : match.player1Id;

        const reconnectedMsg: WSMessage = {
          type: 'game.player.reconnected',
          payload: {
            matchId: oldConnection.currentMatchId,
            reconnectedPlayer: {
              userId: oldConnection.userId,
              username: oldConnection.username,
            },
            gameState: match.gameState,
            resumeIn: 3,
          },
        };
        wsManager.sendToUser(opponentId, reconnectedMsg);
      }
    }

    console.log(
      `[Reconnect] User ${oldConnection.username} reconnected successfully`,
    );
  } catch (error) {
    console.error('[Reconnect] Error handling reconnect:', error);
  }
}
