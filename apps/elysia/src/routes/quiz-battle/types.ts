import type { ServerWebSocket } from 'bun';

// Types for Quiz Battle WebSocket messages

export type UserStatus = 'online' | 'offline' | 'in_game' | 'away';
export type GameMode = 'ranked' | 'casual' | 'friend';
export type Difficulty = 'easy' | 'medium' | 'hard';
export type MatchStatus = 'waiting' | 'playing' | 'finished' | 'cancelled';
export type LobbyStatus = 'waiting' | 'starting' | 'playing' | 'finished';
export type FriendshipStatus = 'pending' | 'accepted' | 'rejected';

// Base WebSocket Message structure
export interface WSMessage<T = unknown> {
  type: string;
  payload: T;
}

// ===== AUTH & CONNECTION =====
export interface AuthConnectPayload {
  token: string;
  userId: string;
  username: string;
  deviceId: string;
}

export interface AuthConnectedPayload {
  userId: string;
  sessionId: string;
  serverTime: number;
  status: UserStatus;
}

export interface AuthErrorPayload {
  code: string;
  message: string;
}

export interface UserStatusUpdatePayload {
  userId: string;
  status: UserStatus;
}

export interface UserStatusChangedPayload {
  userId: string;
  username: string;
  status: UserStatus;
  timestamp: number;
}

export interface ConnectionPingPayload {
  timestamp: number;
}

export interface ConnectionPongPayload {
  timestamp: number;
  latency: number;
}

// ===== MATCHMAKING =====
export interface MatchmakingFindPayload {
  userId: string;
  gameMode: GameMode;
  difficulty: Difficulty;
  category: string;
}

export interface MatchmakingSearchingPayload {
  estimatedWaitTime: number;
  playersInQueue: number;
}

export interface OpponentInfo {
  userId: string;
  username: string;
  points: number;
  wins: number;
  losses: number;
  avatarUrl?: string;
}

export interface GameSettings {
  totalQuestions: number;
  timePerQuestion: number;
  difficulty: Difficulty;
  category: string;
}

export interface MatchmakingFoundPayload {
  matchId: string;
  opponent: OpponentInfo;
  gameSettings: GameSettings;
  startIn: number;
}

export interface MatchmakingCancelPayload {
  userId: string;
}

export interface MatchmakingCancelledPayload {
  reason: string;
}

// ===== LOBBY =====
export interface LobbyCreatePayload {
  hostId: string;
  hostUsername: string;
  isPrivate: boolean;
  maxPlayers: number;
  gameSettings: GameSettings;
}

export interface LobbyCreatedPayload {
  lobbyId: string;
  lobbyCode: string;
  hostId: string;
  createdAt: number;
}

export interface LobbyJoinPayload {
  userId: string;
  username: string;
  lobbyCode: string;
}

export interface LobbyPlayer {
  userId: string;
  username: string;
  isHost: boolean;
  isReady: boolean;
  points?: number;
  avatarUrl?: string;
}

export interface LobbyPlayerJoinedPayload {
  lobbyId: string;
  player: LobbyPlayer;
  players: LobbyPlayer[];
}

export interface LobbyReadyPayload {
  lobbyId: string;
  userId: string;
  isReady: boolean;
}

export interface LobbyPlayerReadyPayload {
  userId: string;
  isReady: boolean;
  allReady: boolean;
}

export interface LobbyStartPayload {
  lobbyId: string;
  hostId: string;
}

export interface LobbyGameStartingPayload {
  matchId: string;
  countdown: number;
}

export interface LobbyLeavePayload {
  lobbyId: string;
  userId: string;
}

export interface LobbyPlayerLeftPayload {
  userId: string;
  username: string;
  players: LobbyPlayer[];
  newHostId?: string;
}

export interface LobbyKickPayload {
  lobbyId: string;
  hostId: string;
  targetUserId: string;
}

export interface LobbyPlayerKickedPayload {
  kickedUserId: string;
  kickedUsername: string;
  players: LobbyPlayer[];
}

export interface LobbyListSyncPayload {
  userId: string;
}

export interface LobbyInfo {
  lobbyId: string;
  lobbyCode: string;
  hostUsername: string;
  currentPlayers: number;
  maxPlayers: number;
  isPrivate: boolean;
  gameSettings: GameSettings;
}

export interface LobbyListDataPayload {
  lobbies: LobbyInfo[];
  totalLobbies: number;
}

// ===== GAME =====
export interface GamePlayer {
  userId: string;
  username: string;
  position: 'left' | 'right';
}

export interface GameState {
  totalQuestions: number;
  currentQuestionIndex: number;
  timePerQuestion: number;
  playerHealth: number;
  opponentHealth: number;
}

export interface GameStartedPayload {
  matchId: string;
  gameState: GameState;
  players: GamePlayer[];
  serverTime: number;
}

export interface QuestionData {
  id: string;
  text: string;
  answers: string[];
  category: string;
  difficulty: Difficulty;
}

export interface GameQuestionNewPayload {
  matchId: string;
  questionIndex: number;
  question: QuestionData;
  timeLimit: number;
  startTime: number;
}

export interface GameAnswerSubmitPayload {
  matchId: string;
  userId: string;
  questionId: string;
  questionIndex: number;
  answerIndex: number;
  answerTime: number;
  timestamp: number;
}

export interface GameAnswerReceivedPayload {
  questionIndex: number;
  isCorrect: boolean;
  correctAnswerIndex: number;
  points: number;
  answerTime: number; // In seconds
  playerHealth: number; // Updated health after this answer
}

export interface AnimationData {
  type: 'attack' | 'hurt' | 'defend' | 'miss';
  target: 'player' | 'opponent';
  damage: number;
}

export interface GameBattleUpdatePayload {
  matchId: string;
  questionIndex: number;
  event: 'player_answered' | 'player_attacked' | 'player_hurt' | 'timeout';
  actor: {
    userId: string;
    action: 'attack' | 'hurt' | 'miss';
  };
  gameState: GameState;
  animation: AnimationData;
}

export interface GameOpponentAnsweredPayload {
  opponentId: string;
  questionIndex: number;
  answerTime: number;
  isCorrect: boolean;
  animation: 'hurt' | 'attack';
}

export interface GameQuestionTimeoutPayload {
  matchId: string;
  questionIndex: number;
  correctAnswerIndex: number;
  players: Array<{
    userId: string;
    answered: boolean;
    isCorrect: boolean;
    tookDamage?: number;
  }>;
}

export interface GameQuestionNextPayload {
  matchId: string;
  questionIndex: number;
  delay: number;
}

export interface PlayerGameResult {
  userId: string;
  username: string;
  finalHealth: number;
  finalScore: number;
  correctAnswers: number;
  totalAnswers: number;
  averageTime: number;
}

export interface GameRewards {
  points: number;
  experience: number;
  coins: number;
}

export interface GameOverPayload {
  matchId: string;
  reason: 'health_depleted' | 'all_questions_answered' | 'player_disconnected';
  winner: PlayerGameResult;
  loser: PlayerGameResult;
  rewards: {
    winner: GameRewards;
    loser: GameRewards;
  };
  gameHistory: {
    historyId: string;
    playedAt: number;
    duration: number;
  };
}

export interface GamePlayerDisconnectedPayload {
  matchId: string;
  disconnectedPlayer: {
    userId: string;
    username: string;
  };
  waitTime: number;
  autoWin: boolean;
}

export interface GamePlayerReconnectedPayload {
  matchId: string;
  reconnectedPlayer: {
    userId: string;
    username: string;
  };
  gameState: GameState;
  resumeIn: number;
}

// ===== FRIEND SYSTEM =====
export interface FriendRequestSendPayload {
  senderId: string;
  senderUsername: string;
  targetUsername: string;
  message?: string;
}

export interface FriendRequestReceivedPayload {
  requestId: string;
  sender: {
    userId: string;
    username: string;
    points: number;
    wins: number;
    avatarUrl?: string;
  };
  message?: string;
  timestamp: number;
}

export interface FriendRequestAcceptPayload {
  requestId: string;
  userId: string;
}

export interface FriendRequestAcceptedPayload {
  friendship: {
    friendshipId: string;
    user1: { userId: string; username: string };
    user2: { userId: string; username: string };
    createdAt: number;
  };
}

export interface FriendRequestRejectPayload {
  requestId: string;
  userId: string;
}

export interface FriendRequestRejectedPayload {
  requestId: string;
  rejectedBy: string;
}

export interface FriendInfo {
  userId: string;
  username: string;
  status: UserStatus;
  points: number;
  wins: number;
  lastSeen: number;
  avatarUrl?: string;
}

export interface FriendListSyncPayload {
  userId: string;
}

export interface FriendListDataPayload {
  friends: FriendInfo[];
  pendingRequests: number;
  totalFriends: number;
}

export interface FriendChallengeSendPayload {
  challengerId: string;
  challengerUsername: string;
  targetFriendId: string;
  gameSettings: GameSettings;
  message?: string;
}

export interface FriendChallengeReceivedPayload {
  challengeId: string;
  challenger: {
    userId: string;
    username: string;
    points: number;
    wins: number;
  };
  gameSettings: GameSettings;
  message?: string;
  expiresIn: number;
}

export interface FriendRemovePayload {
  userId: string;
  friendId: string;
}

export interface FriendRemovedPayload {
  removedFriendId: string;
  removedBy: string;
}

// ===== LEADERBOARD =====
export interface LeaderboardEntry {
  rank: number;
  userId: string;
  username: string;
  points: number;
  wins: number;
  losses: number;
  winRate: number;
  avatarUrl?: string;
  level: number;
}

export interface LeaderboardGlobalSyncPayload {
  limit: number;
  offset: number;
  timeframe: 'all_time' | 'weekly' | 'monthly' | 'daily';
}

export interface LeaderboardGlobalDataPayload {
  leaderboard: LeaderboardEntry[];
  userRank: {
    rank: number;
    points: number;
    percentile: number;
  };
  totalPlayers: number;
  updatedAt: number;
}

export interface LeaderboardUpdatedPayload {
  userId: string;
  oldRank: number;
  newRank: number;
  pointsGained: number;
  newPoints: number;
  rankChange: number;
}

export interface LeaderboardFriendsSyncPayload {
  userId: string;
  timeframe: 'all_time' | 'weekly' | 'monthly' | 'daily';
}

export interface LeaderboardFriendsDataPayload {
  leaderboard: LeaderboardEntry[];
  userRank: {
    rank: number;
    points: number;
  };
  totalFriends: number;
  updatedAt: number;
}

// ===== NOTIFICATIONS =====
export type NotificationType =
  | 'achievement'
  | 'friend_request'
  | 'challenge'
  | 'system';
export type NotificationPriority = 'low' | 'medium' | 'high';

export interface NotificationReceivedPayload {
  notificationId: string;
  notificationType: NotificationType;
  title: string;
  message: string;
  icon?: string;
  data?: Record<string, unknown>;
  priority: NotificationPriority;
  timestamp: number;
  expiresAt?: number;
}

export interface NotificationReadPayload {
  userId: string;
  notificationId: string;
}

export interface NotificationListSyncPayload {
  userId: string;
  limit?: number;
  offset?: number;
  unreadOnly?: boolean;
}

export interface NotificationListDataPayload {
  notifications: NotificationReceivedPayload[];
  totalCount: number;
  unreadCount: number;
  hasMore: boolean;
}

export interface NotificationMarkReadPayload {
  userId: string;
  notificationIds: string[];
}

export interface NotificationMarkAllReadPayload {
  userId: string;
}

export interface NotificationDeletePayload {
  userId: string;
  notificationId: string;
}

// ===== CHAT =====
export interface ChatGlobalSendPayload {
  userId: string;
  username: string;
  message: string;
  timestamp: number;
}

export interface ChatGlobalMessagePayload {
  messageId: string;
  sender: {
    userId: string;
    username: string;
    level: number;
    avatarUrl?: string;
  };
  message: string;
  timestamp: number;
}

export interface ChatPrivateSendPayload {
  senderId: string;
  receiverId: string;
  message: string;
  timestamp: number;
}

export interface ChatPrivateMessagePayload {
  messageId: string;
  conversationId: string;
  sender: {
    userId: string;
    username: string;
    status: UserStatus;
  };
  message: string;
  timestamp: number;
  isRead: boolean;
}

export interface ChatHistorySyncPayload {
  userId: string;
  targetUserId?: string; // For private chat history
  limit?: number;
  offset?: number;
}

export interface ChatHistoryDataPayload {
  messages: ChatGlobalMessagePayload[] | ChatPrivateMessagePayload[];
  totalMessages: number;
  hasMore: boolean;
}

export interface ChatTypingPayload {
  userId: string;
  targetUserId?: string; // For private chat
  isTyping: boolean;
}

export interface ChatTypingIndicatorPayload {
  userId: string;
  username: string;
  isTyping: boolean;
}

// ===== ACHIEVEMENTS =====
export type AchievementRarity = 'common' | 'rare' | 'epic' | 'legendary';

export interface AchievementData {
  achievementId: string;
  name: string;
  description: string;
  icon?: string;
  rarity: AchievementRarity;
  requirement: Record<string, unknown>;
  rewardPoints: number;
  rewardCoins: number;
  isUnlocked: boolean;
  unlockedAt?: number;
  progress?: number;
  progressMax?: number;
}

export interface AchievementListSyncPayload {
  userId: string;
  unlockedOnly?: boolean;
}

export interface AchievementListDataPayload {
  achievements: AchievementData[];
  totalAchievements: number;
  unlockedCount: number;
}

export interface AchievementClaimPayload {
  userId: string;
  achievementId: string;
}

export interface AchievementUnlockedPayload {
  achievementId: string;
  name: string;
  description: string;
  icon?: string;
  rarity: AchievementRarity;
  rewardPoints: number;
  rewardCoins: number;
  timestamp: number;
}

// ===== DAILY MISSIONS =====
export type MissionType =
  | 'play_games'
  | 'win_games'
  | 'answer_correct'
  | 'win_streak'
  | 'perfect_game';

export interface DailyMissionData {
  missionId: string;
  type: MissionType;
  title: string;
  description: string;
  requirement: number;
  progress: number;
  rewardPoints: number;
  rewardCoins: number;
  isCompleted: boolean;
  isClaimed: boolean;
  expiresAt: number;
}

export interface DailyMissionListSyncPayload {
  userId: string;
}

export interface DailyMissionListDataPayload {
  missions: DailyMissionData[];
  dailyResetAt: number;
}

export interface DailyMissionClaimPayload {
  userId: string;
  missionId: string;
}

export interface DailyMissionCompletedPayload {
  missionId: string;
  title: string;
  rewardPoints: number;
  rewardCoins: number;
  timestamp: number;
}

// ===== RANKED SYSTEM =====
export type RankedTier =
  | 'Bronze'
  | 'Silver'
  | 'Gold'
  | 'Platinum'
  | 'Diamond'
  | 'Master'
  | 'Grandmaster';
export type RankedDivision = 1 | 2 | 3 | 4; // 4 = lowest, 1 = highest

export interface RankedInfo {
  tier: RankedTier;
  division: RankedDivision;
  mmr: number; // Using points as MMR
  rank: number; // Global rank position
  lp: number; // League Points (points within current division)
  wins: number;
  losses: number;
  winRate: number;
  nextTierAt: number; // MMR needed for next tier
}

export interface RankedStatsPayload {
  userId: string;
}

export interface RankedStatsDataPayload {
  rankedInfo: RankedInfo;
}

export interface RankedLeaderboardSyncPayload {
  limit?: number;
  offset?: number;
}

export interface RankedLeaderboardDataPayload {
  players: Array<{
    userId: string;
    username: string;
    tier: RankedTier;
    division: RankedDivision;
    mmr: number;
    rank: number;
    wins: number;
    losses: number;
    winRate: number;
  }>;
  totalPlayers: number;
  hasMore: boolean;
}

export interface MMRChangePayload {
  oldMMR: number;
  newMMR: number;
  change: number;
  oldTier: RankedTier;
  newTier: RankedTier;
  oldDivision: RankedDivision;
  newDivision: RankedDivision;
  promoted: boolean;
  demoted: boolean;
}

// ===== WebSocket Connection =====
export interface WSData {
  sessionId?: string;
  userId?: string;
}

export interface WSConnection {
  userId: string;
  username: string;
  sessionId: string;
  ws: ServerWebSocket<WSData>;
  status: UserStatus;
  currentMatchId?: string;
  currentLobbyId?: string;
  lastPing: number;
}

// ===== Match State Management =====
export interface MatchState {
  matchId: string;
  player1Id: string;
  player2Id: string;
  player1: WSConnection;
  player2: WSConnection;
  gameState: GameState;
  questions: QuestionData[];
  currentQuestionStartTime: number;
  status: MatchStatus;
}

// ===== Lobby State Management =====
export interface LobbyState {
  lobbyId: string;
  lobbyCode: string;
  hostId: string;
  members: Map<string, LobbyPlayer>;
  gameSettings: GameSettings;
  status: LobbyStatus;
}

// ===== Matchmaking Queue =====
export interface MatchmakingQueueEntry {
  userId: string;
  username: string;
  gameMode: GameMode;
  difficulty: Difficulty;
  category: string;
  points: number;
  timestamp: number;
}
