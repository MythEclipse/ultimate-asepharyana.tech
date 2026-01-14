/**
 * Centralized Logger Utility
 * Provides consistent logging format across all API and WebSocket handlers
 */

type LogLevel = 'INFO' | 'WARN' | 'ERROR' | 'DEBUG';

interface LogContext {
  userId?: string;
  sessionId?: string;
  requestId?: string;
  ip?: string;
  method?: string;
  path?: string;
  [key: string]: unknown;
}

const getTimestamp = (): string => {
  return new Date().toISOString();
};

const formatContext = (context?: LogContext): string => {
  if (!context || Object.keys(context).length === 0) return '';
  const parts: string[] = [];
  if (context.userId) parts.push(`user=${context.userId}`);
  if (context.sessionId) parts.push(`session=${context.sessionId}`);
  if (context.requestId) parts.push(`req=${context.requestId}`);
  if (context.ip) parts.push(`ip=${context.ip}`);
  if (context.method && context.path) parts.push(`${context.method} ${context.path}`);
  return parts.length > 0 ? ` [${parts.join(', ')}]` : '';
};

const log = (level: LogLevel, prefix: string, message: string, context?: LogContext, data?: unknown): void => {
  const timestamp = getTimestamp();
  const contextStr = formatContext(context);
  const logFn = level === 'ERROR' ? console.error : level === 'WARN' ? console.warn : console.log;
  
  if (data !== undefined) {
    logFn(`[${timestamp}] [${level}] [${prefix}]${contextStr} ${message}`, data);
  } else {
    logFn(`[${timestamp}] [${level}] [${prefix}]${contextStr} ${message}`);
  }
};

// Auth Logger
export const authLogger = {
  loginAttempt: (email: string, ip?: string) => 
    log('INFO', 'AUTH', `Login attempt for ${email}`, { ip }),
  loginSuccess: (userId: string, email: string, ip?: string) => 
    log('INFO', 'AUTH', `Login successful for ${email}`, { userId, ip }),
  loginFailed: (email: string, reason: string, ip?: string) => 
    log('WARN', 'AUTH', `Login failed for ${email}: ${reason}`, { ip }),
  registerAttempt: (email: string, ip?: string) => 
    log('INFO', 'AUTH', `Registration attempt for ${email}`, { ip }),
  registerSuccess: (userId: string, email: string) => 
    log('INFO', 'AUTH', `Registration successful for ${email}`, { userId }),
  registerFailed: (email: string, reason: string) => 
    log('WARN', 'AUTH', `Registration failed for ${email}: ${reason}`),
  tokenVerified: (userId: string) => 
    log('DEBUG', 'AUTH', `Token verified`, { userId }),
  tokenInvalid: (reason: string) => 
    log('WARN', 'AUTH', `Token invalid: ${reason}`),
  logout: (userId: string) => 
    log('INFO', 'AUTH', `User logged out`, { userId }),
  passwordReset: (email: string) => 
    log('INFO', 'AUTH', `Password reset requested for ${email}`),
  emailVerified: (userId: string, email: string) => 
    log('INFO', 'AUTH', `Email verified for ${email}`, { userId }),
};

// API Logger
export const apiLogger = {
  request: (method: string, path: string, userId?: string, ip?: string) => 
    log('INFO', 'API', `Request received`, { method, path, userId, ip }),
  response: (method: string, path: string, status: number, durationMs?: number) => 
    log('INFO', 'API', `Response sent: ${status}${durationMs ? ` (${durationMs}ms)` : ''}`, { method, path }),
  error: (method: string, path: string, error: unknown) => 
    log('ERROR', 'API', `Error processing request`, { method, path }, error),
};

// History Logger
export const historyLogger = {
  fetch: (userId: string, count: number) => 
    log('INFO', 'HISTORY', `Fetched ${count} matches for user`, { userId }),
  fetchError: (userId: string, error: unknown) => 
    log('ERROR', 'HISTORY', `Failed to fetch history`, { userId }, error),
};

// Chat Logger
export const chatLogger = {
  roomCreated: (roomId: string, userId: string, name: string) => 
    log('INFO', 'CHAT', `Room created: ${name}`, { userId }),
  roomFetch: (userId: string, count: number) => 
    log('INFO', 'CHAT', `Fetched ${count} rooms`, { userId }),
  messageSent: (roomId: string, userId: string) => 
    log('DEBUG', 'CHAT', `Message sent to room ${roomId}`, { userId }),
  memberJoined: (roomId: string, userId: string) => 
    log('INFO', 'CHAT', `User joined room ${roomId}`, { userId }),
  memberLeft: (roomId: string, userId: string) => 
    log('INFO', 'CHAT', `User left room ${roomId}`, { userId }),
  error: (action: string, error: unknown) => 
    log('ERROR', 'CHAT', `Error: ${action}`, undefined, error),
};

// WebSocket Logger
export const wsLogger = {
  connected: (sessionId: string, userId?: string, username?: string) => 
    log('INFO', 'WS', `Client connected${username ? `: ${username}` : ''}`, { sessionId, userId }),
  disconnected: (sessionId: string, userId?: string, reason?: string) => 
    log('INFO', 'WS', `Client disconnected${reason ? `: ${reason}` : ''}`, { sessionId, userId }),
  messageReceived: (sessionId: string, type: string, userId?: string) => 
    log('DEBUG', 'WS', `Message received: ${type}`, { sessionId, userId }),
  messageSent: (sessionId: string, type: string) => 
    log('DEBUG', 'WS', `Message sent: ${type}`, { sessionId }),
  error: (sessionId: string, error: unknown) => 
    log('ERROR', 'WS', `Error`, { sessionId }, error),
  authenticated: (sessionId: string, userId: string, username: string) => 
    log('INFO', 'WS', `User authenticated: ${username}`, { sessionId, userId }),
};

// Friend System Logger
export const friendLogger = {
  requestSent: (fromUserId: string, toUserId: string) => 
    log('INFO', 'FRIEND', `Friend request sent from ${fromUserId} to ${toUserId}`),
  requestAccepted: (userId: string, friendId: string) => 
    log('INFO', 'FRIEND', `Friend request accepted`, { userId }),
  requestRejected: (userId: string, requestId: string) => 
    log('INFO', 'FRIEND', `Friend request rejected`, { userId }),
  removed: (userId: string, friendId: string) => 
    log('INFO', 'FRIEND', `Friend removed: ${friendId}`, { userId }),
  listFetched: (userId: string, count: number) => 
    log('DEBUG', 'FRIEND', `Friend list fetched: ${count} friends`, { userId }),
  inviteSent: (senderId: string, receiverId: string) => 
    log('INFO', 'FRIEND', `Match invite sent from ${senderId} to ${receiverId}`),
  inviteAccepted: (userId: string, inviteId: string) => 
    log('INFO', 'FRIEND', `Match invite accepted`, { userId }),
  inviteRejected: (userId: string, inviteId: string) => 
    log('INFO', 'FRIEND', `Match invite rejected`, { userId }),
  error: (action: string, error: unknown) => 
    log('ERROR', 'FRIEND', `Error: ${action}`, undefined, error),
};

// Match/Game Logger
export const matchLogger = {
  created: (matchId: string, player1: string, player2: string) => 
    log('INFO', 'MATCH', `Match created: ${matchId} (${player1} vs ${player2})`),
  started: (matchId: string) => 
    log('INFO', 'MATCH', `Match started: ${matchId}`),
  ended: (matchId: string, winnerId?: string) => 
    log('INFO', 'MATCH', `Match ended: ${matchId}${winnerId ? `, winner: ${winnerId}` : ''}`),
  playerJoined: (matchId: string, userId: string) => 
    log('INFO', 'MATCH', `Player joined match ${matchId}`, { userId }),
  playerLeft: (matchId: string, userId: string) => 
    log('INFO', 'MATCH', `Player left match ${matchId}`, { userId }),
  answerSubmitted: (matchId: string, userId: string, correct: boolean) => 
    log('DEBUG', 'MATCH', `Answer submitted: ${correct ? 'correct' : 'incorrect'}`, { userId }),
  error: (matchId: string, error: unknown) => 
    log('ERROR', 'MATCH', `Error in match ${matchId}`, undefined, error),
};

// Queue Logger
export const queueLogger = {
  joined: (userId: string, username: string) => 
    log('INFO', 'QUEUE', `User joined matchmaking queue: ${username}`, { userId }),
  left: (userId: string, username: string) => 
    log('INFO', 'QUEUE', `User left matchmaking queue: ${username}`, { userId }),
  matched: (user1: string, user2: string, matchId: string) => 
    log('INFO', 'QUEUE', `Players matched: ${user1} vs ${user2}, match: ${matchId}`),
  timeout: (userId: string) => 
    log('INFO', 'QUEUE', `Queue timeout for user`, { userId }),
};

// Leaderboard Logger
export const leaderboardLogger = {
  fetched: (type: string, count: number) => 
    log('DEBUG', 'LEADERBOARD', `Fetched ${type} leaderboard: ${count} entries`),
  updated: (userId: string, points: number) => 
    log('DEBUG', 'LEADERBOARD', `Updated user points: ${points}`, { userId }),
  error: (action: string, error: unknown) => 
    log('ERROR', 'LEADERBOARD', `Error: ${action}`, undefined, error),
};

// Lobby Logger
export const lobbyLogger = {
  created: (lobbyId: string, hostId: string) => 
    log('INFO', 'LOBBY', `Lobby created: ${lobbyId}`, { userId: hostId }),
  joined: (lobbyId: string, userId: string) => 
    log('INFO', 'LOBBY', `User joined lobby ${lobbyId}`, { userId }),
  left: (lobbyId: string, userId: string) => 
    log('INFO', 'LOBBY', `User left lobby ${lobbyId}`, { userId }),
  started: (lobbyId: string) => 
    log('INFO', 'LOBBY', `Lobby game started: ${lobbyId}`),
  closed: (lobbyId: string) => 
    log('INFO', 'LOBBY', `Lobby closed: ${lobbyId}`),
  error: (lobbyId: string, error: unknown) => 
    log('ERROR', 'LOBBY', `Error in lobby ${lobbyId}`, undefined, error),
};

export default {
  auth: authLogger,
  api: apiLogger,
  history: historyLogger,
  chat: chatLogger,
  ws: wsLogger,
  friend: friendLogger,
  match: matchLogger,
  queue: queueLogger,
  leaderboard: leaderboardLogger,
  lobby: lobbyLogger,
};
