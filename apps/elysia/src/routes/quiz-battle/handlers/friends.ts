// Friend System Handlers

import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  FriendRemovePayload,
  FriendRemovedPayload,
  FriendListSyncPayload,
  FriendListDataPayload,
  FriendChallengeSendPayload,
  FriendChallengeReceivedPayload,
  FriendInfo,
  UserStatus,
} from '../types';
import {
  getDb,
  quizFriendships,
  quizNotifications,
  users,
  quizUserStats,
  eq,
  and,
  or,
} from '@asepharyana/services';

/**
 * Handle friend remove
 */
export async function handleFriendRemove(
  sessionId: string,
  payload: FriendRemovePayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Find and delete friendship (bidirectional)
    await db
      .delete(quizFriendships)
      .where(
        or(
          and(
            eq(quizFriendships.userId, payload.userId),
            eq(quizFriendships.friendId, payload.friendId),
          ),
          and(
            eq(quizFriendships.userId, payload.friendId),
            eq(quizFriendships.friendId, payload.userId),
          ),
        ),
      );

    // Send removed message to remover
    const removedMsg: WSMessage<FriendRemovedPayload> = {
      type: 'friend.removed',
      payload: {
        removedFriendId: payload.friendId,
        removedBy: payload.userId,
      },
    };
    wsManager.sendToSession(sessionId, removedMsg);

    // Notify removed friend if online
    const friendSession = wsManager.findSessionByUserId(payload.friendId);
    if (friendSession) {
      const friendRemovedMsg: WSMessage<FriendRemovedPayload> = {
        type: 'friend.removed',
        payload: {
          removedFriendId: payload.userId,
          removedBy: payload.userId,
        },
      };
      wsManager.sendToSession(friendSession, friendRemovedMsg);
    }

    console.log(
      `[Friends] Friend removed: ${payload.userId} <-> ${payload.friendId}`,
    );
  } catch (error) {
    console.error('[Friends] Error removing friend:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal menghapus teman',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend list request
 */
export async function handleFriendListRequest(
  sessionId: string,
  payload: FriendListSyncPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Get all accepted friendships
    const friendships = await db
      .select()
      .from(quizFriendships)
      .where(
        and(
          or(
            eq(quizFriendships.userId, payload.userId),
            eq(quizFriendships.friendId, payload.userId),
          ),
          eq(quizFriendships.status, 'accepted'),
        ),
      );

    // Get friend IDs
    const friendIds = friendships.map((f) =>
      f.userId === payload.userId ? f.friendId : f.userId,
    );

    // Get friends data with stats
    const friendsData: FriendInfo[] = [];
    for (const friendId of friendIds) {
      const [user] = await db
        .select()
        .from(users)
        .where(eq(users.id, friendId))
        .limit(1);

      if (!user) continue;

      const [stats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, friendId))
        .limit(1);

      // Get status from WebSocket manager
      const isOnline = wsManager.findSessionByUserId(friendId) !== undefined;
      const status: UserStatus = isOnline ? 'online' : 'offline';

      friendsData.push({
        userId: friendId,
        username: user.name || 'Unknown',
        status,
        points: stats?.points || 0,
        wins: stats?.wins || 0,
        lastSeen: Date.now(), // TODO: track actual last seen
        avatarUrl: user.image || undefined,
      });
    }

    // Get pending requests count
    const pendingRequests = await db
      .select()
      .from(quizFriendships)
      .where(
        and(
          eq(quizFriendships.friendId, payload.userId),
          eq(quizFriendships.status, 'pending'),
        ),
      );

    // Send friend list
    const listMsg: WSMessage<FriendListDataPayload> = {
      type: 'friend.list.data',
      payload: {
        friends: friendsData,
        pendingRequests: pendingRequests.length,
        totalFriends: friendsData.length,
      },
    };
    wsManager.sendToSession(sessionId, listMsg);

    console.log(
      `[Friends] Friend list sent to ${payload.userId}: ${friendsData.length} friends`,
    );
  } catch (error) {
    console.error('[Friends] Error getting friend list:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengambil daftar teman',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend challenge
 */
export async function handleFriendChallenge(
  sessionId: string,
  payload: FriendChallengeSendPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Check if they are friends
    const [friendship] = await db
      .select()
      .from(quizFriendships)
      .where(
        and(
          or(
            and(
              eq(quizFriendships.userId, payload.challengerId),
              eq(quizFriendships.friendId, payload.targetFriendId),
            ),
            and(
              eq(quizFriendships.userId, payload.targetFriendId),
              eq(quizFriendships.friendId, payload.challengerId),
            ),
          ),
          eq(quizFriendships.status, 'accepted'),
        ),
      )
      .limit(1);

    if (!friendship) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'NOT_FRIENDS',
          message: 'Hanya bisa challenge teman',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Check if friend is online
    const targetSession = wsManager.findSessionByUserId(payload.targetFriendId);
    if (!targetSession) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'USER_OFFLINE',
          message: 'Teman tidak online',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Get challenger stats
    const [challengerStats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, payload.challengerId))
      .limit(1);

    // Generate challenge ID
    const challengeId = `challenge_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;

    // Send challenge to target friend
    const challengeMsg: WSMessage<FriendChallengeReceivedPayload> = {
      type: 'friend.challenge.received',
      payload: {
        challengeId,
        challenger: {
          userId: payload.challengerId,
          username: payload.challengerUsername,
          points: challengerStats?.points || 0,
          wins: challengerStats?.wins || 0,
        },
        gameSettings: payload.gameSettings,
        message: payload.message,
        expiresIn: 60000, // 60 seconds
      },
    };
    wsManager.sendToSession(targetSession, challengeMsg);

    // Send success to challenger
    const successMsg: WSMessage = {
      type: 'success',
      payload: {
        message: `Challenge sent to ${payload.targetFriendId}`,
      },
    };
    wsManager.sendToSession(sessionId, successMsg);

    console.log(
      `[Friends] Challenge sent from ${payload.challengerId} to ${payload.targetFriendId}`,
    );
  } catch (error) {
    console.error('[Friends] Error sending challenge:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengirim challenge',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}
