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
  FriendRequestSendPayload,
  FriendRequestReceivedPayload,
  FriendRequestRespondPayload,
  FriendRequestResponsePayload,
  FriendRequestListPayload,
  FriendRequestListDataPayload,
  MatchInviteSendPayload,
  MatchInviteReceivedPayload,
  MatchInviteRespondPayload,
  MatchInviteAcceptedPayload,
  MatchInviteRejectedPayload,
  MatchState,
  GameSettings,
} from '../types';
import {
  getDb,
  quizFriendships,
  quizNotifications,
  quizMatches,
  users,
  quizUserStats,
  eq,
  and,
  or,
} from '../../../services';
import { friendLogger } from '../../../utils/logger';

// Store pending match invites in memory
const pendingMatchInvites = new Map<
  string,
  {
    inviteId: string;
    senderId: string;
    receiverId: string;
    gameSettings: GameSettings;
    message?: string;
    expiresAt: number;
  }
>();

/**
 * Handle friend request send
 */
export async function handleFriendRequestSend(
  sessionId: string,
  payload: FriendRequestSendPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Check if already friends or pending
    const [existing] = await db
      .select()
      .from(quizFriendships)
      .where(
        or(
          and(
            eq(quizFriendships.userId, payload.userId),
            eq(quizFriendships.friendId, payload.targetUserId),
          ),
          and(
            eq(quizFriendships.userId, payload.targetUserId),
            eq(quizFriendships.friendId, payload.userId),
          ),
        ),
      )
      .limit(1);

    if (existing) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'ALREADY_EXISTS',
          message:
            existing.status === 'accepted'
              ? 'Sudah berteman'
              : 'Permintaan pertemanan sudah dikirim',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Get sender info
    const [sender] = await db
      .select()
      .from(users)
      .where(eq(users.id, payload.userId))
      .limit(1);

    if (!sender) return;

    const [senderStats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, payload.userId))
      .limit(1);

    // Create friend request
    const requestId = `fr_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
    await db.insert(quizFriendships).values({
      id: requestId,
      userId: payload.userId,
      friendId: payload.targetUserId,
      status: 'pending',
    });

    // Create notification
    const notificationId = `notif_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
    await db.insert(quizNotifications).values({
      id: notificationId,
      userId: payload.targetUserId,
      type: 'friend_request',
      title: 'Permintaan Pertemanan',
      message: `${sender.name || 'Seseorang'} ingin berteman dengan kamu`,
      data: JSON.stringify({ requestId, senderId: payload.userId }),
      isRead: 0,
    });

    // Notify target if online
    const targetSession = wsManager.findSessionByUserId(payload.targetUserId);
    if (targetSession) {
      const requestMsg: WSMessage<FriendRequestReceivedPayload> = {
        type: 'friend.request.received',
        payload: {
          requestId,
          sender: {
            userId: payload.userId,
            username: sender.name || 'Unknown',
            points: senderStats?.points || 0,
            avatarUrl: sender.image || undefined,
          },
          message: payload.message,
          timestamp: Date.now(),
        },
      };
      wsManager.sendToSession(targetSession, requestMsg);
    }

    // Send success to sender
    const successMsg: WSMessage = {
      type: 'friend.request.sent',
      payload: {
        requestId,
        targetUserId: payload.targetUserId,
        status: 'pending',
      },
    };
    wsManager.sendToSession(sessionId, successMsg);

    friendLogger.requestSent(payload.userId, payload.targetUserId);
  } catch (error) {
    friendLogger.error('sending friend request', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengirim permintaan pertemanan',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend request respond (accept/reject)
 */
export async function handleFriendRequestRespond(
  sessionId: string,
  payload: FriendRequestRespondPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    // Find the request
    const [request] = await db
      .select()
      .from(quizFriendships)
      .where(
        and(
          eq(quizFriendships.id, payload.requestId),
          eq(quizFriendships.friendId, payload.userId),
          eq(quizFriendships.status, 'pending'),
        ),
      )
      .limit(1);

    if (!request) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'REQUEST_NOT_FOUND',
          message: 'Permintaan tidak ditemukan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    if (payload.accept) {
      // Accept friend request
      await db
        .update(quizFriendships)
        .set({ status: 'accepted' })
        .where(eq(quizFriendships.id, payload.requestId));

      // Get friend info
      const [friend] = await db
        .select()
        .from(users)
        .where(eq(users.id, request.userId))
        .limit(1);

      const [friendStats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, request.userId))
        .limit(1);

      const friendInfo: FriendInfo = {
        userId: request.userId,
        username: friend?.name || 'Unknown',
        status: wsManager.findSessionByUserId(request.userId)
          ? 'online'
          : 'offline',
        points: friendStats?.points || 0,
        wins: friendStats?.wins || 0,
        lastSeen: Date.now(),
        avatarUrl: friend?.image || undefined,
      };

      // Send response to accepter
      const responseMsg: WSMessage<FriendRequestResponsePayload> = {
        type: 'friend.request.response',
        payload: {
          requestId: payload.requestId,
          status: 'accepted',
          friend: friendInfo,
        },
      };
      wsManager.sendToSession(sessionId, responseMsg);

      // Notify sender that request was accepted
      const senderSession = wsManager.findSessionByUserId(request.userId);
      if (senderSession) {
        const [accepter] = await db
          .select()
          .from(users)
          .where(eq(users.id, payload.userId))
          .limit(1);

        const [accepterStats] = await db
          .select()
          .from(quizUserStats)
          .where(eq(quizUserStats.userId, payload.userId))
          .limit(1);

        const accepterInfo: FriendInfo = {
          userId: payload.userId,
          username: accepter?.name || 'Unknown',
          status: 'online',
          points: accepterStats?.points || 0,
          wins: accepterStats?.wins || 0,
          lastSeen: Date.now(),
          avatarUrl: accepter?.image || undefined,
        };

        const acceptedMsg: WSMessage<FriendRequestResponsePayload> = {
          type: 'friend.request.accepted',
          payload: {
            requestId: payload.requestId,
            status: 'accepted',
            friend: accepterInfo,
          },
        };
        wsManager.sendToSession(senderSession, acceptedMsg);
      }

      friendLogger.requestAccepted(payload.userId);
    } else {
      // Reject friend request
      await db
        .delete(quizFriendships)
        .where(eq(quizFriendships.id, payload.requestId));

      // Send response
      const responseMsg: WSMessage<FriendRequestResponsePayload> = {
        type: 'friend.request.response',
        payload: {
          requestId: payload.requestId,
          status: 'rejected',
        },
      };
      wsManager.sendToSession(sessionId, responseMsg);

      friendLogger.requestRejected(payload.userId);
    }
  } catch (error) {
    friendLogger.error('responding to friend request', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal memproses permintaan',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friend request list
 */
export async function handleFriendRequestList(
  sessionId: string,
  payload: FriendRequestListPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();

    let requests;
    if (payload.type === 'incoming') {
      requests = await db
        .select()
        .from(quizFriendships)
        .where(
          and(
            eq(quizFriendships.friendId, payload.userId),
            eq(quizFriendships.status, 'pending'),
          ),
        );
    } else {
      requests = await db
        .select()
        .from(quizFriendships)
        .where(
          and(
            eq(quizFriendships.userId, payload.userId),
            eq(quizFriendships.status, 'pending'),
          ),
        );
    }

    // Get user info for each request
    const requestsWithInfo = [];
    for (const req of requests) {
      const targetId = payload.type === 'incoming' ? req.userId : req.friendId;
      const [user] = await db
        .select()
        .from(users)
        .where(eq(users.id, targetId))
        .limit(1);

      const [stats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, targetId))
        .limit(1);

      requestsWithInfo.push({
        requestId: req.id,
        user: {
          userId: targetId,
          username: user?.name || 'Unknown',
          points: stats?.points || 0,
          avatarUrl: user?.image || undefined,
        },
        timestamp: new Date(req.createdAt).getTime(),
      });
    }

    const listMsg: WSMessage<FriendRequestListDataPayload> = {
      type: 'friend.request.list',
      payload: {
        requests: requestsWithInfo,
        totalCount: requestsWithInfo.length,
      },
    };
    wsManager.sendToSession(sessionId, listMsg);
  } catch (error) {
    console.error('[Friends] Error getting friend request list:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengambil daftar permintaan',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle match invite send
 */
export async function handleMatchInviteSend(
  sessionId: string,
  payload: MatchInviteSendPayload,
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
              eq(quizFriendships.userId, payload.senderId),
              eq(quizFriendships.friendId, payload.receiverId),
            ),
            and(
              eq(quizFriendships.userId, payload.receiverId),
              eq(quizFriendships.friendId, payload.senderId),
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
          message: 'Hanya bisa mengundang teman',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Check if receiver is online
    const receiverSession = wsManager.findSessionByUserId(payload.receiverId);
    if (!receiverSession) {
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

    // Check if receiver is not in game
    const receiverConn = wsManager.getConnection(receiverSession);
    if (receiverConn?.currentMatchId) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'USER_IN_GAME',
          message: 'Teman sedang dalam pertandingan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Get sender info
    const [sender] = await db
      .select()
      .from(users)
      .where(eq(users.id, payload.senderId))
      .limit(1);

    const [senderStats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, payload.senderId))
      .limit(1);

    // Create invite
    const inviteId = `inv_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;
    const expiresAt = Date.now() + 60000; // 60 seconds

    pendingMatchInvites.set(inviteId, {
      inviteId,
      senderId: payload.senderId,
      receiverId: payload.receiverId,
      gameSettings: payload.gameSettings,
      message: payload.message,
      expiresAt,
    });

    // Set timeout to expire invite
    setTimeout(() => {
      const invite = pendingMatchInvites.get(inviteId);
      if (invite) {
        pendingMatchInvites.delete(inviteId);
        // Notify sender that invite expired
        const senderSession = wsManager.findSessionByUserId(invite.senderId);
        if (senderSession) {
          const expiredMsg: WSMessage = {
            type: 'match.invite.expired',
            payload: { inviteId },
          };
          wsManager.sendToSession(senderSession, expiredMsg);
        }
      }
    }, 60000);

    // Send invite to receiver
    const inviteMsg: WSMessage<MatchInviteReceivedPayload> = {
      type: 'match.invite.received',
      payload: {
        inviteId,
        sender: {
          userId: payload.senderId,
          username: sender?.name || 'Unknown',
          points: senderStats?.points || 0,
          wins: senderStats?.wins || 0,
          avatarUrl: sender?.image || undefined,
        },
        gameSettings: payload.gameSettings,
        message: payload.message,
        expiresIn: 60000,
      },
    };
    wsManager.sendToSession(receiverSession, inviteMsg);

    // Send confirmation to sender
    const sentMsg: WSMessage = {
      type: 'match.invite.sent',
      payload: {
        inviteId,
        receiverId: payload.receiverId,
        expiresIn: 60000,
      },
    };
    wsManager.sendToSession(sessionId, sentMsg);

    friendLogger.inviteSent(payload.senderId, payload.receiverId);
  } catch (error) {
    friendLogger.error('sending match invite', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengirim undangan',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle match invite respond
 */
export async function handleMatchInviteRespond(
  sessionId: string,
  payload: MatchInviteRespondPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const invite = pendingMatchInvites.get(payload.inviteId);
    if (!invite) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'INVITE_NOT_FOUND',
          message: 'Undangan tidak ditemukan atau sudah kadaluarsa',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    // Verify receiver
    if (invite.receiverId !== payload.userId) {
      const errorMsg: WSMessage = {
        type: 'error',
        payload: {
          code: 'UNAUTHORIZED',
          message: 'Bukan penerima undangan',
        },
      };
      wsManager.sendToSession(sessionId, errorMsg);
      return;
    }

    pendingMatchInvites.delete(payload.inviteId);

    const senderSession = wsManager.findSessionByUserId(invite.senderId);

    if (!payload.accept) {
      // Reject invite
      const rejectMsg: WSMessage<MatchInviteRejectedPayload> = {
        type: 'match.invite.rejected',
        payload: {
          inviteId: payload.inviteId,
          rejectedBy: payload.userId,
        },
      };
      wsManager.sendToSession(sessionId, rejectMsg);

      if (senderSession) {
        wsManager.sendToSession(senderSession, rejectMsg);
      }

      console.log(
        `[Friends] Match invite rejected: ${invite.senderId} -> ${invite.receiverId}`,
      );
      return;
    }

    // Accept invite - create match
    const db = getDb();
    const matchId = `match_${Date.now()}_${Math.random().toString(36).substring(2, 15)}`;

    // Get opponent info for both players
    const [sender] = await db
      .select()
      .from(users)
      .where(eq(users.id, invite.senderId))
      .limit(1);

    const [receiver] = await db
      .select()
      .from(users)
      .where(eq(users.id, invite.receiverId))
      .limit(1);

    const [senderStats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, invite.senderId))
      .limit(1);

    const [receiverStats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, invite.receiverId))
      .limit(1);

    // Create match in database
    await db.insert(quizMatches).values({
      id: matchId,
      player1Id: invite.senderId,
      player2Id: invite.receiverId,
      gameMode: 'friend',
      difficulty: invite.gameSettings.difficulty,
      category: invite.gameSettings.category,
      status: 'waiting',
      player1Score: 0,
      player2Score: 0,
      player1Health: 100,
      player2Health: 100,
      currentQuestionIndex: 0,
      totalQuestions: invite.gameSettings.totalQuestions,
      timePerQuestion: invite.gameSettings.timePerQuestion,
    });

    // Create match state
    const senderConn = senderSession
      ? wsManager.getConnection(senderSession)
      : null;
    const receiverConn = wsManager.getConnection(sessionId);

    if (senderConn && receiverConn) {
      const matchState: MatchState = {
        matchId,
        player1Id: invite.senderId,
        player2Id: invite.receiverId,
        player1: senderConn,
        player2: receiverConn,
        gameState: {
          totalQuestions: invite.gameSettings.totalQuestions,
          currentQuestionIndex: 0,
          timePerQuestion: invite.gameSettings.timePerQuestion,
          playerHealth: 100,
          opponentHealth: 100,
        },
        questions: [],
        currentQuestionStartTime: 0,
        status: 'waiting',
      };

      wsManager.createMatch(matchId, matchState);

      // Update statuses
      wsManager.updateUserStatus(invite.senderId, 'in_game');
      wsManager.updateUserStatus(invite.receiverId, 'in_game');
    }

    // Send accepted to receiver
    const acceptedMsgReceiver: WSMessage<MatchInviteAcceptedPayload> = {
      type: 'match.invite.accepted',
      payload: {
        inviteId: payload.inviteId,
        matchId,
        opponent: {
          userId: invite.senderId,
          username: sender?.name || 'Unknown',
          points: senderStats?.points || 0,
          wins: senderStats?.wins || 0,
          losses: senderStats?.losses || 0,
          avatarUrl: sender?.image || undefined,
        },
        gameSettings: invite.gameSettings,
        startIn: 5,
      },
    };
    wsManager.sendToSession(sessionId, acceptedMsgReceiver);

    // Send accepted to sender
    if (senderSession) {
      const acceptedMsgSender: WSMessage<MatchInviteAcceptedPayload> = {
        type: 'match.invite.accepted',
        payload: {
          inviteId: payload.inviteId,
          matchId,
          opponent: {
            userId: invite.receiverId,
            username: receiver?.name || 'Unknown',
            points: receiverStats?.points || 0,
            wins: receiverStats?.wins || 0,
            losses: receiverStats?.losses || 0,
            avatarUrl: receiver?.image || undefined,
          },
          gameSettings: invite.gameSettings,
          startIn: 5,
        },
      };
      wsManager.sendToSession(senderSession, acceptedMsgSender);
    }

    console.log(
      `[Friends] Match invite accepted: ${invite.senderId} vs ${invite.receiverId}, matchId: ${matchId}`,
    );

    // Start game after 5 seconds
    setTimeout(async () => {
      const match = wsManager.getMatch(matchId);
      if (match && match.status === 'waiting') {
        // Import startGame dynamically to avoid circular dependency
        const { startGame } = await import('./matchmaking');
        startGame(matchId);
      }
    }, 5000);
  } catch (error) {
    console.error('[Friends] Error responding to match invite:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal memproses undangan',
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
        lastSeen: Date.now(),
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
        expiresIn: 60000,
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
