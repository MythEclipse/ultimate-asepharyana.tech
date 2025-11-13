// Achievements System Handlers
import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  AchievementListSyncPayload,
  AchievementListDataPayload,
  AchievementClaimPayload,
  AchievementUnlockedPayload,
  AchievementData,
} from '../types';
import { getDb, quizUserStats, quizAchievements, quizUserAchievements, eq, and } from '@asepharyana/services';
import { sendNotificationToUser } from './notifications';

// Handler: Get achievement list
export async function handleAchievementListSync(sessionId: string, data: WSMessage<AchievementListSyncPayload>) {
  const { userId, unlockedOnly = false } = data.payload;
  const db = getDb();

  const allAchievements = await db.select().from(quizAchievements);
  const userAchievements = await db
    .select()
    .from(quizUserAchievements)
    .where(eq(quizUserAchievements.userId, userId));

  const unlockedIds = new Set(userAchievements.map(ua => ua.achievementId));

  const achievements: AchievementData[] = allAchievements
    .filter(a => !unlockedOnly || unlockedIds.has(a.id))
    .map(a => {
      const unlocked = userAchievements.find(ua => ua.achievementId === a.id);
      return {
        achievementId: a.id,
        name: a.name,
        description: a.description,
        icon: a.icon || undefined,
        rarity: a.rarity as 'common' | 'rare' | 'epic' | 'legendary',
        requirement: a.requirement ? JSON.parse(a.requirement) : {},
        rewardPoints: a.rewardPoints,
        rewardCoins: a.rewardCoins,
        isUnlocked: !!unlocked,
        unlockedAt: unlocked?.unlockedAt.getTime(),
      };
    });

  const response: AchievementListDataPayload = {
    achievements,
    totalAchievements: allAchievements.length,
    unlockedCount: userAchievements.length,
  };

  wsManager.sendToSession(sessionId, 'achievement.list.data', response);
}

// Handler: Claim achievement (already unlocked, just acknowledge)
export async function handleAchievementClaim(sessionId: string, data: WSMessage<AchievementClaimPayload>) {
  const { userId, achievementId } = data.payload;
  const db = getDb();

  const [achievement] = await db
    .select()
    .from(quizAchievements)
    .where(eq(quizAchievements.id, achievementId))
    .limit(1);

  if (!achievement) {
    wsManager.sendToSession(sessionId, 'error', {
      message: 'Achievement tidak ditemukan',
      code: 'ACHIEVEMENT_NOT_FOUND',
    });
    return;
  }

  const [userAchievement] = await db
    .select()
    .from(quizUserAchievements)
    .where(and(
      eq(quizUserAchievements.userId, userId),
      eq(quizUserAchievements.achievementId, achievementId)
    ))
    .limit(1);

  if (!userAchievement) {
    wsManager.sendToSession(sessionId, 'error', {
      message: 'Achievement belum di-unlock',
      code: 'ACHIEVEMENT_NOT_UNLOCKED',
    });
    return;
  }

  wsManager.sendToSession(sessionId, 'achievement.claim.success', {
    achievementId,
    timestamp: new Date().toISOString(),
  });
}

// Achievement Detection: Check after game end
export async function checkAchievementsForUser(userId: string) {
  const db = getDb();
  const [stats] = await db.select().from(quizUserStats).where(eq(quizUserStats.userId, userId)).limit(1);
  if (!stats) return;

  await checkFirstWin(userId, stats);
  await checkWinStreak(userId, stats);
  await checkQuestionsAnswered(userId, stats);
  await checkLevelReached(userId, stats);
  await checkPerfectGames(userId, stats);
}

type UserStats = {
  wins: number;
  losses: number;
  currentStreak: number; // Use currentStreak instead of currentWinStreak
  level: number;
  totalGames: number;
};

async function checkFirstWin(userId: string, stats: UserStats) {
  if (stats.wins !== 1) return;
  await unlockAchievement(userId, 'first_win', 'Kemenangan Pertama', 'Menangkan game pertama kamu', 'common', 10, 50);
}

async function checkWinStreak(userId: string, stats: UserStats) {
  if (stats.currentStreak >= 3) {
    await unlockAchievement(userId, 'win_streak_3', 'Hot Streak!', 'Menang 3 kali berturut-turut', 'rare', 50, 100);
  }
  if (stats.currentStreak >= 5) {
    await unlockAchievement(userId, 'win_streak_5', 'Unstoppable!', 'Menang 5 kali berturut-turut', 'epic', 100, 200);
  }
  if (stats.currentStreak >= 10) {
    await unlockAchievement(userId, 'win_streak_10', 'Legendary!', 'Menang 10 kali berturut-turut', 'legendary', 500, 1000);
  }
}

async function checkQuestionsAnswered(userId: string, stats: UserStats) {
  const totalGames = stats.wins + stats.losses;
  const questionsAnswered = totalGames * 5;

  if (questionsAnswered >= 100) {
    await unlockAchievement(userId, 'questions_100', 'Newbie Scholar', 'Jawab 100 pertanyaan', 'common', 20, 50);
  }
  if (questionsAnswered >= 500) {
    await unlockAchievement(userId, 'questions_500', 'Knowledge Seeker', 'Jawab 500 pertanyaan', 'rare', 100, 200);
  }
  if (questionsAnswered >= 1000) {
    await unlockAchievement(userId, 'questions_1000', 'Quiz Master', 'Jawab 1000 pertanyaan', 'epic', 250, 500);
  }
}

async function checkLevelReached(userId: string, stats: UserStats) {
  if (stats.level >= 10) {
    await unlockAchievement(userId, 'level_10', 'Rising Star', 'Capai Level 10', 'common', 50, 100);
  }
  if (stats.level >= 25) {
    await unlockAchievement(userId, 'level_25', 'Quiz Veteran', 'Capai Level 25', 'rare', 150, 300);
  }
  if (stats.level >= 50) {
    await unlockAchievement(userId, 'level_50', 'Elite Player', 'Capai Level 50', 'epic', 500, 1000);
  }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function checkPerfectGames(_userId: string, _stats: UserStats) {
  // Perfect games tracking would need to be added to schema
  // For now, just placeholder - can be implemented when schema is updated
  return;
}

async function unlockAchievement(
  userId: string,
  achievementId: string,
  name: string,
  description: string,
  rarity: string,
  rewardPoints: number,
  rewardCoins: number
) {
  const db = getDb();

  const [existing] = await db
    .select()
    .from(quizUserAchievements)
    .where(and(
      eq(quizUserAchievements.userId, userId),
      eq(quizUserAchievements.achievementId, achievementId)
    ))
    .limit(1);

  if (existing) return;

  let [achievement] = await db
    .select()
    .from(quizAchievements)
    .where(eq(quizAchievements.id, achievementId))
    .limit(1);

  if (!achievement) {
    await db.insert(quizAchievements).values({
      id: achievementId,
      name,
      description,
      rarity,
      requirement: JSON.stringify({}),
      rewardPoints,
      rewardCoins,
    });

    [achievement] = await db
      .select()
      .from(quizAchievements)
      .where(eq(quizAchievements.id, achievementId))
      .limit(1);
  }

  await db.insert(quizUserAchievements).values({
    id: `ua_${Date.now()}_${userId}`,
    userId,
    achievementId,
    unlockedAt: new Date(),
  });

  // Get current stats
  const [currentStats] = await db
    .select()
    .from(quizUserStats)
    .where(eq(quizUserStats.userId, userId))
    .limit(1);

  if (currentStats) {
    // Update with rewards
    await db
      .update(quizUserStats)
      .set({
        points: currentStats.points + rewardPoints,
        coins: currentStats.coins + rewardCoins,
      })
      .where(eq(quizUserStats.userId, userId));
  }

  const payload: AchievementUnlockedPayload = {
    achievementId,
    name,
    description,
    rarity: rarity as 'common' | 'rare' | 'epic' | 'legendary',
    rewardPoints,
    rewardCoins,
    timestamp: Date.now(),
  };

  const session = wsManager.findSessionByUserId(userId);
  if (session) {
    wsManager.sendToSession(session, 'achievement.unlocked', payload);
  }

  await sendNotificationToUser(
    userId,
    'achievement',
    'Achievement Unlocked!',
    `You unlocked: ${name}`,
    { achievementId, rewardPoints, rewardCoins },
    'high'
  );
}
