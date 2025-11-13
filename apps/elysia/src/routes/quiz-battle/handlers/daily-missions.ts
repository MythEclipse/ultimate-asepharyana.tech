// Daily Missions System Handlers (In-Memory)
import { wsManager } from '../ws-manager';
import type {
  WSMessage,
  DailyMissionListSyncPayload,
  DailyMissionListDataPayload,
  DailyMissionClaimPayload,
  DailyMissionCompletedPayload,
  DailyMissionData,
  MissionType,
} from '../types';
import { getDb, quizUserStats, eq, sql } from '@asepharyana/services';
import { sendNotificationToUser } from './notifications';

interface UserMissionProgress {
  [missionId: string]: {
    progress: number;
    isCompleted: boolean;
    isClaimed: boolean;
  };
}

const userMissions = new Map<string, UserMissionProgress>();
let dailyResetTime = getNextDailyReset();

function getNextDailyReset(): number {
  const now = new Date();
  const tomorrow = new Date(now);
  tomorrow.setDate(tomorrow.getDate() + 1);
  tomorrow.setHours(0, 0, 0, 0);
  return tomorrow.getTime();
}

function checkDailyReset() {
  if (Date.now() >= dailyResetTime) {
    userMissions.clear();
    dailyResetTime = getNextDailyReset();
    console.log('[DailyMissions] Daily reset executed');
  }
}

const DAILY_MISSIONS = [
  { id: 'play_3', type: 'play_games' as MissionType, title: 'Play 3 Games', desc: 'Complete 3 games today', req: 3, pts: 50, coins: 25 },
  { id: 'win_2', type: 'win_games' as MissionType, title: 'Win 2 Games', desc: 'Win 2 games today', req: 2, pts: 100, coins: 50 },
  { id: 'answer_15', type: 'answer_correct' as MissionType, title: 'Answer 15 Correctly', desc: 'Answer 15 questions correctly', req: 15, pts: 75, coins: 30 },
  { id: 'streak_2', type: 'win_streak' as MissionType, title: 'Win Streak', desc: 'Win 2 games in a row', req: 2, pts: 150, coins: 75 },
  { id: 'perfect_1', type: 'perfect_game' as MissionType, title: 'Perfect Game', desc: 'Complete 1 perfect game', req: 1, pts: 200, coins: 100 },
];

function getUserProgress(userId: string): UserMissionProgress {
  checkDailyReset();
  if (!userMissions.has(userId)) {
    const progress: UserMissionProgress = {};
    DAILY_MISSIONS.forEach(m => {
      progress[m.id] = { progress: 0, isCompleted: false, isClaimed: false };
    });
    userMissions.set(userId, progress);
  }
  return userMissions.get(userId)!;
}

export async function handleDailyMissionListSync(ws: any, data: WSMessage<DailyMissionListSyncPayload>) {
  const { userId } = data.payload;
  const userProgress = getUserProgress(userId);
  
  const missions: DailyMissionData[] = DAILY_MISSIONS.map(m => ({
    missionId: m.id,
    type: m.type,
    title: m.title,
    description: m.desc,
    requirement: m.req,
    progress: userProgress[m.id].progress,
    rewardPoints: m.pts,
    rewardCoins: m.coins,
    isCompleted: userProgress[m.id].isCompleted,
    isClaimed: userProgress[m.id].isClaimed,
    expiresAt: dailyResetTime,
  }));
  
  const response: DailyMissionListDataPayload = {
    missions,
    dailyResetAt: dailyResetTime,
  };
  
  wsManager.sendToSession(ws, 'daily.mission.list.data', response);
}

export async function handleDailyMissionClaim(ws: any, data: WSMessage<DailyMissionClaimPayload>) {
  const { userId, missionId } = data.payload;
  const userProgress = getUserProgress(userId);
  
  const mission = DAILY_MISSIONS.find(m => m.id === missionId);
  if (!mission) {
    wsManager.sendToSession(ws, 'error', { message: 'Mission not found', code: 'MISSION_NOT_FOUND' });
    return;
  }
  
  const progress = userProgress[missionId];
  if (!progress.isCompleted) {
    wsManager.sendToSession(ws, 'error', { message: 'Mission not completed', code: 'MISSION_NOT_COMPLETED' });
    return;
  }
  
  if (progress.isClaimed) {
    wsManager.sendToSession(ws, 'error', { message: 'Mission already claimed', code: 'MISSION_ALREADY_CLAIMED' });
    return;
  }
  
  progress.isClaimed = true;
  
  const db = getDb();
  await db
    .update(quizUserStats)
    .set({
      points: sql`${quizUserStats.points} + ${mission.pts}`,
      coins: sql`${quizUserStats.coins} + ${mission.coins}`,
    })
    .where(eq(quizUserStats.userId, userId));
  
  wsManager.sendToSession(ws, 'daily.mission.claim.success', {
    missionId,
    rewardPoints: mission.pts,
    rewardCoins: mission.coins,
    timestamp: new Date().toISOString(),
  });
}

export async function updateMissionProgress(userId: string, type: MissionType, increment: number = 1) {
  const userProgress = getUserProgress(userId);
  const missions = DAILY_MISSIONS.filter(m => m.type === type);
  
  for (const mission of missions) {
    const progress = userProgress[mission.id];
    if (progress.isCompleted) continue;
    
    progress.progress += increment;
    
    if (progress.progress >= mission.req && !progress.isCompleted) {
      progress.isCompleted = true;
      
      const payload: DailyMissionCompletedPayload = {
        missionId: mission.id,
        title: mission.title,
        rewardPoints: mission.pts,
        rewardCoins: mission.coins,
        timestamp: Date.now(),
      };
      
      const session = wsManager.findSessionByUserId(userId);
      if (session) {
        wsManager.sendToSession(session, 'daily.mission.completed', payload);
      }
      
      await sendNotificationToUser(
        userId,
        'system',
        'Daily Mission Completed!',
        `You completed: ${mission.title}`,
        { missionId: mission.id, rewardPoints: mission.pts, rewardCoins: mission.coins },
        'medium'
      );
    }
  }
}

export async function trackGamePlayed(userId: string) {
  await updateMissionProgress(userId, 'play_games', 1);
}

export async function trackGameWon(userId: string) {
  await updateMissionProgress(userId, 'win_games', 1);
}

export async function trackCorrectAnswers(userId: string, count: number) {
  await updateMissionProgress(userId, 'answer_correct', count);
}

export async function trackWinStreak(userId: string, streakCount: number) {
  const userProgress = getUserProgress(userId);
  const missions = DAILY_MISSIONS.filter(m => m.type === 'win_streak');
  
  for (const mission of missions) {
    const progress = userProgress[mission.id];
    if (progress.isCompleted) continue;
    
    if (streakCount >= mission.req) {
      progress.progress = mission.req;
      progress.isCompleted = true;
      
      const payload: DailyMissionCompletedPayload = {
        missionId: mission.id,
        title: mission.title,
        rewardPoints: mission.pts,
        rewardCoins: mission.coins,
        timestamp: Date.now(),
      };
      
      const session = wsManager.findSessionByUserId(userId);
      if (session) {
        wsManager.sendToSession(session, 'daily.mission.completed', payload);
      }
      
      await sendNotificationToUser(
        userId,
        'system',
        'Daily Mission Completed!',
        `You completed: ${mission.title}`,
        { missionId: mission.id, rewardPoints: mission.pts, rewardCoins: mission.coins },
        'medium'
      );
    }
  }
}

export async function trackPerfectGame(userId: string) {
  await updateMissionProgress(userId, 'perfect_game', 1);
}
