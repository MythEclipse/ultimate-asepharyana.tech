// Leaderboard Handlers

import { wsManager } from '../ws-manager';
import { leaderboardLogger } from '../../../utils/logger';
import type {
  WSMessage,
  LeaderboardGlobalSyncPayload,
  LeaderboardGlobalDataPayload,
  LeaderboardFriendsSyncPayload,
  LeaderboardFriendsDataPayload,
  LeaderboardEntry,
} from '../types';
import {
  getDb,
  quizUserStats,
  quizFriendships,
  users,
  eq,
  and,
  or,
  desc,
} from '@asepharyana/services';

/**
 * Handle global leaderboard sync
 */
export async function handleLeaderboardGlobalSync(
  sessionId: string,
  payload: LeaderboardGlobalSyncPayload
): Promise<void> {
  try {
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const db = getDb();
    const limit = Math.min(payload.limit || 50, 100); // Max 100
    const offset = payload.offset || 0;

    // Get top players ordered by points
    const topPlayers = await db
      .select({
        userId: quizUserStats.userId,
        points: quizUserStats.points,
        wins: quizUserStats.wins,
        losses: quizUserStats.losses,
      })
      .from(quizUserStats)
      .orderBy(desc(quizUserStats.points))
      .limit(limit)
      .offset(offset);

    // Get user info for each player
    const leaderboard: LeaderboardEntry[] = [];
    for (let i = 0; i < topPlayers.length; i++) {
      const player = topPlayers[i];
      const [user] = await db
        .select()
        .from(users)
        .where(eq(users.id, player.userId))
        .limit(1);

      if (!user) continue;

      const totalGames = player.wins + player.losses;
      const winRate = totalGames > 0 ? (player.wins / totalGames) * 100 : 0;

      // Calculate level (simple formula: 1 level per 100 points)
      const level = Math.floor(player.points / 100) + 1;

      leaderboard.push({
        rank: offset + i + 1,
        userId: player.userId,
        username: user.name || 'Unknown',
        points: player.points,
        wins: player.wins,
        losses: player.losses,
        winRate: Math.round(winRate * 100) / 100,
        avatarUrl: user.image || undefined,
        level,
      });
    }

    // Find current user rank
    let userRank: { rank: number; points: number; percentile: number } = {
      rank: 0,
      points: 0,
      percentile: 0,
    };

    if (connection.userId) {
      const [userStats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, connection.userId))
        .limit(1);

      if (userStats) {
        // Count total users
        const allStats = await db.select().from(quizUserStats);
        const totalUsers = allStats.length;

        // Count users with higher points
        const rank = allStats.filter((s) => s.points > userStats.points).length + 1;
        const percentile = totalUsers > 0 ? ((totalUsers - rank + 1) / totalUsers) * 100 : 0;

        userRank = {
          rank,
          points: userStats.points,
          percentile: Math.round(percentile * 100) / 100,
        };
      }
    }

    // Send leaderboard data
    const leaderboardMsg: WSMessage<LeaderboardGlobalDataPayload> = {
      type: 'leaderboard.global.data',
      payload: {
        leaderboard,
        userRank,
        totalPlayers: leaderboard.length,
        updatedAt: Date.now(),
      },
    };

    wsManager.sendToSession(sessionId, leaderboardMsg);
    leaderboardLogger.fetched('global', leaderboard.length);
  } catch (error) {
    console.error('[Leaderboard] Error getting global leaderboard:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengambil leaderboard',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}

/**
 * Handle friends leaderboard sync
 */
export async function handleLeaderboardFriendsSync(
  sessionId: string,
  payload: LeaderboardFriendsSyncPayload
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

    // Get friend IDs + self
    const friendIds = friendships.map((f) =>
      f.userId === payload.userId ? f.friendId : f.userId
    );
    friendIds.push(payload.userId); // Include self

    // Get stats for all friends
    const friendsStats = await db
      .select()
      .from(quizUserStats)
      .where(
        or(...friendIds.map((id) => eq(quizUserStats.userId, id)))
      );

    // Sort by points
    friendsStats.sort((a, b) => b.points - a.points);

    // Build leaderboard
    const leaderboard: LeaderboardEntry[] = [];
    for (let i = 0; i < friendsStats.length; i++) {
      const stats = friendsStats[i];
      const [user] = await db
        .select()
        .from(users)
        .where(eq(users.id, stats.userId))
        .limit(1);

      if (!user) continue;

      const totalGames = stats.wins + stats.losses;
      const winRate = totalGames > 0 ? (stats.wins / totalGames) * 100 : 0;

      // Calculate level
      const level = Math.floor(stats.points / 100) + 1;

      leaderboard.push({
        rank: i + 1,
        userId: stats.userId,
        username: user.name || 'Unknown',
        points: stats.points,
        wins: stats.wins,
        losses: stats.losses,
        winRate: Math.round(winRate * 100) / 100,
        avatarUrl: user.image || undefined,
        level,
      });
    }

    // Find current user rank
    const userEntry = leaderboard.find((e) => e.userId === payload.userId);
    const userRank = userEntry
      ? { rank: userEntry.rank, points: userEntry.points }
      : { rank: 0, points: 0 };

    // Send friends leaderboard data
    const leaderboardMsg: WSMessage<LeaderboardFriendsDataPayload> = {
      type: 'leaderboard.friends.data',
      payload: {
        leaderboard,
        userRank,
        totalFriends: leaderboard.length - 1, // Exclude self
        updatedAt: Date.now(),
      },
    };

    wsManager.sendToSession(sessionId, leaderboardMsg);
    console.log(
      `[Leaderboard] Sent friends leaderboard to ${payload.userId}: ${leaderboard.length} entries`
    );
  } catch (error) {
    console.error('[Leaderboard] Error getting friends leaderboard:', error);
    const errorMsg: WSMessage = {
      type: 'error',
      payload: {
        code: 'INTERNAL_ERROR',
        message: 'Gagal mengambil leaderboard teman',
      },
    };
    wsManager.sendToSession(sessionId, errorMsg);
  }
}
