// Main WebSocket Route Handler for Quiz Battle
/* eslint-disable @typescript-eslint/no-explicit-any */

import { Elysia, t } from 'elysia';
import { wsManager } from './ws-manager';
import type { WSMessage, WSData } from './types';
import type { ServerWebSocket } from 'bun';

// Import handlers
import {
  handleAuthConnect,
  handleUserStatusUpdate,
  handleConnectionPing,
  handleDisconnect,
  handleReconnect,
} from './handlers/connection';

import {
  handleMatchmakingFind,
  handleMatchmakingCancel,
} from './handlers/matchmaking';

import { handleGameAnswerSubmit } from './handlers/game';

import {
  handleFriendRequestSend,
  handleFriendRequestAccept,
  handleFriendRequestReject,
  handleFriendRemove,
  handleFriendListRequest,
  handleFriendChallenge,
} from './handlers/friends';

import {
  handleLeaderboardGlobalSync,
  handleLeaderboardFriendsSync,
} from './handlers/leaderboard';

import {
  handleLobbyCreate,
  handleLobbyJoin,
  handleLobbyReady,
  handleLobbyStart,
  handleLobbyLeave,
  handleLobbyKick,
  handleLobbyListSync,
} from './handlers/lobby';

import {
  handleChatGlobalSend,
  handleChatPrivateSend,
  handleChatHistorySync,
  handleChatTyping,
  handleChatMarkRead,
} from './handlers/chat';

import {
  handleNotificationListSync,
  handleNotificationMarkRead,
  handleNotificationMarkAllRead,
  handleNotificationDelete,
} from './handlers/notifications';

import {
  handleAchievementListSync,
  handleAchievementClaim,
} from './handlers/achievements';

import {
  handleDailyMissionListSync,
  handleDailyMissionClaim,
} from './handlers/daily-missions';

import {
  handleRankedStatsSync,
  handleRankedLeaderboardSync,
} from './handlers/ranked';

// Store session IDs in a WeakMap to avoid type issues
const sessionIds = new WeakMap<ServerWebSocket<WSData>, string>();

export const quizBattleWS = new Elysia({ prefix: '/api/quiz' })
  .ws('/battle', {
    body: t.Any(),

    open(ws) {
      console.log('[WS] Client connected');
      // Store sessionId in WeakMap
      const sessionId = Math.random().toString(36).substring(2, 15);
      sessionIds.set(ws.raw as ServerWebSocket<WSData>, sessionId);
    },

    message(ws, rawMessage: unknown) {
      try {
        const message: WSMessage =
          typeof rawMessage === 'string'
            ? JSON.parse(rawMessage)
            : (rawMessage as WSMessage);

        console.log(`[WS] Received message type: ${message.type}`);

        // Get session ID from WeakMap
        const sessionId =
          sessionIds.get(ws.raw as ServerWebSocket<WSData>) ||
          Math.random().toString(36).substring(2, 15);

        // Route messages based on type
        switch (message.type) {
          // ===== AUTHENTICATION & CONNECTION =====
          case 'auth:connect':
            handleAuthConnect(ws.raw as any, message.payload as any);
            break;

          case 'user.status.update':
            if (sessionId) {
              handleUserStatusUpdate(sessionId, message.payload as any);
            }
            break;

          case 'connection.ping':
            if (sessionId) {
              handleConnectionPing(sessionId, message.payload as any);
            }
            break;

          case 'connection.reconnect':
            handleReconnect(ws.raw as any, message.payload as any);
            break;

          // ===== MATCHMAKING =====
          case 'matchmaking.find':
            if (sessionId) {
              handleMatchmakingFind(sessionId, message.payload as any);
            }
            break;

          case 'matchmaking.cancel':
            if (sessionId) {
              handleMatchmakingCancel(sessionId, message.payload as any);
            }
            break;

          // ===== GAME =====
          case 'game.connect':
            // Client acknowledges connection to match - game starts automatically via startGameMatch
            console.log(`[WS] Client connected to game match`);
            ws.send(
              JSON.stringify({
                type: 'game.connect.ack',
                payload: { status: 'connected', timestamp: Date.now() },
              }),
            );
            break;

          case 'game.answer.submit':
            if (sessionId) {
              handleGameAnswerSubmit(sessionId, message.payload as any);
            }
            break;

          // ===== FRIEND SYSTEM =====
          case 'friend.request.send':
            if (sessionId) {
              handleFriendRequestSend(sessionId, message.payload as any);
            }
            break;

          case 'friend.request.accept':
            if (sessionId) {
              handleFriendRequestAccept(sessionId, message.payload as any);
            }
            break;

          case 'friend.request.reject':
            if (sessionId) {
              handleFriendRequestReject(sessionId, message.payload as any);
            }
            break;

          case 'friend.remove':
            if (sessionId) {
              handleFriendRemove(sessionId, message.payload as any);
            }
            break;

          case 'friend.list.request':
            if (sessionId) {
              handleFriendListRequest(sessionId, message.payload as any);
            }
            break;

          case 'friend.challenge':
            if (sessionId) {
              handleFriendChallenge(sessionId, message.payload as any);
            }
            break;

          // ===== LEADERBOARD =====
          case 'leaderboard.global.sync':
            if (sessionId) {
              handleLeaderboardGlobalSync(sessionId, message.payload as any);
            }
            break;

          case 'leaderboard.friends.sync':
            if (sessionId) {
              handleLeaderboardFriendsSync(sessionId, message.payload as any);
            }
            break;

          // ===== LOBBY SYSTEM =====
          case 'lobby.create':
            if (sessionId) {
              handleLobbyCreate(sessionId, message.payload as any);
            }
            break;

          case 'lobby.join':
            if (sessionId) {
              handleLobbyJoin(sessionId, message.payload as any);
            }
            break;

          case 'lobby.ready':
            if (sessionId) {
              handleLobbyReady(sessionId, message.payload as any);
            }
            break;

          case 'lobby.start':
            if (sessionId) {
              handleLobbyStart(sessionId, message.payload as any);
            }
            break;

          case 'lobby.leave':
            if (sessionId) {
              handleLobbyLeave(sessionId, message.payload as any);
            }
            break;

          case 'lobby.kick':
            if (sessionId) {
              handleLobbyKick(sessionId, message.payload as any);
            }
            break;

          case 'lobby.list.sync':
            if (sessionId) {
              handleLobbyListSync(sessionId);
            }
            break;

          // ===== CHAT SYSTEM =====
          case 'chat:global:send':
            if (sessionId) {
              handleChatGlobalSend(sessionId, message as any);
            }
            break;

          case 'chat:private:send':
            if (sessionId) {
              handleChatPrivateSend(sessionId, message as any);
            }
            break;

          case 'chat:history:sync':
            if (sessionId) {
              handleChatHistorySync(sessionId, message as any);
            }
            break;

          case 'chat:typing':
            if (sessionId) {
              handleChatTyping(sessionId, message as any);
            }
            break;

          case 'chat:mark:read':
            if (sessionId) {
              handleChatMarkRead(sessionId, message as any);
            }
            break;

          // ===== NOTIFICATIONS =====
          case 'notification.list.sync':
            if (sessionId) {
              handleNotificationListSync(sessionId, message as any);
            }
            break;

          case 'notification.mark.read':
            if (sessionId) {
              handleNotificationMarkRead(sessionId, message as any);
            }
            break;

          case 'notification.mark.all.read':
            if (sessionId) {
              handleNotificationMarkAllRead(sessionId, message as any);
            }
            break;

          case 'notification.delete':
            if (sessionId) {
              handleNotificationDelete(sessionId, message as any);
            }
            break;

          // ===== ACHIEVEMENTS =====
          case 'achievement.list.sync':
            if (sessionId) {
              handleAchievementListSync(sessionId, message as any);
            }
            break;

          case 'achievement.claim':
            if (sessionId) {
              handleAchievementClaim(sessionId, message as any);
            }
            break;

          // ===== DAILY MISSIONS =====
          case 'daily.mission.list.sync':
            if (sessionId) {
              handleDailyMissionListSync(sessionId, message as any);
            }
            break;

          case 'daily.mission.claim':
            if (sessionId) {
              handleDailyMissionClaim(sessionId, message as any);
            }
            break;

          // ===== RANKED SYSTEM =====
          case 'ranked.stats.sync':
            handleRankedStatsSync(ws.raw as any, message as any);
            break;

          case 'ranked.leaderboard.sync':
            handleRankedLeaderboardSync(ws.raw as any, message as any);
            break;

          // ===== FUTURE ENHANCEMENTS =====
          // - Shop & Cosmetics
          // - Tournament Brackets

          default:
            console.warn(`[WS] Unknown message type: ${message.type}`);
            ws.send(
              JSON.stringify({
                type: 'error',
                payload: {
                  code: 'UNKNOWN_MESSAGE_TYPE',
                  message: `Unknown message type: ${message.type}`,
                },
              }),
            );
        }
      } catch (error) {
        console.error('[WS] Error processing message:', error);
        ws.send(
          JSON.stringify({
            type: 'error',
            payload: {
              code: 'MESSAGE_PROCESSING_ERROR',
              message: 'Error processing message',
            },
          }),
        );
      }
    },

    close(ws) {
      console.log('[WS] Client disconnected');
      const sessionId = sessionIds.get(ws.raw as ServerWebSocket<WSData>);
      if (sessionId) {
        handleDisconnect(sessionId);
        sessionIds.delete(ws.raw as ServerWebSocket<WSData>);
      }
    },
  })
  .get(
    '/stats',
    () => {
      // Endpoint untuk monitoring stats
      return {
        success: true,
        stats: wsManager.getStats(),
        timestamp: Date.now(),
      };
    },
    {
      detail: {
        tags: ['Quiz Battle'],
        summary: 'Get Quiz Battle WebSocket statistics',
        description:
          'Returns current statistics about active connections, matches, lobbies, and queue',
      },
    },
  )
  .get(
    '/lobbies',
    () => {
      // Get list of all active lobbies
      const lobbies = Array.from(wsManager['activeLobbies'].values()).map(
        (lobby) => ({
          lobbyId: lobby.lobbyId,
          lobbyCode: lobby.lobbyCode,
          hostId: lobby.hostId,
          memberCount: lobby.members.size,
          difficulty: lobby.gameSettings.difficulty,
          totalQuestions: lobby.gameSettings.totalQuestions,
          timePerQuestion: lobby.gameSettings.timePerQuestion,
          category: lobby.gameSettings.category,
          status: lobby.status,
        }),
      );

      return {
        success: true,
        data: lobbies,
        total: lobbies.length,
      };
    },
    {
      detail: {
        tags: ['Quiz Battle'],
        summary: 'Get all active lobbies',
        description: 'Returns list of all currently active game lobbies',
        responses: {
          200: {
            description: 'Successfully retrieved lobbies',
            content: {
              'application/json': {
                schema: {
                  type: 'object',
                  properties: {
                    success: { type: 'boolean' },
                    data: {
                      type: 'array',
                      items: {
                        type: 'object',
                        properties: {
                          lobbyId: { type: 'string' },
                          lobbyCode: { type: 'string' },
                          hostId: { type: 'string' },
                          memberCount: { type: 'number' },
                          difficulty: {
                            type: 'string',
                            enum: ['easy', 'medium', 'hard'],
                          },
                          totalQuestions: { type: 'number' },
                          timePerQuestion: { type: 'number' },
                          category: { type: 'string' },
                          status: {
                            type: 'string',
                            enum: [
                              'waiting',
                              'starting',
                              'playing',
                              'finished',
                            ],
                          },
                        },
                      },
                    },
                    total: { type: 'number' },
                  },
                },
              },
            },
          },
        },
      },
    },
  )
  .get(
    '/matches',
    () => {
      // Get list of all active matches
      const matches = Array.from(wsManager['activeMatches'].values()).map(
        (match) => ({
          matchId: match.matchId,
          player1Id: match.player1Id,
          player2Id: match.player2Id,
          currentQuestion: match.gameState.currentQuestionIndex,
          totalQuestions: match.questions.length,
          status: match.status,
        }),
      );

      return {
        success: true,
        data: matches,
        total: matches.length,
      };
    },
    {
      detail: {
        tags: ['Quiz Battle'],
        summary: 'Get all active matches',
        description: 'Returns list of all currently active quiz battle matches',
        responses: {
          200: {
            description: 'Successfully retrieved matches',
            content: {
              'application/json': {
                schema: {
                  type: 'object',
                  properties: {
                    success: { type: 'boolean' },
                    data: {
                      type: 'array',
                      items: {
                        type: 'object',
                        properties: {
                          matchId: { type: 'string' },
                          player1Id: { type: 'string' },
                          player2Id: { type: 'string' },
                          currentQuestion: { type: 'number' },
                          totalQuestions: { type: 'number' },
                          status: {
                            type: 'string',
                            enum: [
                              'waiting',
                              'playing',
                              'finished',
                              'cancelled',
                            ],
                          },
                        },
                      },
                    },
                    total: { type: 'number' },
                  },
                },
              },
            },
          },
        },
      },
    },
  )
  .get(
    '/queue',
    () => {
      // Get matchmaking queue info
      const queue = Array.from(wsManager['matchmakingQueue'].values()).map(
        (entry) => ({
          userId: entry.userId,
          username: entry.username,
          gameMode: entry.gameMode,
          difficulty: entry.difficulty,
          category: entry.category,
          points: entry.points,
          timestamp: entry.timestamp,
        }),
      );

      return {
        success: true,
        data: queue,
        total: queue.length,
      };
    },
    {
      detail: {
        tags: ['Quiz Battle'],
        summary: 'Get matchmaking queue',
        description: 'Returns list of players currently in matchmaking queue',
        responses: {
          200: {
            description: 'Successfully retrieved queue',
            content: {
              'application/json': {
                schema: {
                  type: 'object',
                  properties: {
                    success: { type: 'boolean' },
                    data: {
                      type: 'array',
                      items: {
                        type: 'object',
                        properties: {
                          userId: { type: 'string' },
                          username: { type: 'string' },
                          gameMode: {
                            type: 'string',
                            enum: ['ranked', 'casual', 'friend'],
                          },
                          difficulty: {
                            type: 'string',
                            enum: ['easy', 'medium', 'hard'],
                          },
                          category: { type: 'string' },
                          points: { type: 'number' },
                          timestamp: { type: 'number' },
                        },
                      },
                    },
                    total: { type: 'number' },
                  },
                },
              },
            },
          },
        },
      },
    },
  )
  .get(
    '/connection-info',
    () => {
      // Get WebSocket connection info
      return {
        success: true,
        data: {
          endpoint: '/api/quiz/battle',
          protocol: 'WebSocket',
          messageFormat: 'JSON',
          description:
            'Connect to this WebSocket endpoint for real-time Quiz Battle features',
          documentation:
            'See API_DOCUMENTATION.md for complete WebSocket message types',
        },
      };
    },
    {
      detail: {
        tags: ['Quiz Battle'],
        summary: 'Get WebSocket connection information',
        description:
          'Returns information about how to connect to Quiz Battle WebSocket',
        responses: {
          200: {
            description: 'Connection information',
            content: {
              'application/json': {
                schema: {
                  type: 'object',
                  properties: {
                    success: { type: 'boolean' },
                    data: {
                      type: 'object',
                      properties: {
                        endpoint: { type: 'string' },
                        protocol: { type: 'string' },
                        messageFormat: { type: 'string' },
                        description: { type: 'string' },
                        documentation: { type: 'string' },
                      },
                    },
                  },
                },
              },
            },
          },
        },
      },
    },
  );

// Cleanup task yang berjalan setiap 5 menit
setInterval(
  () => {
    wsManager.cleanupDisconnectedUsers();
    wsManager.cleanupExpiredLobbies();
    wsManager.printStats();
  },
  5 * 60 * 1000,
);
