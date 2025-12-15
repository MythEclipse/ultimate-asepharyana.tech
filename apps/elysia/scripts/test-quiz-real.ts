// Real user WebSocket test for Quiz Battle
// Uses actual user IDs from database

const WS_URL = 'ws://localhost:4092/api/quiz/battle';

// Real user IDs from database
const USER1_ID = 'user_1765812579468_ojq8xs';
const USER2_ID = 'user_1765812579571_5pquj';

// Generate JWT-like token (matches server's basic validation)
function makeToken(userId: string, username: string): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const payload = btoa(
    JSON.stringify({ userId, username, sub: userId, name: username }),
  );
  const signature = 'test_signature';
  return `${header}.${payload}.${signature}`;
}

interface WSMessage {
  type: string;
  payload: unknown;
}

class QuizBattleClient {
  private ws: WebSocket | null = null;
  private userId: string;
  private username: string;
  private sessionId: string | null = null;

  constructor(userId: string, username: string) {
    this.userId = userId;
    this.username = username;
  }

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(WS_URL);

      this.ws.onopen = () => {
        console.log(`✅ [${this.username}] Connected`);
        this.authenticate();
        resolve();
      };

      this.ws.onmessage = (event) => {
        this.handleMessage(event.data);
      };

      this.ws.onerror = (error) => {
        console.error(`❌ [${this.username}] Error:`, error);
        reject(error);
      };

      this.ws.onclose = () => {
        console.log(`🔌 [${this.username}] Disconnected`);
      };
    });
  }

  private send(message: WSMessage): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
      console.log(`📤 [${this.username}] Sent: ${message.type}`);
    }
  }

  private handleMessage(data: string): void {
    const message: WSMessage = JSON.parse(data);
    console.log(
      `📥 [${this.username}] Received: ${message.type}`,
      JSON.stringify(message.payload).substring(0, 100),
    );

    if (message.type === 'auth.connected') {
      const p = message.payload as { sessionId: string };
      this.sessionId = p.sessionId;
      console.log(
        `🔐 [${this.username}] Authenticated! Session: ${this.sessionId}`,
      );
    }
  }

  private authenticate(): void {
    this.send({
      type: 'auth:connect',
      payload: {
        token: makeToken(this.userId, this.username),
        userId: this.userId,
        username: this.username,
        deviceId: `device_${this.userId}`,
      },
    });
  }

  findMatch(): void {
    this.send({
      type: 'matchmaking.find',
      payload: {
        userId: this.userId,
        gameMode: 'casual',
        difficulty: 'easy',
        category: 'all',
      },
    });
  }

  disconnect(): void {
    if (this.ws) this.ws.close();
  }

  get isAuthenticated(): boolean {
    return this.sessionId !== null;
  }
}

async function runTest() {
  console.log('🎮 Quiz Battle Real User Test\n');
  console.log('Using real user IDs from database:');
  console.log(`  Player 1: ${USER1_ID}`);
  console.log(`  Player 2: ${USER2_ID}\n`);

  const player1 = new QuizBattleClient(USER1_ID, 'Quiz Player 1');
  const player2 = new QuizBattleClient(USER2_ID, 'Quiz Player 2');

  try {
    // Connect both players
    await player1.connect();
    await new Promise((r) => setTimeout(r, 1000));
    await player2.connect();
    await new Promise((r) => setTimeout(r, 2000));

    // Check authentication status
    console.log(`\n📊 Auth Status:`);
    console.log(
      `   Player 1: ${player1.isAuthenticated ? '✅ Authenticated' : '❌ Failed'}`,
    );
    console.log(
      `   Player 2: ${player2.isAuthenticated ? '✅ Authenticated' : '❌ Failed'}`,
    );

    if (player1.isAuthenticated && player2.isAuthenticated) {
      console.log('\n🔍 Starting matchmaking...\n');
      player1.findMatch();
      await new Promise((r) => setTimeout(r, 500));
      player2.findMatch();
      await new Promise((r) => setTimeout(r, 5000));
    }

    // Cleanup
    player1.disconnect();
    player2.disconnect();
    console.log('\n✅ Test completed!');
  } catch (error) {
    console.error('❌ Test failed:', error);
  }
}

runTest();
