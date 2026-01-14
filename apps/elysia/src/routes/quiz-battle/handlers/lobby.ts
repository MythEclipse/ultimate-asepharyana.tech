// Lobby System Handlers
// Simplified version focusing on core functionality

import { wsManager } from '../ws-manager';
import { lobbyLogger } from '../../../utils/logger';
import type {
  WSMessage,
  LobbyCreatePayload,
  LobbyCreatedPayload,
  LobbyJoinPayload,
  LobbyPlayerJoinedPayload,
  LobbyReadyPayload,
  LobbyPlayerReadyPayload,
  LobbyStartPayload,
  LobbyGameStartingPayload,
  LobbyLeavePayload,
  LobbyKickPayload,
  LobbyListDataPayload,
  LobbyPlayer,
  LobbyInfo,
  LobbyState,
  Difficulty,
} from '../types';
import {
  getDb,
  quizLobbies,
  quizLobbyMembers,
  users,
  quizUserStats,
  eq,
  and,
} from '@asepharyana/services';

/**
 * Generate unique lobby code
 */
function generateLobbyCode(): string {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  let code = '';
  for (let i = 0; i < 6; i++) {
    code += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return code;
}

/**
 * Calculate expiry time (30 minutes from now)
 */
function getExpiryTime(): Date {
  const now = new Date();
  return new Date(now.getTime() + 30 * 60 * 1000);
}

/**
 * Handle lobby creation
 */
export async function handleLobbyCreate(
  sessionId: string,
  payload: LobbyCreatePayload
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Generate unique lobby code
    let lobbyCode = generateLobbyCode();
    let attempts = 0;
    while (attempts < 10) {
      const [existing] = await db
        .select()
        .from(quizLobbies)
        .where(eq(quizLobbies.lobbyCode, lobbyCode))
        .limit(1);

      if (!existing) break;
      lobbyCode = generateLobbyCode();
      attempts++;
    }

    if (attempts >= 10) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'LOBBY_CODE_GENERATION_FAILED',
          message: 'Gagal generate lobby code, coba lagi',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Create lobby in database
    const lobbyId = `lobby_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
    await db.insert(quizLobbies).values({
      id: lobbyId,
      lobbyCode,
      hostId: payload.hostId,
      isPrivate: payload.isPrivate ? 1 : 0,
      maxPlayers: payload.maxPlayers,
      status: 'waiting',
      difficulty: payload.gameSettings.difficulty,
      category: payload.gameSettings.category || 'General',
      totalQuestions: payload.gameSettings.totalQuestions,
      timePerQuestion: payload.gameSettings.timePerQuestion,
      createdAt: new Date(),
      expiresAt: getExpiryTime(),
    });

    // Add host as first member
    await db.insert(quizLobbyMembers).values({
      id: `member_${Date.now()}`,
      lobbyId,
      userId: payload.hostId,
      isHost: 1,
      isReady: 1, // Host is always ready
      joinedAt: new Date(),
    });

    // Create lobby state in WSManager
    const lobbyState: LobbyState = {
      lobbyId,
      lobbyCode,
      hostId: payload.hostId,
      members: new Map(),
      gameSettings: payload.gameSettings,
      status: 'waiting',
    };

    lobbyState.members.set(payload.hostId, {
      userId: payload.hostId,
      username: payload.hostUsername,
      isHost: true,
      isReady: true,
    });

    wsManager.createLobby(lobbyId, lobbyState);
    wsManager.addLobbyMember(lobbyId, payload.hostId);

    // Send success response
    const createdMsg: WSMessage<LobbyCreatedPayload> = {
      type: 'lobby.created',
      payload: {
        lobbyId,
        lobbyCode,
        hostId: payload.hostId,
        createdAt: Date.now(),
      },
    };
    wsManager.sendToSession(sessionId, createdMsg);

    lobbyLogger.created(lobbyId, payload.hostId);
  } catch (error) {
    console.error('[Lobby] Error creating lobby:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal membuat lobby',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle lobby join - minimal implementation
 */
export async function handleLobbyJoin(
  sessionId: string,
  payload: LobbyJoinPayload
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Find lobby by code
    const [lobby] = await db
      .select()
      .from(quizLobbies)
      .where(eq(quizLobbies.lobbyCode, payload.lobbyCode))
      .limit(1);

    if (!lobby) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'LOBBY_NOT_FOUND',
          message: 'Lobby tidak ditemukan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Add member to database
    await db.insert(quizLobbyMembers).values({
      id: `member_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`,
      lobbyId: lobby.id,
      userId: payload.userId,
      isHost: 0,
      isReady: 0,
      joinedAt: new Date(),
    });

    // Add to WSManager
    wsManager.addLobbyMember(lobby.id, payload.userId);

    // Get all players
    const allMembers = await db
      .select()
      .from(quizLobbyMembers)
      .where(eq(quizLobbyMembers.lobbyId, lobby.id));

    const players: LobbyPlayer[] = [];
    for (const member of allMembers) {
      const [user] = await db.select().from(users).where(eq(users.id, member.userId)).limit(1);
      const [stats] = await db.select().from(quizUserStats).where(eq(quizUserStats.userId, member.userId)).limit(1);

      players.push({
        userId: member.userId,
        username: user?.name || 'Unknown',
        isHost: member.isHost === 1,
        isReady: member.isReady === 1,
        points: stats?.points || 0,
        avatarUrl: user?.image || undefined,
      });
    }

    const joinedMsg: WSMessage<LobbyPlayerJoinedPayload> = {
      type: 'lobby.player.joined',
      payload: {
        lobbyId: lobby.id,
        player: players.find((p) => p.userId === payload.userId) || players[0],
        players,
      },
    };
    wsManager.sendToSession(sessionId, joinedMsg);

    lobbyLogger.joined(lobby.id, payload.userId);
  } catch (error) {
    console.error('[Lobby] Error joining lobby:', error);
  }
}

/**
 * Handle player ready toggle
 */
export async function handleLobbyReady(
  sessionId: string,
  payload: LobbyReadyPayload
): Promise<void> {
  try {
    const db = getDb();

    await db
      .update(quizLobbyMembers)
      .set({ isReady: payload.isReady ? 1 : 0 })
      .where(
        and(
          eq(quizLobbyMembers.lobbyId, payload.lobbyId),
          eq(quizLobbyMembers.userId, payload.userId)
        )
      );

    const members = await db
      .select()
      .from(quizLobbyMembers)
      .where(eq(quizLobbyMembers.lobbyId, payload.lobbyId));

    const allReady = members.every((m) => m.isReady === 1);

    const readyMsg: WSMessage<LobbyPlayerReadyPayload> = {
      type: 'lobby.player.ready',
      payload: {
        userId: payload.userId,
        isReady: payload.isReady,
        allReady,
      },
    };

    wsManager.sendToSession(sessionId, readyMsg);
    console.log(`[Lobby] ${payload.userId} ready: ${payload.isReady}`);
  } catch (error) {
    console.error('[Lobby] Error updating ready:', error);
  }
}

/**
 * Handle lobby start (simplified)
 */
export async function handleLobbyStart(
  sessionId: string,
  payload: LobbyStartPayload
): Promise<void> {
  try {
    const db = getDb();

    const [lobby] = await db
      .select()
      .from(quizLobbies)
      .where(eq(quizLobbies.id, payload.lobbyId))
      .limit(1);

    if (!lobby || lobby.hostId !== payload.hostId) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'UNAUTHORIZED',
          message: 'Hanya host yang bisa start game',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    const members = await db
      .select()
      .from(quizLobbyMembers)
      .where(eq(quizLobbyMembers.lobbyId, payload.lobbyId));

    const allReady = members.every((m) => m.isReady === 1);
    if (!allReady || members.length < 2) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'NOT_READY',
          message: 'Minimal 2 player dan semua harus ready',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    await db
      .update(quizLobbies)
      .set({ status: 'in_game' })
      .where(eq(quizLobbies.id, payload.lobbyId));

    const matchId = `match_${Date.now()}`;
    const startingMsg: WSMessage<LobbyGameStartingPayload> = {
      type: 'lobby.game.starting',
      payload: {
        matchId,
        countdown: 3,
      },
    };

    wsManager.sendToSession(sessionId, startingMsg);
    lobbyLogger.started(payload.lobbyId);
  } catch (error) {
    console.error('[Lobby] Error starting lobby:', error);
  }
}

/**
 * Handle lobby leave
 */
export async function handleLobbyLeave(
  sessionId: string,
  payload: LobbyLeavePayload
): Promise<void> {
  try {
    const db = getDb();

    await db
      .delete(quizLobbyMembers)
      .where(
        and(
          eq(quizLobbyMembers.lobbyId, payload.lobbyId),
          eq(quizLobbyMembers.userId, payload.userId)
        )
      );

    wsManager.removeLobbyMember(payload.lobbyId, payload.userId);

    lobbyLogger.left(payload.lobbyId, payload.userId);
  } catch (error) {
    console.error('[Lobby] Error leaving lobby:', error);
  }
}

/**
 * Handle lobby kick
 */
export async function handleLobbyKick(
  sessionId: string,
  payload: LobbyKickPayload
): Promise<void> {
  try {
    const db = getDb();

    const [lobby] = await db
      .select()
      .from(quizLobbies)
      .where(eq(quizLobbies.id, payload.lobbyId))
      .limit(1);

    if (!lobby || lobby.hostId !== payload.hostId) {
      return;
    }

    await db
      .delete(quizLobbyMembers)
      .where(
        and(
          eq(quizLobbyMembers.lobbyId, payload.lobbyId),
          eq(quizLobbyMembers.userId, payload.targetUserId)
        )
      );

    wsManager.removeLobbyMember(payload.lobbyId, payload.targetUserId);
    console.log(`[Lobby] ${payload.targetUserId} kicked from ${payload.lobbyId}`);
  } catch (error) {
    console.error('[Lobby] Error kicking player:', error);
  }
}

/**
 * Handle lobby list request
 */
export async function handleLobbyListSync(
  sessionId: string
): Promise<void> {
  try {
    const db = getDb();

    const allLobbies = await db
      .select()
      .from(quizLobbies)
      .where(
        and(
          eq(quizLobbies.status, 'waiting'),
          eq(quizLobbies.isPrivate, 0)
        )
      );

    const lobbies: LobbyInfo[] = [];
    for (const lobby of allLobbies) {
      const [host] = await db.select().from(users).where(eq(users.id, lobby.hostId)).limit(1);
      const members = await db.select().from(quizLobbyMembers).where(eq(quizLobbyMembers.lobbyId, lobby.id));

      lobbies.push({
        lobbyId: lobby.id,
        lobbyCode: lobby.lobbyCode,
        hostUsername: host?.name || 'Unknown',
        currentPlayers: members.length,
        maxPlayers: lobby.maxPlayers,
        isPrivate: lobby.isPrivate === 1,
        gameSettings: {
          difficulty: lobby.difficulty as Difficulty,
          category: lobby.category,
          totalQuestions: lobby.totalQuestions,
          timePerQuestion: lobby.timePerQuestion,
        },
      });
    }

    const listMsg: WSMessage<LobbyListDataPayload> = {
      type: 'lobby.list.data',
      payload: {
        lobbies,
        totalLobbies: lobbies.length,
      },
    };
    wsManager.sendToSession(sessionId, listMsg);

    console.log(`[Lobby] Sent lobby list: ${lobbies.length} lobbies`);
  } catch (error) {
    console.error('[Lobby] Error getting lobby list:', error);
  }
}
