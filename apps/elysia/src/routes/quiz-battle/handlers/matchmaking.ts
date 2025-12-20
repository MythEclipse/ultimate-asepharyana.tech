// Matchmaking Handlers

import type {
  WSMessage,
  MatchmakingFindPayload,
  MatchmakingSearchingPayload,
  MatchmakingFoundPayload,
  MatchmakingCancelPayload,
  MatchmakingCancelledPayload,
  MatchmakingQueueEntry,
  OpponentInfo,
  GameSettings,
  MatchState,
} from '../types';
import { wsManager } from '../ws-manager';
import {
  getDb,
  users,
  quizUserStats,
  quizMatches,
  eq,
} from '@asepharyana/services';

// Generate unique match ID
function generateMatchId(): string {
  return `match_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
}

// Get opponent info from database
async function getOpponentInfo(userId: string): Promise<OpponentInfo | null> {
  try {
    const db = getDb();

    const [user] = await db
      .select()
      .from(users)
      .where(eq(users.id, userId))
      .limit(1);

    if (!user) return null;

    const [stats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, userId))
      .limit(1);

    // Get username from connection if DB name is null
    const connection = wsManager.getConnectionByUserId(userId);
    const username =
      user.name ||
      connection?.username ||
      user.email?.split('@')[0] ||
      'Player';

    return {
      userId: user.id,
      username,
      points: stats?.points || 0,
      wins: stats?.wins || 0,
      losses: stats?.losses || 0,
      avatarUrl: user.image || undefined,
    };
  } catch (error) {
    console.error('[Matchmaking] Error getting opponent info:', error);
    return null;
  }
}

export async function handleMatchmakingFind(
  sessionId: string,
  payload: MatchmakingFindPayload,
): Promise<void> {
  try {
    // Use userId from payload to find connection (sessionId from index.ts is unreliable)
    const connection = wsManager.getConnectionByUserId(payload.userId);
    if (!connection) {
      console.log(
        `[Matchmaking] No connection found for user ${payload.userId}`,
      );
      return;
    }

    // Check if user is already in match or lobby
    if (connection.currentMatchId || connection.currentLobbyId) {
      const errorMsg: WSMessage = {
        type: 'matchmaking.error',
        payload: {
          code: 'ALREADY_IN_GAME',
          message: 'Anda sudah berada dalam game atau lobby',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Get user stats for matchmaking
    const db = getDb();
    const [stats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, payload.userId))
      .limit(1);

    const userPoints = stats?.points || 0;

    // Try to find match in queue (pass MMR for ranked mode)
    const matchFound = wsManager.findMatchInQueue(
      payload.userId,
      payload.gameMode,
      payload.difficulty,
      payload.category,
      payload.gameMode === 'ranked' ? userPoints : undefined,
    );

    if (matchFound) {
      // Found a match!
      wsManager.removeFromQueue(matchFound.userId);

      // Get opponent connection
      const opponentConnection = wsManager.getConnectionByUserId(
        matchFound.userId,
      );
      if (!opponentConnection) {
        // Opponent disconnected, add user to queue instead
        await addUserToQueue(payload, userPoints);
        return;
      }

      // Get opponent info
      const opponentInfo = await getOpponentInfo(matchFound.userId);
      const userInfo = await getOpponentInfo(payload.userId);

      if (!opponentInfo || !userInfo) {
        console.error('[Matchmaking] Failed to get player info');
        return;
      }

      // Create match in database
      const matchId = generateMatchId();
      const gameSettings: GameSettings = {
        totalQuestions: 5,
        timePerQuestion: 10,
        difficulty: payload.difficulty,
        category: payload.category,
      };

      await db.insert(quizMatches).values({
        id: matchId,
        player1Id: payload.userId,
        player2Id: matchFound.userId,
        gameMode: payload.gameMode,
        difficulty: payload.difficulty,
        category: payload.category,
        status: 'waiting',
        player1Score: 0,
        player2Score: 0,
        player1Health: 100,
        player2Health: 100,
        currentQuestionIndex: 0,
        totalQuestions: gameSettings.totalQuestions,
        timePerQuestion: gameSettings.timePerQuestion,
      });

      // Create match state in memory
      const matchState: MatchState = {
        matchId,
        player1Id: payload.userId,
        player2Id: matchFound.userId,
        player1: connection,
        player2: opponentConnection,
        gameState: {
          totalQuestions: gameSettings.totalQuestions,
          currentQuestionIndex: 0,
          timePerQuestion: gameSettings.timePerQuestion,
          playerHealth: 100,
          opponentHealth: 100,
          playerScore: 0,
          opponentScore: 0,
        },
        questions: [],
        currentQuestionStartTime: 0,
        status: 'waiting',
      };

      wsManager.createMatch(matchId, matchState);

      // Update user statuses
      wsManager.updateUserStatus(payload.userId, 'in_game');
      wsManager.updateUserStatus(matchFound.userId, 'in_game');

      // Send match found to both players
      const matchFoundMsgPlayer1: WSMessage<MatchmakingFoundPayload> = {
        type: 'matchmaking.found',
        payload: {
          matchId,
          opponent: opponentInfo,
          gameSettings,
          startIn: 5,
        },
      };

      const matchFoundMsgPlayer2: WSMessage<MatchmakingFoundPayload> = {
        type: 'matchmaking.found',
        payload: {
          matchId,
          opponent: userInfo,
          gameSettings,
          startIn: 5,
        },
      };

      wsManager.sendToUser(payload.userId, matchFoundMsgPlayer1);
      wsManager.sendToUser(matchFound.userId, matchFoundMsgPlayer2);

      console.log(
        `[Matchmaking] Match found: ${connection.username} vs ${opponentConnection.username}`,
      );

      // Start game after 5 seconds
      setTimeout(() => {
        startGame(matchId);
      }, 5000);
    } else {
      // No match found, add to queue
      await addUserToQueue(payload, userPoints);
    }
  } catch (error) {
    console.error('[Matchmaking] Error finding match:', error);
  }
}

async function addUserToQueue(
  payload: MatchmakingFindPayload,
  points: number,
): Promise<void> {
  const queueEntry: MatchmakingQueueEntry = {
    userId: payload.userId,
    username: payload.userId, // We'll get username from connection
    gameMode: payload.gameMode,
    difficulty: payload.difficulty,
    category: payload.category,
    points,
    timestamp: Date.now(),
  };

  wsManager.addToQueue(queueEntry);

  // Send searching message
  const searchingMsg: WSMessage<MatchmakingSearchingPayload> = {
    type: 'matchmaking.searching',
    payload: {
      estimatedWaitTime: 30,
      playersInQueue: wsManager.getQueueSize(),
    },
  };

  wsManager.sendToUser(payload.userId, searchingMsg);

  console.log(`[Matchmaking] User ${payload.userId} added to queue`);
}

export function handleMatchmakingCancel(
  sessionId: string,
  payload: MatchmakingCancelPayload,
): void {
  try {
    wsManager.removeFromQueue(payload.userId);

    const cancelledMsg: WSMessage<MatchmakingCancelledPayload> = {
      type: 'matchmaking.cancelled',
      payload: {
        reason: 'user_cancelled',
      },
    };

    wsManager.sendToSession(sessionId, cancelledMsg);

    console.log(`[Matchmaking] User ${payload.userId} cancelled matchmaking`);
  } catch (error) {
    console.error('[Matchmaking] Error cancelling matchmaking:', error);
  }
}

// This will be called from game handlers
async function startGame(matchId: string): Promise<void> {
  try {
    const match = wsManager.getMatch(matchId);
    if (!match) return;

    // Update match status
    match.status = 'playing';

    const db = getDb();
    await db
      .update(quizMatches)
      .set({
        status: 'playing',
        startedAt: new Date(),
      })
      .where(eq(quizMatches.id, matchId));

    // Load questions for this match
    // This will be implemented in game handlers
    const gameModule = await import('./game');
    const startGameMatch = gameModule.startGameMatch;
    await startGameMatch(matchId);
  } catch (error) {
    console.error('[Matchmaking] Error starting game:', error);
  }
}
