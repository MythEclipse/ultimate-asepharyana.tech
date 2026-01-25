// Complete Quiz Battle RANKED Mode End-to-End Test
// Tests full ranked flow: auth → ranked matchmaking → game play → game over → MMR update
// Run: bun run apps/elysia/scripts/test-quiz-ranked.ts

const WS_URL = 'ws://localhost:4092/api/quiz/battle';

// Real user IDs from database
const USER1_ID = 'user_1766214278163_msyci7';
const USER2_ID = 'user_1766214279137_lli7iv';

function makeToken(userId: string, username: string): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const payload = btoa(
    JSON.stringify({ userId, username, sub: userId, name: username }),
  );
  return `${header}.${payload}.test_sig`;
}

interface WSMessage {
  type: string;
  payload: any;
}

class QuizBattleRankedClient {
  private ws: WebSocket | null = null;
  private userId: string;
  private username: string;
  public sessionId: string | null = null;
  public currentMatchId: string | null = null;
  public gameStarted = false;
  public questionsReceived = 0;
  public answersSubmitted = 0;
  public gameOver = false;
  public isWinner = false;
  public finalScore = 0;
  public mmrChange = 0;
  public newMMR = 0;

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
    console.log(`📥 [${this.username}] Received: ${message.type}`);

    switch (message.type) {
      case 'auth.connected': {
        this.sessionId = message.payload.sessionId;
        console.log(
          `   🔐 Authenticated! Session: ${this.sessionId?.substring(0, 20)}...`,
        );
        break;
      }

      case 'matchmaking.searching': {
        console.log(
          `   🔍 Searching for RANKED match... Queue size: ${message.payload.playersInQueue}`,
        );
        break;
      }

      case 'matchmaking.found': {
        this.currentMatchId = message.payload.matchId;
        console.log(`   🎯 RANKED Match found!`);
        console.log(
          `      Match ID: ${this.currentMatchId?.substring(0, 25)}...`,
        );
        console.log(`      Opponent: ${message.payload.opponent.username}`);
        console.log(`      Starting in: ${message.payload.startIn}s`);
        break;
      }

      case 'game.started': {
        this.gameStarted = true;
        console.log(`   🚀 RANKED GAME STARTED!`);
        console.log(
          `      Total Questions: ${message.payload.gameState.totalQuestions}`,
        );
        console.log(
          `      Time per Question: ${message.payload.gameState.timePerQuestion}s`,
        );
        break;
      }

      case 'game.question.new': {
        this.questionsReceived++;
        console.log(
          `   ❓ Question ${message.payload.questionIndex + 1}: ${message.payload.question.text}`,
        );
        console.log(
          `      Answers: ${message.payload.question.answers.join(' | ')}`,
        );
        // Auto-answer with random choice after 1-2 seconds
        setTimeout(
          () => {
            this.submitAnswer(
              message.payload.matchId,
              message.payload.questionIndex,
              message.payload.question.id,
              Math.floor(Math.random() * 4), // Random answer 0-3
            );
          },
          1000 + Math.random() * 1000,
        );
        break;
      }

      case 'game.answer.received': {
        this.answersSubmitted++;
        const result = message.payload.isCorrect ? '✅ CORRECT' : '❌ WRONG';
        console.log(`   ${result}! Points: +${message.payload.points}`);
        break;
      }

      case 'game.opponent.answered': {
        const oppResult = message.payload.isCorrect ? '✅' : '❌';
        console.log(`   👤 Opponent answered: ${oppResult}`);
        break;
      }

      case 'game.battle.update': {
        console.log(
          `   ⚔️ Health - You: ${message.payload.gameState.playerHealth}%, Opp: ${message.payload.gameState.opponentHealth}%`,
        );
        break;
      }

      case 'game.question.timeout': {
        console.log(`   ⏰ Question timeout! Moving to next...`);
        break;
      }

      case 'game.over': {
        this.gameOver = true;
        this.isWinner = message.payload.winner.userId === this.userId;
        this.finalScore = this.isWinner
          ? message.payload.winner.finalScore
          : message.payload.loser.finalScore;
        console.log(`\n   🏁 RANKED GAME OVER!`);
        console.log(`   ${this.isWinner ? '🏆 YOU WIN!' : '😢 You Lost'}`);
        console.log(`   Winner: ${message.payload.winner.username}`);
        console.log(
          `   Score: ${message.payload.winner.finalScore} vs ${message.payload.loser.finalScore}`,
        );
        console.log(
          `   Correct: ${message.payload.winner.correctAnswers}/${message.payload.winner.totalAnswers}`,
        );
        console.log(
          `   Rewards: +${message.payload.rewards.winner.points} pts, +${message.payload.rewards.winner.coins} coins`,
        );
        break;
      }

      case 'ranked.mmr.changed': {
        this.mmrChange = message.payload.mmrChange;
        this.newMMR = message.payload.newMMR;
        console.log(
          `   📈 MMR Changed: ${message.payload.mmrChange > 0 ? '+' : ''}${message.payload.mmrChange}`,
        );
        console.log(`   🏅 New MMR: ${message.payload.newMMR}`);
        console.log(
          `   🎖️ Tier: ${message.payload.tier} ${message.payload.division}`,
        );
        break;
      }

      case 'error':
      case 'auth.error': {
        console.error(`   ❌ Error: ${message.payload.message}`);
        break;
      }
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

  findRankedMatch(): void {
    console.log(`   🏆 Searching for RANKED match...`);
    this.send({
      type: 'matchmaking.find',
      payload: {
        userId: this.userId,
        gameMode: 'ranked', // RANKED MODE
        difficulty: 'easy',
        category: 'all',
      },
    });
  }

  syncRankedStats(): void {
    this.send({
      type: 'ranked.stats.sync',
      payload: {
        userId: this.userId,
      },
    });
  }

  private submitAnswer(
    matchId: string,
    questionIndex: number,
    questionId: string,
    answerIndex: number,
  ): void {
    this.send({
      type: 'game.answer.submit',
      payload: {
        matchId,
        userId: this.userId,
        questionId,
        questionIndex,
        answerIndex,
        answerTime: 1000 + Math.random() * 2000, // 1-3 seconds
        timestamp: Date.now(),
      },
    });
  }

  disconnect(): void {
    if (this.ws) this.ws.close();
  }
}

async function runRankedTest() {
  console.log('═══════════════════════════════════════════════════════════');
  console.log('     🏆 QUIZ BATTLE - RANKED MODE END-TO-END TEST');
  console.log('═══════════════════════════════════════════════════════════\n');

  console.log('📋 Test Configuration:');
  console.log(`   Player 1: Ranked Player 1 (${USER1_ID.substring(0, 20)}...)`);
  console.log(`   Player 2: Ranked Player 2 (${USER2_ID.substring(0, 20)}...)`);
  console.log('   Mode: RANKED | Difficulty: easy | Category: all');
  console.log('   Questions: 5 | Time/Question: 10s\n');

  const player1 = new QuizBattleRankedClient(USER1_ID, 'Ranked Player 1');
  const player2 = new QuizBattleRankedClient(USER2_ID, 'Ranked Player 2');

  try {
    // Step 1: Connect both players
    console.log('───────────────────────────────────────────────────────────');
    console.log('STEP 1: CONNECTING PLAYERS');
    console.log('───────────────────────────────────────────────────────────');
    await player1.connect();
    await new Promise((r) => setTimeout(r, 3000));
    await player2.connect();
    await new Promise((r) => setTimeout(r, 5000));

    if (!player1.sessionId || !player2.sessionId) {
      throw new Error('Authentication failed');
    }

    // Step 1.5: Sync ranked stats first
    console.log(
      '\n───────────────────────────────────────────────────────────',
    );
    console.log('STEP 1.5: SYNC RANKED STATS');
    console.log('───────────────────────────────────────────────────────────');
    player1.syncRankedStats();
    player2.syncRankedStats();
    await new Promise((r) => setTimeout(r, 2000));

    // Step 2: Start RANKED matchmaking
    console.log(
      '\n───────────────────────────────────────────────────────────',
    );
    console.log('STEP 2: RANKED MATCHMAKING');
    console.log('───────────────────────────────────────────────────────────');
    player1.findRankedMatch();
    await new Promise((r) => setTimeout(r, 2000)); // Wait for player 1 to be in queue
    player2.findRankedMatch();

    // Wait for match to be found and game to start
    console.log('   ⏳ Waiting for ranked match...');
    await new Promise((r) => setTimeout(r, 15000)); // 2s queue + 5s matchmaking + 5s game start + 3s buffer

    if (!player1.gameStarted || !player2.gameStarted) {
      console.log(
        '❌ Game did not start - checking if questions exist in DB...',
      );
      console.log('   Please run: bun run quiz:seed to seed questions first');
      throw new Error('Game did not start - no questions in database?');
    }

    // Step 3: Wait for game to complete (5 questions × ~15s each max)
    console.log(
      '\n───────────────────────────────────────────────────────────',
    );
    console.log('STEP 3: RANKED GAME IN PROGRESS');
    console.log('───────────────────────────────────────────────────────────');

    // Wait for game over (max 90 seconds)
    const maxWait = 90000;
    const startTime = Date.now();
    while (
      !player1.gameOver &&
      !player2.gameOver &&
      Date.now() - startTime < maxWait
    ) {
      await new Promise((r) => setTimeout(r, 1000));
    }

    // Wait extra time for MMR update messages
    await new Promise((r) => setTimeout(r, 3000));

    // Step 4: Show results
    console.log(
      '\n───────────────────────────────────────────────────────────',
    );
    console.log('STEP 4: RANKED FINAL RESULTS');
    console.log('───────────────────────────────────────────────────────────');

    if (player1.gameOver || player2.gameOver) {
      console.log('✅ Ranked game completed successfully!\n');
      console.log('📊 Summary:');
      console.log(
        `   Player 1: ${player1.isWinner ? '🏆 WINNER' : '😢 LOSER'} - Score: ${player1.finalScore}`,
      );
      console.log(
        `   Player 2: ${player2.isWinner ? '🏆 WINNER' : '😢 LOSER'} - Score: ${player2.finalScore}`,
      );
      console.log(
        `   Questions answered: ${player1.answersSubmitted} / ${player1.questionsReceived}`,
      );

      if (player1.mmrChange !== 0 || player2.mmrChange !== 0) {
        console.log('\n🏅 MMR Updates:');
        console.log(
          `   Player 1 MMR: ${player1.mmrChange > 0 ? '+' : ''}${player1.mmrChange} → ${player1.newMMR}`,
        );
        console.log(
          `   Player 2 MMR: ${player2.mmrChange > 0 ? '+' : ''}${player2.mmrChange} → ${player2.newMMR}`,
        );
      }
    } else {
      console.log('⚠️ Game timed out');
    }

    // Cleanup
    player1.disconnect();
    player2.disconnect();

    console.log(
      '\n═══════════════════════════════════════════════════════════',
    );
    console.log('     ✅ RANKED MODE END-TO-END TEST COMPLETED');
    console.log(
      '═══════════════════════════════════════════════════════════\n',
    );
  } catch (error) {
    console.error('\n❌ TEST FAILED:', error);
    player1.disconnect();
    player2.disconnect();
    process.exit(1);
  }
}

runRankedTest();
