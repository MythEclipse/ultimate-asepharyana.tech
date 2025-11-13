// WebSocket Manager untuk Quiz Battle
// Mengelola koneksi, state, dan broadcasting messages

import type {
  WSConnection,
  MatchState,
  LobbyState,
  MatchmakingQueueEntry,
  WSMessage,
} from './types';

export class QuizBattleWSManager {
  // Storage untuk semua koneksi aktif
  private connections: Map<string, WSConnection> = new Map();

  // Storage untuk match yang sedang berlangsung
  private activeMatches: Map<string, MatchState> = new Map();

  // Storage untuk lobby yang aktif
  private activeLobbies: Map<string, LobbyState> = new Map();

  // Mapping dari lobbyCode ke lobbyId
  private lobbyCodeMap: Map<string, string> = new Map();

  // Queue untuk matchmaking
  private matchmakingQueue: Map<string, MatchmakingQueueEntry> = new Map();

  // Mapping userId ke sessionId untuk cepat lookup
  private userSessionMap: Map<string, string> = new Map();

  // ===== CONNECTION MANAGEMENT =====

  addConnection(sessionId: string, connection: WSConnection): void {
    this.connections.set(sessionId, connection);
    this.userSessionMap.set(connection.userId, sessionId);
    console.log(`[WS] User ${connection.username} connected (${sessionId})`);
  }

  removeConnection(sessionId: string): void {
    const connection = this.connections.get(sessionId);
    if (connection) {
      this.userSessionMap.delete(connection.userId);
      this.connections.delete(sessionId);

      // Cleanup: remove from matchmaking queue if exists
      this.matchmakingQueue.delete(connection.userId);

      console.log(`[WS] User ${connection.username} disconnected (${sessionId})`);
    }
  }

  getConnection(sessionId: string): WSConnection | undefined {
    return this.connections.get(sessionId);
  }

  getConnectionByUserId(userId: string): WSConnection | undefined {
    const sessionId = this.userSessionMap.get(userId);
    return sessionId ? this.connections.get(sessionId) : undefined;
  }

  findSessionByUserId(userId: string): string | undefined {
    return this.userSessionMap.get(userId);
  }

  getAllConnections(): WSConnection[] {
    return Array.from(this.connections.values());
  }

  getConnectionsCount(): number {
    return this.connections.size;
  }

  getSessionIdByConnection(connection: WSConnection): string | undefined {
    for (const [sessionId, conn] of this.connections.entries()) {
      if (conn === connection) {
        return sessionId;
      }
    }
    return undefined;
  }

  // ===== MATCH MANAGEMENT =====

  createMatch(matchId: string, matchState: MatchState): void {
    this.activeMatches.set(matchId, matchState);

    // Update connections dengan matchId
    const player1 = this.getConnectionByUserId(matchState.player1Id);
    const player2 = this.getConnectionByUserId(matchState.player2Id);

    if (player1) player1.currentMatchId = matchId;
    if (player2) player2.currentMatchId = matchId;

    console.log(`[Match] Created match ${matchId}`);
  }

  getMatch(matchId: string): MatchState | undefined {
    return this.activeMatches.get(matchId);
  }

  updateMatch(matchId: string, matchState: Partial<MatchState>): void {
    const match = this.activeMatches.get(matchId);
    if (match) {
      Object.assign(match, matchState);
    }
  }

  removeMatch(matchId: string): void {
    const match = this.activeMatches.get(matchId);
    if (match) {
      // Clear currentMatchId dari connections
      const player1 = this.getConnectionByUserId(match.player1Id);
      const player2 = this.getConnectionByUserId(match.player2Id);

      if (player1) player1.currentMatchId = undefined;
      if (player2) player2.currentMatchId = undefined;

      this.activeMatches.delete(matchId);
      console.log(`[Match] Removed match ${matchId}`);
    }
  }

  getActiveMatchesCount(): number {
    return this.activeMatches.size;
  }

  // ===== LOBBY MANAGEMENT =====

  createLobby(lobbyId: string, lobbyState: LobbyState): void {
    this.activeLobbies.set(lobbyId, lobbyState);
    this.lobbyCodeMap.set(lobbyState.lobbyCode, lobbyId);

    // Update connection dengan lobbyId
    const host = this.getConnectionByUserId(lobbyState.hostId);
    if (host) host.currentLobbyId = lobbyId;

    console.log(`[Lobby] Created lobby ${lobbyId} with code ${lobbyState.lobbyCode}`);
  }

  getLobby(lobbyId: string): LobbyState | undefined {
    return this.activeLobbies.get(lobbyId);
  }

  getLobbyByCode(lobbyCode: string): LobbyState | undefined {
    const lobbyId = this.lobbyCodeMap.get(lobbyCode);
    return lobbyId ? this.activeLobbies.get(lobbyId) : undefined;
  }

  updateLobby(lobbyId: string, lobbyState: Partial<LobbyState>): void {
    const lobby = this.activeLobbies.get(lobbyId);
    if (lobby) {
      Object.assign(lobby, lobbyState);
    }
  }

  addLobbyMember(lobbyId: string, userId: string): void {
    const connection = this.getConnectionByUserId(userId);
    if (connection) {
      connection.currentLobbyId = lobbyId;
    }
  }

  removeLobbyMember(lobbyId: string, userId: string): void {
    const lobby = this.activeLobbies.get(lobbyId);
    if (lobby) {
      lobby.members.delete(userId);

      const connection = this.getConnectionByUserId(userId);
      if (connection) {
        connection.currentLobbyId = undefined;
      }
    }
  }

  removeLobby(lobbyId: string): void {
    const lobby = this.activeLobbies.get(lobbyId);
    if (lobby) {
      this.lobbyCodeMap.delete(lobby.lobbyCode);

      // Clear currentLobbyId dari semua members
      lobby.members.forEach((_, userId) => {
        const connection = this.getConnectionByUserId(userId);
        if (connection) connection.currentLobbyId = undefined;
      });

      this.activeLobbies.delete(lobbyId);
      console.log(`[Lobby] Removed lobby ${lobbyId}`);
    }
  }

  getActiveLobbiesCount(): number {
    return this.activeLobbies.size;
  }

  // ===== MATCHMAKING QUEUE =====

  addToQueue(entry: MatchmakingQueueEntry): void {
    this.matchmakingQueue.set(entry.userId, entry);
    console.log(`[Queue] Added ${entry.username} to matchmaking queue`);
  }

  removeFromQueue(userId: string): void {
    const entry = this.matchmakingQueue.get(userId);
    if (entry) {
      this.matchmakingQueue.delete(userId);
      console.log(`[Queue] Removed ${entry.username} from matchmaking queue`);
    }
  }

  getQueueEntry(userId: string): MatchmakingQueueEntry | undefined {
    return this.matchmakingQueue.get(userId);
  }

  findMatchInQueue(
    userId: string,
    gameMode: string,
    difficulty: string,
    category: string,
    userMMR?: number
  ): MatchmakingQueueEntry | undefined {
    // For ranked mode, use MMR-based matching with fallback
    // This ensures matches always happen, even with only 2 players of vastly different ranks
    if (gameMode === 'ranked' && userMMR !== undefined) {
      const MMR_RANGE = 200; // Preferred MMR range (±200)
      let bestMatch: MatchmakingQueueEntry | undefined; // Best match within range
      let smallestDiff = Infinity;
      let anyRankedPlayer: MatchmakingQueueEntry | undefined; // Fallback: any ranked player

      for (const [queueUserId, entry] of this.matchmakingQueue.entries()) {
        if (queueUserId === userId) continue;
        if (entry.gameMode !== 'ranked') continue;

        // Keep track of any ranked player as fallback (ensures 2-player matches work)
        if (!anyRankedPlayer) {
          anyRankedPlayer = entry;
        }

        const mmrDiff = Math.abs(entry.points - userMMR);

        // Priority 1: Matches within ±200 MMR range (closest first)
        if (mmrDiff <= MMR_RANGE && mmrDiff < smallestDiff) {
          smallestDiff = mmrDiff;
          bestMatch = entry;
        } else if (!bestMatch && mmrDiff < smallestDiff) {
          // Priority 2: If no match in range, track closest overall
          smallestDiff = mmrDiff;
          anyRankedPlayer = entry;
        }
      }

      // Return: Best in range > Closest overall > Any ranked player
      // This ensures Bronze can match Grandmaster if queue is empty (ELO handles fairness)
      return bestMatch || anyRankedPlayer;
    }

    // For casual/friend mode, use simple matching
    for (const [queueUserId, entry] of this.matchmakingQueue.entries()) {
      // Skip self
      if (queueUserId === userId) continue;

      // Match criteria: same gameMode, difficulty, category
      if (
        entry.gameMode === gameMode &&
        entry.difficulty === difficulty &&
        (entry.category === category || category === 'all' || entry.category === 'all')
      ) {
        return entry;
      }
    }
    return undefined;
  }

  getQueueSize(): number {
    return this.matchmakingQueue.size;
  }

  // ===== MESSAGE BROADCASTING =====

  sendToUser(userId: string, message: WSMessage): void {
    const connection = this.getConnectionByUserId(userId);
    if (connection && connection.ws) {
      try {
        connection.ws.send(JSON.stringify(message));
      } catch (error) {
        console.error(`[WS] Error sending message to user ${userId}:`, error);
      }
    }
  }

  sendToSession(sessionId: string, typeOrMessage: string | WSMessage, payload?: unknown): void {
    const connection = this.connections.get(sessionId);
    if (connection && connection.ws) {
      try {
        // Support both formats: sendToSession(id, message) and sendToSession(id, type, payload)
        const message: WSMessage = typeof typeOrMessage === 'string'
          ? { type: typeOrMessage, payload: payload || {} }
          : typeOrMessage;

        connection.ws.send(JSON.stringify(message));
      } catch (error) {
        console.error(`[WS] Error sending message to session ${sessionId}:`, error);
      }
    }
  }

  broadcastToMatch(matchId: string, message: WSMessage): void {
    const match = this.activeMatches.get(matchId);
    if (match) {
      this.sendToUser(match.player1Id, message);
      this.sendToUser(match.player2Id, message);
    }
  }

  broadcastToLobby(lobbyId: string, message: WSMessage): void {
    const lobby = this.activeLobbies.get(lobbyId);
    if (lobby) {
      lobby.members.forEach((_, userId) => {
        this.sendToUser(userId, message);
      });
    }
  }

  broadcastToFriends(userId: string, message: WSMessage, friendIds: string[]): void {
    friendIds.forEach(friendId => {
      const connection = this.getConnectionByUserId(friendId);
      if (connection && connection.status === 'online') {
        this.sendToUser(friendId, message);
      }
    });
  }

  broadcastToAll(message: WSMessage): void {
    this.connections.forEach((connection) => {
      if (connection.ws) {
        try {
          connection.ws.send(JSON.stringify(message));
        } catch (error) {
          console.error(`[WS] Error broadcasting to ${connection.userId}:`, error);
        }
      }
    });
  }

  // ===== UTILITY FUNCTIONS =====

  updateUserStatus(userId: string, status: 'online' | 'offline' | 'in_game' | 'away'): void {
    const connection = this.getConnectionByUserId(userId);
    if (connection) {
      connection.status = status;
    }
  }

  getUserStatus(userId: string): 'online' | 'offline' | 'in_game' | 'away' {
    const connection = this.getConnectionByUserId(userId);
    return connection ? connection.status : 'offline';
  }

  isUserInMatch(userId: string): boolean {
    const connection = this.getConnectionByUserId(userId);
    return connection ? !!connection.currentMatchId : false;
  }

  isUserInLobby(userId: string): boolean {
    const connection = this.getConnectionByUserId(userId);
    return connection ? !!connection.currentLobbyId : false;
  }

  // ===== CLEANUP & MAINTENANCE =====

  cleanupExpiredLobbies(): void {
    // const now = Date.now();
    const expiredLobbies: string[] = [];

    // Find expired lobbies (created > 30 minutes ago and status still 'waiting')
    this.activeLobbies.forEach((lobby, lobbyId) => {
      // TODO: Implement lobby expiration logic when needed
      // For now, just check if lobby is empty
      if (!lobby.hostId) {
        expiredLobbies.push(lobbyId);
      }
    });

    expiredLobbies.forEach(lobbyId => this.removeLobby(lobbyId));
  }

  cleanupDisconnectedUsers(): void {
    const now = Date.now();
    const disconnectTimeout = 60000; // 60 seconds

    this.connections.forEach((connection, sessionId) => {
      // Check last ping time
      if (now - connection.lastPing > disconnectTimeout) {
        console.log(`[WS] Cleaning up disconnected user ${connection.username}`);
        this.removeConnection(sessionId);
      }
    });
  }

  // ===== DEBUGGING & STATS =====

  getStats() {
    return {
      activeConnections: this.getConnectionsCount(),
      activeMatches: this.getActiveMatchesCount(),
      activeLobbies: this.getActiveLobbiesCount(),
      queueSize: this.getQueueSize(),
    };
  }

  printStats(): void {
    const stats = this.getStats();
    console.log('[WS Stats]', stats);
  }
}

// Singleton instance
export const wsManager = new QuizBattleWSManager();
