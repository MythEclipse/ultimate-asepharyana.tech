// Ranked System Handlers
import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  RankedStatsPayload,
  RankedStatsDataPayload,
  RankedLeaderboardSyncPayload,
  RankedLeaderboardDataPayload,
  MMRChangePayload,
  RankedInfo,
  RankedTier,
  RankedDivision,
} from '../types';
import { getDb, users, quizUserStats, eq, desc } from '@asepharyana/services';

const TIER_THRESHOLDS = {
  Bronze: { min: 0, max: 999 },
  Silver: { min: 1000, max: 1499 },
  Gold: { min: 1500, max: 1999 },
  Platinum: { min: 2000, max: 2499 },
  Diamond: { min: 2500, max: 2999 },
  Master: { min: 3000, max: 3499 },
  Grandmaster: { min: 3500, max: 999999 },
};

const K_FACTOR = 32;

function getTierFromMMR(mmr: number): { tier: RankedTier; division: RankedDivision } {
  if (mmr < 1000) {
    const div = Math.floor((1000 - mmr) / 250) + 1;
    return { tier: 'Bronze', division: Math.min(4, Math.max(1, div)) as RankedDivision };
  } else if (mmr < 1500) {
    const div = Math.floor((1500 - mmr) / 125) + 1;
    return { tier: 'Silver', division: Math.min(4, Math.max(1, div)) as RankedDivision };
  } else if (mmr < 2000) {
    const div = Math.floor((2000 - mmr) / 125) + 1;
    return { tier: 'Gold', division: Math.min(4, Math.max(1, div)) as RankedDivision };
  } else if (mmr < 2500) {
    const div = Math.floor((2500 - mmr) / 125) + 1;
    return { tier: 'Platinum', division: Math.min(4, Math.max(1, div)) as RankedDivision };
  } else if (mmr < 3000) {
    const div = Math.floor((3000 - mmr) / 125) + 1;
    return { tier: 'Diamond', division: Math.min(4, Math.max(1, div)) as RankedDivision };
  } else if (mmr < 3500) {
    return { tier: 'Master', division: 1 };
  } else {
    return { tier: 'Grandmaster', division: 1 };
  }
}

function getNextTierMMR(currentMMR: number): number {
  if (currentMMR < 1000) return 1000;
  if (currentMMR < 1500) return 1500;
  if (currentMMR < 2000) return 2000;
  if (currentMMR < 2500) return 2500;
  if (currentMMR < 3000) return 3000;
  if (currentMMR < 3500) return 3500;
  return 4000;
}

export async function handleRankedStatsSync(ws: any, data: WSMessage<RankedStatsPayload>) {
  const { userId } = data.payload;
  const db = getDb();
  
  const [stats] = await db.select().from(quizUserStats).where(eq(quizUserStats.userId, userId)).limit(1);
  if (!stats) {
    wsManager.sendToSession(ws, 'error', { message: 'Stats not found', code: 'STATS_NOT_FOUND' });
    return;
  }
  
  const mmr = stats.points;
  const { tier, division } = getTierFromMMR(mmr);
  const totalGames = stats.wins + stats.losses;
  const winRate = totalGames > 0 ? (stats.wins / totalGames) * 100 : 0;
  
  const allPlayers = await db.select().from(quizUserStats).orderBy(desc(quizUserStats.points));
  const rank = allPlayers.findIndex(p => p.userId === userId) + 1;
  
  const divisionMin = Math.floor(mmr / 125) * 125;
  const lp = mmr - divisionMin;
  
  const rankedInfo: RankedInfo = {
    tier,
    division,
    mmr,
    rank,
    lp,
    wins: stats.wins,
    losses: stats.losses,
    winRate: Math.round(winRate * 10) / 10,
    nextTierAt: getNextTierMMR(mmr),
  };
  
  const response: RankedStatsDataPayload = { rankedInfo };
  wsManager.sendToSession(ws, 'ranked.stats.data', response);
}

export async function handleRankedLeaderboardSync(ws: any, data: WSMessage<RankedLeaderboardSyncPayload>) {
  const { limit = 100, offset = 0 } = data.payload;
  const db = getDb();
  
  const allPlayers = await db
    .select()
    .from(quizUserStats)
    .orderBy(desc(quizUserStats.points))
    .limit(limit)
    .offset(offset);
  
  const totalPlayers = await db.select().from(quizUserStats);
  
  const players = await Promise.all(
    allPlayers.map(async (stats, index) => {
      const [user] = await db.select().from(users).where(eq(users.id, stats.userId)).limit(1);
      const { tier, division } = getTierFromMMR(stats.points);
      const totalGames = stats.wins + stats.losses;
      const winRate = totalGames > 0 ? (stats.wins / totalGames) * 100 : 0;
      
      return {
        userId: stats.userId,
        username: user?.username || 'Unknown',
        tier,
        division,
        mmr: stats.points,
        rank: offset + index + 1,
        wins: stats.wins,
        losses: stats.losses,
        winRate: Math.round(winRate * 10) / 10,
      };
    })
  );
  
  const response: RankedLeaderboardDataPayload = {
    players,
    totalPlayers: totalPlayers.length,
    hasMore: offset + limit < totalPlayers.length,
  };
  
  wsManager.sendToSession(ws, 'ranked.leaderboard.data', response);
}

export function calculateMMRChange(winnerMMR: number, loserMMR: number, isWinner: boolean): number {
  const expectedWin = 1 / (1 + Math.pow(10, (loserMMR - winnerMMR) / 400));
  const actualScore = isWinner ? 1 : 0;
  const change = Math.round(K_FACTOR * (actualScore - expectedWin));
  return change;
}

export async function updateRankedMMR(userId: string, opponentMMR: number, isWinner: boolean) {
  const db = getDb();
  const [stats] = await db.select().from(quizUserStats).where(eq(quizUserStats.userId, userId)).limit(1);
  if (!stats) return;
  
  const oldMMR = stats.points;
  const mmrChange = calculateMMRChange(oldMMR, opponentMMR, isWinner);
  const newMMR = Math.max(0, oldMMR + mmrChange);
  
  const oldRank = getTierFromMMR(oldMMR);
  const newRank = getTierFromMMR(newMMR);
  
  await db
    .update(quizUserStats)
    .set({ points: newMMR })
    .where(eq(quizUserStats.userId, userId));
  
  const promoted = (oldRank.tier !== newRank.tier && TIER_THRESHOLDS[newRank.tier].min > TIER_THRESHOLDS[oldRank.tier].min) || 
                   (oldRank.tier === newRank.tier && newRank.division < oldRank.division);
  const demoted = (oldRank.tier !== newRank.tier && TIER_THRESHOLDS[newRank.tier].min < TIER_THRESHOLDS[oldRank.tier].min) || 
                  (oldRank.tier === newRank.tier && newRank.division > oldRank.division);
  
  const mmrChangePayload: MMRChangePayload = {
    oldMMR,
    newMMR,
    change: mmrChange,
    oldTier: oldRank.tier,
    newTier: newRank.tier,
    oldDivision: oldRank.division,
    newDivision: newRank.division,
    promoted,
    demoted,
  };
  
  const session = wsManager.findSessionByUserId(userId);
  if (session) {
    wsManager.sendToSession(session, 'ranked.mmr.changed', mmrChangePayload);
  }
  
  return mmrChangePayload;
}
