// Game Logic Handlers

import { wsManager } from '../ws-manager';
import { checkAchievementsForUser } from './achievements';
import { updateRankedMMR } from './ranked';
import {
  trackGamePlayed,
  trackGameWon,
  trackCorrectAnswers,
  trackWinStreak,
  trackPerfectGame,
} from './daily-missions';
import type {
  WSMessage,
  GameStartedPayload,
  GameQuestionNewPayload,
  GameAnswerSubmitPayload,
  GameAnswerReceivedPayload,
  GameBattleUpdatePayload,
  GameOpponentAnsweredPayload,
  GameOverPayload,
  QuestionData,
  PlayerGameResult,
  GameRewards,
} from '../types';
import {
  getDb,
  quizQuestions,
  quizAnswers,
  quizMatches,
  quizMatchQuestions,
  quizMatchAnswers,
  quizUserStats,
  eq,
  and,
  sql,
} from '@asepharyana/services';

// Load random questions based on difficulty and category
async function loadQuestions(
  difficulty: string,
  category: string,
  count: number,
): Promise<QuestionData[]> {
  try {
    const db = getDb();

    // Build query conditions
    let conditions;

    if (category !== 'all') {
      conditions = and(
        eq(quizQuestions.difficulty, difficulty),
        eq(quizQuestions.category, category),
      );
    } else {
      conditions = eq(quizQuestions.difficulty, difficulty);
    }

    // Get random questions
    const questions = await db
      .select()
      .from(quizQuestions)
      .where(conditions)
      .orderBy(sql`RAND()`)
      .limit(count);

    // Load answers for each question
    const questionsWithAnswers: QuestionData[] = [];

    for (const question of questions) {
      const answers = await db
        .select()
        .from(quizAnswers)
        .where(eq(quizAnswers.questionId, question.id))
        .orderBy(quizAnswers.answerIndex);

      questionsWithAnswers.push({
        id: question.id,
        text: question.text,
        answers: answers.map((a: { text: string }) => a.text),
        category: question.category,
        difficulty: question.difficulty as 'easy' | 'medium' | 'hard',
      });
    }

    return questionsWithAnswers;
  } catch (error) {
    console.error('[Game] Error loading questions:', error);
    return [];
  }
}

// Start the game match
export async function startGameMatch(matchId: string): Promise<void> {
  console.log(`[Game] startGameMatch called for match ${matchId}`);
  try {
    const match = wsManager.getMatch(matchId);
    if (!match) {
      console.error(`[Game] Match ${matchId} not found in wsManager`);
      return;
    }
    console.log(`[Game] Match found, loading match data from DB...`);

    const db = getDb();

    // Get match settings from database
    const [matchData] = await db
      .select()
      .from(quizMatches)
      .where(eq(quizMatches.id, matchId))
      .limit(1);

    if (!matchData) {
      console.error(`[Game] Match ${matchId} not found in database`);
      return;
    }
    console.log(
      `[Game] Match data loaded: difficulty=${matchData.difficulty}, category=${matchData.category}, totalQuestions=${matchData.totalQuestions}`,
    );

    // Load questions
    const questions = await loadQuestions(
      matchData.difficulty,
      matchData.category,
      50, // Force 50 questions for survival mode
    );

    console.log(
      `[Game] Loaded ${questions.length} questions for match ${matchId}`,
    );

    if (questions.length === 0) {
      console.error(
        `[Game] No questions found for match - difficulty: ${matchData.difficulty}, category: ${matchData.category}`,
      );
      return;
    }

    match.questions = questions;

    // Save match questions to database
    for (let i = 0; i < questions.length; i++) {
      await db.insert(quizMatchQuestions).values({
        id: `mq_${matchId}_${i}`,
        matchId,
        questionId: questions[i].id,
        questionIndex: i,
      });
    }

    // Send game started to both players
    const gameStartedMsg: WSMessage<GameStartedPayload> = {
      type: 'game.started',
      payload: {
        matchId,
        gameState: match.gameState,
        players: [
          {
            userId: match.player1Id,
            username: match.player1.username,
            position: 'left',
          },
          {
            userId: match.player2Id,
            username: match.player2.username,
            position: 'right',
          },
        ],
        serverTime: Date.now(),
      },
    };

    wsManager.broadcastToMatch(matchId, gameStartedMsg);

    console.log(`[Game] Match ${matchId} started`);

    // Send ALL questions to both players immediately (no delay)
    const allQuestionsMsg: WSMessage<any> = {
      type: 'game.questions.all',
      payload: {
        matchId,
        questions: questions.map((q, idx) => ({
          questionIndex: idx,
          id: q.id,
          text: q.text,
          answers: q.answers,
          category: q.category,
        })),
        totalQuestions: questions.length,
      },
    };

    wsManager.broadcastToMatch(matchId, allQuestionsMsg);
    console.log(
      `[Game] Sent all ${questions.length} questions for match ${matchId}`,
    );
  } catch (error) {
    console.error('[Game] Error starting match:', error);
  }
}

// Send next question to players
async function sendNextQuestion(matchId: string): Promise<void> {
  try {
    const match = wsManager.getMatch(matchId);
    if (!match) return;

    const currentIndex = match.gameState.currentQuestionIndex;

    if (currentIndex >= match.questions.length) {
      // No more questions, end game
      await endGame(matchId, 'all_questions_answered');
      return;
    }

    const question = match.questions[currentIndex];
    match.currentQuestionStartTime = Date.now();

    const questionMsg: WSMessage<GameQuestionNewPayload> = {
      type: 'game.question.new',
      payload: {
        matchId,
        questionIndex: currentIndex,
        question,
        timeLimit: match.gameState.timePerQuestion,
        startTime: match.currentQuestionStartTime,
      },
    };

    wsManager.broadcastToMatch(matchId, questionMsg);

    console.log(
      `[Game] Sent question ${currentIndex + 1}/${match.questions.length} for match ${matchId}`,
    );

    // Auto-timeout after time limit
    setTimeout(
      () => {
        handleQuestionTimeout(matchId, currentIndex);
      },
      match.gameState.timePerQuestion * 1000 + 1000,
    ); // +1s grace period
  } catch (error) {
    console.error('[Game] Error sending question:', error);
  }
}

// Handle player answer submission
export async function handleGameAnswerSubmit(
  sessionId: string,
  payload: GameAnswerSubmitPayload,
): Promise<void> {
  try {
    const connection = wsManager.getConnectionByUserId(payload.userId);
    if (!connection) {
      console.warn(`[Game] No connection found for user ${payload.userId}`);
      return;
    }

    const match = wsManager.getMatch(payload.matchId);
    if (!match) {
      console.warn(`[Game] No match found for matchId ${payload.matchId}`);
      return;
    }

    // Allow spam answering - don't validate question index
    const db = getDb();

    // Get correct answer from database
    const [questionData] = await db
      .select()
      .from(quizQuestions)
      .where(eq(quizQuestions.id, payload.questionId))
      .limit(1);

    if (!questionData) return;

    const isCorrect = payload.answerIndex === questionData.correctAnswer;

    // Save answer to database
    await db.insert(quizMatchAnswers).values({
      id: `ma_${payload.matchId}_${payload.userId}_${payload.questionIndex}`,
      matchId: payload.matchId,
      userId: payload.userId,
      questionId: payload.questionId,
      questionIndex: payload.questionIndex,
      answerIndex: payload.answerIndex,
      isCorrect: isCorrect ? 1 : 0,
      answerTime: Math.round(payload.answerTime),
      points: 0, // No points in health mode
    });

    // Health-based damage logic (like offline mode):
    // - Correct answer = opponent takes damage (-10 HP)
    // - Wrong answer = player takes damage (-10 HP)
    const isPlayer1 = payload.userId === match.player1Id;

    if (isCorrect) {
      // Correct answer - damage OPPONENT
      if (isPlayer1) {
        match.gameState.opponentHealth = Math.max(
          0,
          match.gameState.opponentHealth - 10,
        );
      } else {
        match.gameState.playerHealth = Math.max(
          0,
          match.gameState.playerHealth - 10,
        );
      }
    } else {
      // Wrong answer - damage SELF
      if (isPlayer1) {
        match.gameState.playerHealth = Math.max(
          0,
          match.gameState.playerHealth - 10,
        );
      } else {
        match.gameState.opponentHealth = Math.max(
          0,
          match.gameState.opponentHealth - 10,
        );
      }
    }

    // Get current player's health after update
    const playerHealth = isPlayer1
      ? match.gameState.playerHealth
      : match.gameState.opponentHealth;
    const opponentHealth = isPlayer1
      ? match.gameState.opponentHealth
      : match.gameState.playerHealth;

    // Send answer result to player with updated healths
    const answerResultMsg: WSMessage<GameAnswerReceivedPayload> = {
      type: 'game.answer.received',
      payload: {
        questionIndex: payload.questionIndex,
        isCorrect,
        correctAnswerIndex: questionData.correctAnswer,
        points: 0,
        answerTime: payload.answerTime / 1000,
        playerHealth: playerHealth, // This player's health
        opponentHealth: opponentHealth, // Opponent's health from this player's perspective
      },
    };

    wsManager.sendToUser(payload.userId, answerResultMsg);

    // Broadcast health update
    const battleUpdateMsg: WSMessage<any> = {
      type: 'game.battle.update',
      payload: {
        matchId: payload.matchId,
        player1Health: match.gameState.playerHealth,
        player2Health: match.gameState.opponentHealth,
      },
    };

    wsManager.broadcastToMatch(payload.matchId, battleUpdateMsg);

    // Check if game should end (health <= 0)
    console.log(
      `[Game] Health Check: User ${payload.userId} health=${playerHealth} (isPlayer1=${isPlayer1})`,
    );

    if (playerHealth <= 0) {
      console.log(
        `[Game] Player ${payload.userId} health depleted (0), ending game matchId=${payload.matchId}`,
      );
      await endGame(payload.matchId, 'health_depleted');
    }

    // Notify opponent of answer (with animation data)
    const opponentUserId = isPlayer1 ? match.player2Id : match.player1Id;
    const opponentMsg: WSMessage<GameOpponentAnsweredPayload> = {
      type: 'game.opponent.answered',
      payload: {
        opponentId: payload.userId,
        questionIndex: payload.questionIndex,
        answerTime: payload.answerTime / 1000,
        isCorrect,
        animation: isCorrect ? 'attack' : 'hurt',
      },
    };

    wsManager.sendToUser(opponentUserId, opponentMsg);
  } catch (error) {
    console.error('[Game] Error handling answer submission:', error);
  }
}

// Handle question timeout
async function handleQuestionTimeout(
  matchId: string,
  questionIndex: number,
): Promise<void> {
  try {
    const match = wsManager.getMatch(matchId);
    if (!match) return;

    // Check if already moved to next question
    if (match.gameState.currentQuestionIndex !== questionIndex) return;

    const db = getDb();
    const question = match.questions[questionIndex];

    // Get correct answer
    const [questionData] = await db
      .select()
      .from(quizQuestions)
      .where(eq(quizQuestions.id, question.id))
      .limit(1);

    if (!questionData) return;

    // Apply damage to both players for not answering (timeout penalty)
    const damageAmount = 10;
    match.gameState.playerHealth = Math.max(
      0,
      match.gameState.playerHealth - damageAmount,
    );
    match.gameState.opponentHealth = Math.max(
      0,
      match.gameState.opponentHealth - damageAmount,
    );

    // Update database with new health values
    await db
      .update(quizMatches)
      .set({
        player1Health: match.gameState.playerHealth,
        player2Health: match.gameState.opponentHealth,
      })
      .where(eq(quizMatches.id, matchId));

    // Broadcast health update to both players
    const battleUpdateMsg: WSMessage<any> = {
      type: 'game.battle.update',
      payload: {
        matchId,
        player1Health: match.gameState.playerHealth,
        player2Health: match.gameState.opponentHealth,
      },
    };
    wsManager.broadcastToMatch(matchId, battleUpdateMsg);

    // Move to next question
    match.gameState.currentQuestionIndex++;

    // Send timeout notification
    const timeoutMsg: WSMessage = {
      type: 'game.question.timeout',
      payload: {
        matchId,
        questionIndex,
        correctAnswerIndex: questionData.correctAnswer,
        players: [
          {
            userId: match.player1Id,
            answered: false,
            isCorrect: false,
            tookDamage: damageAmount,
          },
          {
            userId: match.player2Id,
            answered: false,
            isCorrect: false,
            tookDamage: damageAmount,
          },
        ],
      },
    };

    wsManager.broadcastToMatch(matchId, timeoutMsg);

    // Check if game should end due to health depletion
    if (
      match.gameState.playerHealth <= 0 ||
      match.gameState.opponentHealth <= 0
    ) {
      console.log(
        `[Game] Health depleted after timeout, ending game matchId=${matchId}`,
      );
      await endGame(matchId, 'health_depleted');
      return;
    }

    // Send next question after delay
    setTimeout(() => {
      sendNextQuestion(matchId);
    }, 3000);
  } catch (error) {
    console.error('[Game] Error handling question timeout:', error);
  }
}

// End game and calculate results
async function endGame(
  matchId: string,
  reason: 'health_depleted' | 'all_questions_answered' | 'player_disconnected',
  forfeitingPlayerId?: string,
): Promise<void> {
  try {
    const match = wsManager.getMatch(matchId);
    if (!match) return;

    // Guard against multiple calls - check if already ending/finished
    if (match.status === 'finished') {
      console.log(
        `[Game] EndGame already called for match ${matchId}, skipping`,
      );
      return;
    }
    match.status = 'finished'; // Mark as finished immediately to prevent re-entrance

    const db = getDb();

    // Determine winner
    const player1Health = match.gameState.playerHealth;
    const player2Health = match.gameState.opponentHealth;

    console.log(`[Game] EndGame Debug: reason=${reason}`);
    console.log(
      `[Game] EndGame Debug: player1Health=${player1Health}, player2Health=${player2Health}`,
    );

    let winnerId: string;
    let loserId: string;

    if (reason === 'player_disconnected' && forfeitingPlayerId) {
      // Player who disconnected loses
      loserId = forfeitingPlayerId;
      winnerId =
        loserId === match.player1Id ? match.player2Id : match.player1Id;
      console.log(`[Game] Player ${loserId} forfeited by disconnecting`);
    } else {
      // Health-based winner: player with more health wins
      // If both at 0, player1 wins by default
      winnerId =
        player1Health >= player2Health ? match.player1Id : match.player2Id;
      loserId =
        winnerId === match.player1Id ? match.player2Id : match.player1Id;
    }

    console.log(
      `[Game] EndGame Debug: winnerId=${winnerId}, loserId=${loserId}`,
    );

    // Get player answers stats
    const player1Answers = await db
      .select()
      .from(quizMatchAnswers)
      .where(
        and(
          eq(quizMatchAnswers.matchId, matchId),
          eq(quizMatchAnswers.userId, match.player1Id),
        ),
      );

    const player2Answers = await db
      .select()
      .from(quizMatchAnswers)
      .where(
        and(
          eq(quizMatchAnswers.matchId, matchId),
          eq(quizMatchAnswers.userId, match.player2Id),
        ),
      );

    const player1Correct = player1Answers.filter(
      (a: { isCorrect: number }) => a.isCorrect === 1,
    ).length;
    const player2Correct = player2Answers.filter(
      (a: { isCorrect: number }) => a.isCorrect === 1,
    ).length;

    const player1AvgTime =
      player1Answers.length > 0
        ? player1Answers.reduce(
            (sum: number, a: { answerTime: number }) => sum + a.answerTime,
            0,
          ) /
          player1Answers.length /
          1000
        : 0;

    const player2AvgTime =
      player2Answers.length > 0
        ? player2Answers.reduce(
            (sum: number, a: { answerTime: number }) => sum + a.answerTime,
            0,
          ) /
          player2Answers.length /
          1000
        : 0;

    // Calculate rewards
    const winnerRewards: GameRewards = {
      points: 100,
      experience: 150,
      coins: 50,
    };

    const loserRewards: GameRewards = {
      points: 30,
      experience: 50,
      coins: 10,
    };

    // Update database
    await db
      .update(quizMatches)
      .set({
        winnerId,
        status: 'finished',
        finishedAt: new Date(),
        player1Health,
        player2Health,
      })
      .where(eq(quizMatches.id, matchId));

    // Get stats BEFORE updates for MMR calculation
    const [matchRecord] = await db
      .select()
      .from(quizMatches)
      .where(eq(quizMatches.id, matchId))
      .limit(1);
    let winnerOldMMR = 0;
    let loserOldMMR = 0;

    if (matchRecord && matchRecord.gameMode === 'ranked') {
      const [winnerStats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, winnerId))
        .limit(1);
      const [loserStats] = await db
        .select()
        .from(quizUserStats)
        .where(eq(quizUserStats.userId, loserId))
        .limit(1);
      winnerOldMMR = winnerStats?.points || 0;
      loserOldMMR = loserStats?.points || 0;
    }

    // Update user stats
    await updateUserStats(winnerId, true, winnerRewards);
    await updateUserStats(loserId, false, loserRewards);

    // Update MMR for ranked matches AFTER stats update
    if (matchRecord && matchRecord.gameMode === 'ranked') {
      await updateRankedMMR(winnerId, loserOldMMR, true);
      await updateRankedMMR(loserId, winnerOldMMR, false);
    }

    // Check achievements for both players
    await checkAchievementsForUser(winnerId);
    await checkAchievementsForUser(loserId);

    // Track daily missions for both players
    await trackGamePlayed(match.player1Id);
    await trackGamePlayed(match.player2Id);
    await trackGameWon(winnerId);

    await trackCorrectAnswers(match.player1Id, player1Correct);
    await trackCorrectAnswers(match.player2Id, player2Correct);

    const [player1Stats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, match.player1Id))
      .limit(1);
    const [player2Stats] = await db
      .select()
      .from(quizUserStats)
      .where(eq(quizUserStats.userId, match.player2Id))
      .limit(1);

    if (player1Stats)
      await trackWinStreak(match.player1Id, player1Stats.currentStreak);
    if (player2Stats)
      await trackWinStreak(match.player2Id, player2Stats.currentStreak);

    if (player1Correct === player1Answers.length && player1Answers.length > 0) {
      await trackPerfectGame(match.player1Id);
    }
    if (player2Correct === player2Answers.length && player2Answers.length > 0) {
      await trackPerfectGame(match.player2Id);
    }

    // Prepare game over message
    const winnerResult: PlayerGameResult =
      winnerId === match.player1Id
        ? {
            userId: match.player1Id,
            username: match.player1.username,
            finalHealth: player1Health,
            finalScore: 0,
            correctAnswers: player1Correct,
            totalAnswers: player1Answers.length,
            averageTime: player1AvgTime,
          }
        : {
            userId: match.player2Id,
            username: match.player2.username,
            finalHealth: player2Health,
            finalScore: 0,
            correctAnswers: player2Correct,
            totalAnswers: player2Answers.length,
            averageTime: player2AvgTime,
          };

    const loserResult: PlayerGameResult =
      loserId === match.player1Id
        ? {
            userId: match.player1Id,
            username: match.player1.username,
            finalHealth: player1Health,
            finalScore: 0,
            correctAnswers: player1Correct,
            totalAnswers: player1Answers.length,
            averageTime: player1AvgTime,
          }
        : {
            userId: match.player2Id,
            username: match.player2.username,
            finalHealth: player2Health,
            finalScore: 0,
            correctAnswers: player2Correct,
            totalAnswers: player2Answers.length,
            averageTime: player2AvgTime,
          };

    const gameOverMsg: WSMessage<GameOverPayload> = {
      type: 'game.over',
      payload: {
        matchId,
        reason,
        winner: winnerResult,
        loser: loserResult,
        rewards: {
          winner: winnerRewards,
          loser: loserRewards,
        },
        gameHistory: {
          historyId: matchId,
          playedAt: Date.now(),
          duration: 0, // Calculate from match data
        },
      },
    };

    // Broadcast game over to both players
    wsManager.broadcastToMatch(matchId, gameOverMsg);

    // CRITICAL: Clear currentMatchId from player connections so they can join new games
    const player1Conn = wsManager.getConnectionByUserId(match.player1Id);
    const player2Conn = wsManager.getConnectionByUserId(match.player2Id);

    if (player1Conn) {
      player1Conn.currentMatchId = undefined;
      console.log(
        `[Game] Cleared currentMatchId for player ${match.player1Id}`,
      );
    }
    if (player2Conn) {
      player2Conn.currentMatchId = undefined;
      console.log(
        `[Game] Cleared currentMatchId for player ${match.player2Id}`,
      );
    }

    // Update user statuses
    wsManager.updateUserStatus(match.player1Id, 'online');
    wsManager.updateUserStatus(match.player2Id, 'online');

    // Remove match after 5 seconds
    setTimeout(() => {
      wsManager.removeMatch(matchId);
    }, 5000);

    console.log(`[Game] Match ${matchId} ended. Winner: ${winnerId}`);
  } catch (error) {
    console.error('[Game] Error ending game:', error);
  }
}

// Update user stats after game
async function updateUserStats(
  userId: string,
  isWinner: boolean,
  rewards: GameRewards,
): Promise<void> {
  try {
    const db = getDb();

    await db
      .update(quizUserStats)
      .set({
        points: sql`${quizUserStats.points} + ${rewards.points}`,
        wins: isWinner
          ? sql`${quizUserStats.wins} + 1`
          : sql`${quizUserStats.wins}`,
        losses: !isWinner
          ? sql`${quizUserStats.losses} + 1`
          : sql`${quizUserStats.losses}`,
        totalGames: sql`${quizUserStats.totalGames} + 1`,
        experience: sql`${quizUserStats.experience} + ${rewards.experience}`,
        coins: sql`${quizUserStats.coins} + ${rewards.coins}`,
        currentStreak: isWinner
          ? sql`${quizUserStats.currentStreak} + 1`
          : sql`0`,
      })
      .where(eq(quizUserStats.userId, userId));
  } catch (error) {
    console.error('[Game] Error updating user stats:', error);
  }
}

// End game due to player forfeit (disconnect)
export async function endGameByForfeit(
  matchId: string,
  forfeitingPlayerId: string,
): Promise<void> {
  console.log(
    `[Game] Player ${forfeitingPlayerId} forfeiting match ${matchId}`,
  );
  await endGame(matchId, 'player_disconnected', forfeitingPlayerId);
}
