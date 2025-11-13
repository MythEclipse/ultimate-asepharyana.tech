// Friend System Handlers

import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  FriendRequestSendPayload,
  FriendRequestReceivedPayload,
  FriendRequestAcceptPayload,
  FriendRequestAcceptedPayload,
  FriendRequestRejectPayload,
  FriendRequestRejectedPayload,
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
 * Handle friend request send
 */
export async function handleFriendRequestSend(
  sessionId: string,
  payload: FriendRequestSendPayload
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Get sender info
    const [senderUser] = await db
      .select()
      .from(users)
      .where(eq(users.id, payload.senderId))
      .limit(1);

    if (!senderUser) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'USER_NOT_FOUND',
          message: 'Sender tidak ditemukan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Check if target user exists
    const [targetUser] = await db
      .select()
      .from(users)
      .where(eq(users.name, payload.targetUsername))
      .limit(1);

    if (!targetUser) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'USER_NOT_FOUND',
          message: 'User tidak ditemukan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Check if already friends or request exists
    const [existingRelation] = await db
      .select()
      .from(quizFriendships)
      .where(
        or(
          and(
            eq(quizFriendships.userId, payload.senderId),
            eq(quizFriendships.friendId, targetUser.id)
          ),
          and(
            eq(quizFriendships.userId, targetUser.id),
            eq(quizFriendships.friendId, payload.senderId)
          )
        )
      )
      .limit(1);

    if (existingRelation) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'FRIEND_REQUEST_EXISTS',
          message:
            existingRelation.status === 'accepted'
              ? 'Sudah berteman'
              : 'Request sudah dikirim',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Create friend request
    const requestId = `fr_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
    await db.insert(quizFriendships).values({
      id: requestId,
      userId: payload.senderId,
      friendId: targetUser.id,
      status: 'pending',
      createdAt: new Date(),
    });

    // Create notification for target user
    const notificationId = `notif_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
    await db.insert(quizNotifications).values({
      id: notificationId,
      userId: targetUser.id,
      type: 'friend_request',
      title: 'Friend Request',
      message: `${payload.senderUsername} mengirim friend request`,
      data: JSON.stringify({
        fromUserId: payload.senderId,
        fromUsername: payload.senderUsername,
        requestId,
      }),
      isRead: 0,
      createdAt: new Date(),
    });

    // Send success to sender
    const successMsg: WSMessage = {
      type: 'success',
      payload: {
        message: `Friend request sent to ${payload.targetUsername}`,
      },
    };
    wsManager.sendToSession(sessionId, successMsg);

    // Send notification to target if online
    const targetSession = wsManager.findSessionByUserId(targetUser.id);
    if (targetSession) {
      // Get sender stats
      const [senderStats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, payload.senderId))
        .limit(1);

      const receivedMsg: WSMessage<FriendRequestReceivedPayload> = {
        type: 'friend.request.received',
        payload: {
          requestId,
          sender: {
            userId: payload.senderId,
            username: payload.senderUsername,
            points: senderStats?.points || 0,
            wins: senderStats?.wins || 0,
            avatarUrl: senderUser.image || undefined,
          },
          message: payload.message,
          timestamp: Date.now(),
        },
      };
      wsManager.sendToSession(targetSession, receivedMsg);
    }

    console.log(`[Friends] Friend request sent from ${payload.senderId} to ${targetUser.id}`);
  } catch (error) {
    console.error('[Friends] Error sending friend request:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengirim friend request',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend request accept
 */
export async function handleFriendRequestAccept(
  sessionId: string,
  payload: FriendRequestAcceptPayload
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Get friendship request
    const [friendship] = await db
      .select()
      .from(quizFriendships)
      .where(eq(quizFriendships.id, payload.requestId))
      .limit(1);

    if (!friendship) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'REQUEST_NOT_FOUND',
          message: 'Friend request tidak ditemukan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Verify it's for this user
    if (friendship.friendId !== payload.userId) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'UNAUTHORIZED',
          message: 'Tidak berhak menerima request ini',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Update status to accepted
    await db
      .update(quizFriendships)
      .set({ status: 'accepted' })
      .where(eq(quizFriendships.id, payload.requestId));

    // Get user info for both users
    const [user1] = await db
      .select()
      .from(users)
      .where(eq(users.id, friendship.userId))
      .limit(1);

    const [user2] = await db
      .select()
      .from(users)
      .where(eq(users.id, friendship.friendId))
      .limit(1);

    // Send accepted message to acceptor
    const acceptedMsg: WSMessage<FriendRequestAcceptedPayload> = {
      type: 'friend.request.accepted',
      payload: {
        friendship: {
          friendshipId: friendship.id,
          user1: {
            userId: friendship.userId,
            username: user1?.name || 'Unknown',
          },
          user2: {
            userId: friendship.friendId,
            username: user2?.name || 'Unknown',
          },
          createdAt: friendship.createdAt.getTime(),
        },
      },
    };
    wsManager.sendToSession(sessionId, acceptedMsg);

    // Notify sender if online
    const senderSession = wsManager.findSessionByUserId(friendship.userId);
    if (senderSession) {
      wsManager.sendToSession(senderSession, acceptedMsg);
    }

    console.log(`[Friends] Friend request ${payload.requestId} accepted by ${payload.userId}`);
  } catch (error) {
    console.error('[Friends] Error accepting friend request:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal menerima friend request',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend request reject
 */
export async function handleFriendRequestReject(
  sessionId: string,
  payload: FriendRequestRejectPayload
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Get friendship request
    const [friendship] = await db
      .select()
      .from(quizFriendships)
      .where(eq(quizFriendships.id, payload.requestId))
      .limit(1);

    if (!friendship) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'REQUEST_NOT_FOUND',
          message: 'Friend request tidak ditemukan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Verify it's for this user
    if (friendship.friendId !== payload.userId) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'UNAUTHORIZED',
          message: 'Tidak berhak menolak request ini',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Delete friendship request
    await db
      .delete(quizFriendships)
      .where(eq(quizFriendships.id, payload.requestId));

    // Send rejected message to rejecter
    const rejectedMsg: WSMessage<FriendRequestRejectedPayload> = {
      type: 'friend.request.rejected',
      payload: {
        requestId: payload.requestId,
        rejectedBy: payload.userId,
      },
    };
    wsManager.sendToSession(sessionId, rejectedMsg);

    // Notify sender if online
    const senderSession = wsManager.findSessionByUserId(friendship.userId);
    if (senderSession) {
      wsManager.sendToSession(senderSession, rejectedMsg);
    }

    console.log(`[Friends] Friend request ${payload.requestId} rejected by ${payload.userId}`);
  } catch (error) {
    console.error('[Friends] Error rejecting friend request:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal menolak friend request',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend remove
 */
export async function handleFriendRemove(
  sessionId: string,
  payload: FriendRemovePayload
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
            eq(quizFriendships.friendId, payload.friendId)
          ),
          and(
            eq(quizFriendships.userId, payload.friendId),
            eq(quizFriendships.friendId, payload.userId)
          )
        )
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

    console.log(`[Friends] Friend removed: ${payload.userId} <-> ${payload.friendId}`);
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
  payload: FriendListSyncPayload
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
            eq(quizFriendships.friendId, payload.userId)
          ),
          eq(quizFriendships.status, 'accepted')
        )
      );

    // Get friend IDs
    const friendIds = friendships.map((f) =>
      f.userId === payload.userId ? f.friendId : f.userId
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
          eq(quizFriendships.status, 'pending')
        )
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

    console.log(`[Friends] Friend list sent to ${payload.userId}: ${friendsData.length} friends`);
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
  payload: FriendChallengeSendPayload
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
              eq(quizFriendships.friendId, payload.targetFriendId)
            ),
            and(
              eq(quizFriendships.userId, payload.targetFriendId),
              eq(quizFriendships.friendId, payload.challengerId)
            )
          ),
          eq(quizFriendships.status, 'accepted')
        )
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

    console.log(`[Friends] Challenge sent from ${payload.challengerId} to ${payload.targetFriendId}`);
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
