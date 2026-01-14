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
  MatchmakingConfirmPayload,
  MatchmakingConfirmRequestPayload,
  MatchmakingConfirmStatusPayload,
} from '../types';
import { wsManager } from '../ws-manager';
import {
  getDb,
  users,
  quizUserStats,
  quizMatches,
  eq,
} from '@asepharyana/services';

// Store pending match confirmations
const pendingConfirmations = new Map<
  string,
  {
    matchId: string;
    player1Id: string;
    player2Id: string;
    player1Confirmed: boolean;
    player2Confirmed: boolean;
    gameSettings: GameSettings;
    expiresAt: number;
    timeoutId: ReturnType<typeof setTimeout>;
  }
>();

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
        status: 'pending_confirm', // Changed to pending_confirm
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
        },
        questions: [],
        currentQuestionStartTime: 0,
        status: 'waiting',
      };

      wsManager.createMatch(matchId, matchState);

      // Create pending confirmation with 30 second timeout
      const timeoutId = setTimeout(() => {
        handleConfirmationTimeout(matchId);
      }, 30000);

      pendingConfirmations.set(matchId, {
        matchId,
        player1Id: payload.userId,
        player2Id: matchFound.userId,
        player1Confirmed: false,
        player2Confirmed: false,
        gameSettings,
        expiresAt: Date.now() + 30000,
        timeoutId,
      });

      // Send confirmation request to both players
      const confirmReqPlayer1: WSMessage<MatchmakingConfirmRequestPayload> = {
        type: 'matchmaking.confirm.request',
        payload: {
          matchId,
          opponent: opponentInfo,
          gameSettings,
          timeToConfirm: 30,
        },
      };

      const confirmReqPlayer2: WSMessage<MatchmakingConfirmRequestPayload> = {
        type: 'matchmaking.confirm.request',
        payload: {
          matchId,
          opponent: userInfo,
          gameSettings,
          timeToConfirm: 30,
        },
      };

      wsManager.sendToUser(payload.userId, confirmReqPlayer1);
      wsManager.sendToUser(matchFound.userId, confirmReqPlayer2);

      console.log(
        `[Matchmaking] Match found, waiting for confirmation: ${connection.username} vs ${opponentConnection.username}`,
      );
    } else {
      // No match found, add to queue
      await addUserToQueue(payload, userPoints);
    }
  } catch (error) {
    console.error('[Matchmaking] Error finding match:', error);
  }
}

/**
 * Handle match confirmation from player
 */
export async function handleMatchmakingConfirm(
  sessionId: string,
  payload: MatchmakingConfirmPayload,
): Promise<void> {
  try {
    const pending = pendingConfirmations.get(payload.matchId);
    if (!pending) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'MATCH_NOT_FOUND',
          message: 'Match tidak ditemukan atau sudah kadaluarsa',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Determine which player confirmed
    const isPlayer1 = payload.userId === pending.player1Id;
    const isPlayer2 = payload.userId === pending.player2Id;

    if (!isPlayer1 && !isPlayer2) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'NOT_IN_MATCH',
          message: 'Anda tidak terdaftar di match ini',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    if (!payload.confirmed) {
      // Player declined
      clearTimeout(pending.timeoutId);
      pendingConfirmations.delete(payload.matchId);

      // Notify both players
      const declinedMsg: WSMessage<MatchmakingConfirmStatusPayload> = {
        type: 'matchmaking.confirm.status',
        payload: {
          matchId: payload.matchId,
          playerConfirmed: false,
          opponentConfirmed: false,
          status: 'declined',
        },
      };

      wsManager.sendToUser(pending.player1Id, declinedMsg);
      wsManager.sendToUser(pending.player2Id, declinedMsg);

      // Clean up match
      wsManager.deleteMatch(payload.matchId);

      // Update database status
      const db = getDb();
      await db
        .update(quizMatches)
        .set({ status: 'cancelled' })
        .where(eq(quizMatches.id, payload.matchId));

      // Reset statuses
      wsManager.updateUserStatus(pending.player1Id, 'online');
      wsManager.updateUserStatus(pending.player2Id, 'online');

      console.log(
        `[Matchmaking] Match ${payload.matchId} declined by ${payload.userId}`,
      );
      return;
    }

    // Player confirmed
    if (isPlayer1) {
      pending.player1Confirmed = true;
    } else {
      pending.player2Confirmed = true;
    }

    // Send status update to both players
    const statusMsgPlayer1: WSMessage<MatchmakingConfirmStatusPayload> = {
      type: 'matchmaking.confirm.status',
      payload: {
        matchId: payload.matchId,
        playerConfirmed: pending.player1Confirmed,
        opponentConfirmed: pending.player2Confirmed,
        status: 'waiting',
      },
    };

    const statusMsgPlayer2: WSMessage<MatchmakingConfirmStatusPayload> = {
      type: 'matchmaking.confirm.status',
      payload: {
        matchId: payload.matchId,
        playerConfirmed: pending.player2Confirmed,
        opponentConfirmed: pending.player1Confirmed,
        status: 'waiting',
      },
    };

    wsManager.sendToUser(pending.player1Id, statusMsgPlayer1);
    wsManager.sendToUser(pending.player2Id, statusMsgPlayer2);

    // Check if both confirmed
    if (pending.player1Confirmed && pending.player2Confirmed) {
      clearTimeout(pending.timeoutId);
      pendingConfirmations.delete(payload.matchId);

      // Update user statuses
      wsManager.updateUserStatus(pending.player1Id, 'in_game');
      wsManager.updateUserStatus(pending.player2Id, 'in_game');

      // Send both confirmed message
      const bothConfirmedMsg: WSMessage<MatchmakingConfirmStatusPayload> = {
        type: 'matchmaking.confirm.status',
        payload: {
          matchId: payload.matchId,
          playerConfirmed: true,
          opponentConfirmed: true,
          status: 'both_confirmed',
        },
      };

      wsManager.sendToUser(pending.player1Id, bothConfirmedMsg);
      wsManager.sendToUser(pending.player2Id, bothConfirmedMsg);

      console.log(
        `[Matchmaking] Match ${payload.matchId} confirmed by both players`,
      );

      // Start game after short delay
      setTimeout(() => {
        startGame(payload.matchId);
      }, 3000);
    }
  } catch (error) {
    console.error('[Matchmaking] Error handling confirmation:', error);
  }
}

/**
 * Handle confirmation timeout
 */
function handleConfirmationTimeout(matchId: string): void {
  const pending = pendingConfirmations.get(matchId);
  if (!pending) return;

  pendingConfirmations.delete(matchId);

  // Notify both players of timeout
  const timeoutMsg: WSMessage<MatchmakingConfirmStatusPayload> = {
    type: 'matchmaking.confirm.status',
    payload: {
      matchId,
      playerConfirmed: pending.player1Confirmed,
      opponentConfirmed: pending.player2Confirmed,
      status: 'timeout',
    },
  };

  wsManager.sendToUser(pending.player1Id, timeoutMsg);
  wsManager.sendToUser(pending.player2Id, timeoutMsg);

  // Clean up match
  wsManager.deleteMatch(matchId);

  // Reset statuses
  wsManager.updateUserStatus(pending.player1Id, 'online');
  wsManager.updateUserStatus(pending.player2Id, 'online');

  console.log(
    `[Matchmaking] Match ${matchId} timed out waiting for confirmation`,
  );
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
export async function startGame(matchId: string): Promise<void> {
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
