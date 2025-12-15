// Example WebSocket Client untuk testing Quiz Battle
// Run: bun run apps/elysia/scripts/test-quiz-client.ts
/* eslint-disable @typescript-eslint/no-explicit-any */

const WS_URL = 'ws://localhost:4092/api/quiz/battle';

// Simulated JWT token (replace with real token in production)
const TEST_TOKEN =
  'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJ0ZXN0X3VzZXJfMSIsInVzZXJuYW1lIjoiVGVzdFBsYXllcjEifQ.test';

interface WSMessage {
  type: string;
  payload: unknown;
}

class QuizBattleClient {
  private ws: WebSocket | null = null;
  private userId: string;
  private username: string;
  private sessionId: string | null = null;
  private currentMatchId: string | null = null;

  constructor(userId: string, username: string) {
    this.userId = userId;
    this.username = username;
  }

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(WS_URL);

        this.ws.onopen = () => {
          console.log(`✅ [${this.username}] Connected to server`);
          this.authenticate();
          resolve();
        };

        this.ws.onmessage = (event) => {
          this.handleMessage(event.data);
        };

        this.ws.onerror = (error) => {
          console.error(`❌ [${this.username}] WebSocket error:`, error);
          reject(error);
        };

        this.ws.onclose = () => {
          console.log(`🔌 [${this.username}] Disconnected from server`);
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  private send(message: WSMessage): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
      console.log(`📤 [${this.username}] Sent:`, message.type);
    } else {
      console.error(`❌ [${this.username}] WebSocket not connected`);
    }
  }

  private handleMessage(data: string): void {
    try {
      const message: WSMessage = JSON.parse(data);
      console.log(`📥 [${this.username}] Received:`, message.type);

      switch (message.type) {
        case 'auth.connected':
          if (
            typeof message.payload === 'object' &&
            message.payload &&
            'sessionId' in message.payload
          ) {
            this.sessionId = (
              message.payload as { sessionId: string }
            ).sessionId;
            console.log(
              `🔐 [${this.username}] Authenticated! Session: ${this.sessionId}`,
            );
          }
          break;

        case 'auth.error':
          if (
            typeof message.payload === 'object' &&
            message.payload &&
            'message' in message.payload
          ) {
            console.error(
              `❌ [${this.username}] Auth error:`,
              (message.payload as { message: string }).message,
            );
          }
          break;

        case 'matchmaking.searching':
          if (
            typeof message.payload === 'object' &&
            message.payload &&
            'playersInQueue' in message.payload
          ) {
            console.log(
              `🔍 [${this.username}] Searching for match... Queue size: ${(message.payload as { playersInQueue: number }).playersInQueue}`,
            );
          }
          break;

        case 'matchmaking.found':
          if (
            typeof message.payload === 'object' &&
            message.payload &&
            'matchId' in message.payload &&
            'opponent' in message.payload &&
            'startIn' in message.payload
          ) {
            const payload = message.payload as {
              matchId: string;
              opponent: { username: string };
              startIn: number;
            };
            this.currentMatchId = payload.matchId;
            console.log(`🎮 [${this.username}] Match found!`);
            console.log(`   Opponent: ${payload.opponent.username}`);
            console.log(`   Starting in ${payload.startIn} seconds...`);
          }
          break;

        case 'game.started':
          if (
            typeof message.payload === 'object' &&
            message.payload &&
            'matchId' in message.payload &&
            'gameState' in message.payload
          ) {
            const payload = message.payload as {
              matchId: string;
              gameState: { totalQuestions: number };
            };
            console.log(`🚀 [${this.username}] Game started!`);
            console.log(`   Match ID: ${payload.matchId}`);
            console.log(
              `   Total Questions: ${payload.gameState.totalQuestions}`,
            );
          }
          break;

        case 'game.question.new':
          if (
            typeof message.payload === 'object' &&
            message.payload &&
            'questionIndex' in message.payload &&
            'question' in message.payload
          ) {
            const payload = message.payload as {
              questionIndex: number;
              question: { text: string; answers: string[] };
            };
            console.log(
              `❓ [${this.username}] New question ${payload.questionIndex + 1}:`,
            );
            console.log(`   ${payload.question.text}`);
            payload.question.answers.forEach((answer: string, idx: number) => {
              console.log(`   ${idx}: ${answer}`);
            });
            // Auto-answer after random delay (for testing)
            this.autoAnswer(payload);
          }
          break;

        case 'game.answer.received': {
          const result = (message.payload as any).isCorrect
            ? '✅ Correct!'
            : '❌ Wrong!';
          console.log(`${result} [${this.username}] Answer result:`);
          console.log(`   Points earned: ${(message.payload as any).points}`);
          console.log(
            `   Answer time: ${(message.payload as any).answerTime.toFixed(2)}s`,
          );
          break;
        }

        case 'game.opponent.answered': {
          const oppResult = (message.payload as any).isCorrect ? '✅' : '❌';
          console.log(
            `${oppResult} [${this.username}] Opponent answered in ${(message.payload as any).answerTime.toFixed(2)}s`,
          );
          break;
        }

        case 'game.battle.update':
          console.log(`⚔️ [${this.username}] Battle update:`);
          console.log(
            `   Your Health: ${(message.payload as any).gameState.playerHealth}`,
          );
          console.log(
            `   Opponent Health: ${(message.payload as any).gameState.opponentHealth}`,
          );
          break;

        case 'game.over': {
          const isWinner =
            (message.payload as any).winner.userId === this.userId;
          console.log(
            `\n${isWinner ? '🏆' : '😢'} [${this.username}] Game Over! ${isWinner ? 'YOU WIN!' : 'You Lost'}`,
          );
          console.log(`   Winner: ${(message.payload as any).winner.username}`);
          console.log(
            `   Your Score: ${isWinner ? (message.payload as any).winner.finalScore : (message.payload as any).loser.finalScore}`,
          );
          console.log(
            `   Correct Answers: ${isWinner ? (message.payload as any).winner.correctAnswers : (message.payload as any).loser.correctAnswers}/${isWinner ? (message.payload as any).winner.totalAnswers : (message.payload as any).loser.totalAnswers}`,
          );
          console.log(`   Rewards:`);
          const rewards = isWinner
            ? (message.payload as any).rewards.winner
            : (message.payload as any).rewards.loser;
          console.log(`     Points: +${rewards.points}`);
          console.log(`     Experience: +${rewards.experience}`);
          console.log(`     Coins: +${rewards.coins}`);
          break;
        }

        case 'error':
          console.error(
            `❌ [${this.username}] Error:`,
            (message.payload as any).message,
          );
          break;

        default:
          console.log(
            `📨 [${this.username}] Unhandled message:`,
            message.type,
            message.payload,
          );
      }
    } catch (error) {
      console.error(`❌ [${this.username}] Error handling message:`, error);
    }
  }

  private authenticate(): void {
    this.send({
      type: 'auth:connect',
      payload: {
        token: TEST_TOKEN,
        userId: this.userId,
        username: this.username,
        deviceId: `device_${this.userId}`,
      },
    });
  }

  findMatch(gameMode = 'casual', difficulty = 'easy', category = 'all'): void {
    this.send({
      type: 'matchmaking.find',
      payload: {
        userId: this.userId,
        gameMode,
        difficulty,
        category,
      },
    });
  }

  cancelMatchmaking(): void {
    this.send({
      type: 'matchmaking.cancel',
      payload: {
        userId: this.userId,
      },
    });
  }

  private autoAnswer(questionPayload: unknown): void {
    // Simulate thinking time (1-3 seconds)
    const thinkingTime = 1000 + Math.random() * 2000;

    setTimeout(() => {
      const payload = questionPayload as any;
      // Random answer (for testing, 70% chance of correct answer)
      const shouldAnswerCorrect = Math.random() > 0.3;

      let answerIndex: number;
      if (shouldAnswerCorrect) {
        // Get correct answer from question (in real scenario, client doesn't know correct answer)
        // For testing, we'll just pick random
        answerIndex = Math.floor(
          Math.random() * payload.question.answers.length,
        );
      } else {
        answerIndex = Math.floor(
          Math.random() * payload.question.answers.length,
        );
      }

      this.submitAnswer(
        payload.questionIndex,
        payload.question.id,
        answerIndex,
        thinkingTime,
      );
    }, thinkingTime);
  }

  submitAnswer(
    questionIndex: number,
    questionId: string,
    answerIndex: number,
    answerTime: number,
  ): void {
    if (!this.currentMatchId) {
      console.error(`❌ [${this.username}] No active match`);
      return;
    }

    this.send({
      type: 'game.answer.submit',
      payload: {
        matchId: this.currentMatchId,
        userId: this.userId,
        questionId: questionId,
        questionIndex: questionIndex,
        answerIndex: answerIndex,
        answerTime: answerTime,
        timestamp: Date.now(),
      },
    });
  }

  ping(): void {
    this.send({
      type: 'connection.ping',
      payload: {
        timestamp: Date.now(),
      },
    });
  }

  disconnect(): void {
    if (this.ws) {
      this.ws.close();
    }
  }
}

// Test scenario
async function runTest() {
  console.log('🎮 Quiz Battle WebSocket Client Test\n');

  // Create two clients
  const player1 = new QuizBattleClient('test_user_1', 'TestPlayer1');
  const player2 = new QuizBattleClient('test_user_2', 'TestPlayer2');

  try {
    // Connect both players
    console.log('📡 Connecting players...\n');
    await player1.connect();
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await player2.connect();

    // Wait for authentication
    await new Promise((resolve) => setTimeout(resolve, 2000));

    // Start matchmaking for both
    console.log('\n🔍 Starting matchmaking...\n');
    player1.findMatch('casual', 'easy', 'all');
    await new Promise((resolve) => setTimeout(resolve, 1000));
    player2.findMatch('casual', 'easy', 'all');

    // Let the game run (questions will be auto-answered)
    console.log('\n⏳ Waiting for match to complete...\n');
    await new Promise((resolve) => setTimeout(resolve, 60000)); // Wait 60 seconds

    // Cleanup
    console.log('\n🧹 Cleaning up...');
    player1.disconnect();
    player2.disconnect();
  } catch (error) {
    console.error('❌ Test failed:', error);
  }
}

// Run the test
console.log('Starting in 2 seconds...\n');
setTimeout(() => {
  runTest();
}, 2000);
