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
      matchData.totalQuestions,
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

    // Send first question after 2 seconds
    setTimeout(() => {
      sendNextQuestion(matchId);
    }, 2000);
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
    const connection = wsManager.getConnection(sessionId);
    if (!connection) return;

    const match = wsManager.getMatch(payload.matchId);
    if (!match) return;

    // Verify question index
    if (payload.questionIndex !== match.gameState.currentQuestionIndex) {
      console.warn('[Game] Answer submitted for wrong question index');
      return;
    }

    const db = getDb();

    // Get correct answer from database
    const [questionData] = await db
      .select()
      .from(quizQuestions)
      .where(eq(quizQuestions.id, payload.questionId))
      .limit(1);

    if (!questionData) return;

    const isCorrect = payload.answerIndex === questionData.correctAnswer;
    const answerTimeSeconds = payload.answerTime / 1000;

    // Calculate points (faster answer = more points)
    let points = 0;
    if (isCorrect) {
      const maxPoints = 100;
      const timeBonus = Math.max(
        0,
        match.gameState.timePerQuestion - answerTimeSeconds,
      );
      points = Math.round(
        maxPoints * (1 + timeBonus / match.gameState.timePerQuestion),
      );
    }

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
      points,
    });

    // Send answer result to player
    const answerResultMsg: WSMessage<GameAnswerReceivedPayload> = {
      type: 'game.answer.received',
      payload: {
        questionIndex: payload.questionIndex,
        isCorrect,
        correctAnswerIndex: questionData.correctAnswer,
        points,
        answerTime: answerTimeSeconds,
      },
    };

    wsManager.sendToUser(payload.userId, answerResultMsg);

    // Update game state
    const isPlayer1 = payload.userId === match.player1Id;
    const damage = isCorrect ? 0 : 20; // Take damage if wrong

    if (isPlayer1) {
      match.gameState.playerScore = (match.gameState.playerScore || 0) + points;
      match.gameState.playerHealth = Math.max(
        0,
        match.gameState.playerHealth - damage,
      );
      if (!isCorrect) {
        match.gameState.opponentHealth = Math.max(
          0,
          match.gameState.opponentHealth - 20,
        );
      }
    } else {
      match.gameState.opponentScore =
        (match.gameState.opponentScore || 0) + points;
      match.gameState.opponentHealth = Math.max(
        0,
        match.gameState.opponentHealth - damage,
      );
      if (!isCorrect) {
        match.gameState.playerHealth = Math.max(
          0,
          match.gameState.playerHealth - 20,
        );
      }
    }

    // Notify opponent
    const opponentId = isPlayer1 ? match.player2Id : match.player1Id;
    const opponentMsg: WSMessage<GameOpponentAnsweredPayload> = {
      type: 'game.opponent.answered',
      payload: {
        opponentId: payload.userId,
        questionIndex: payload.questionIndex,
        answerTime: answerTimeSeconds,
        isCorrect,
        animation: isCorrect ? 'attack' : 'hurt',
      },
    };

    wsManager.sendToUser(opponentId, opponentMsg);

    // Broadcast battle update
    const battleUpdateMsg: WSMessage<GameBattleUpdatePayload> = {
      type: 'game.battle.update',
      payload: {
        matchId: payload.matchId,
        questionIndex: payload.questionIndex,
        event: 'player_answered',
        actor: {
          userId: payload.userId,
          action: isCorrect ? 'attack' : 'hurt',
        },
        gameState: match.gameState,
        animation: {
          type: isCorrect ? 'attack' : 'hurt',
          target: isCorrect ? 'opponent' : 'player',
          damage: isCorrect ? 20 : 0,
        },
      },
    };

    wsManager.broadcastToMatch(payload.matchId, battleUpdateMsg);

    // Check if game should end (health depleted)
    if (
      match.gameState.playerHealth <= 0 ||
      match.gameState.opponentHealth <= 0
    ) {
      await endGame(payload.matchId, 'health_depleted');
    }
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
          { userId: match.player1Id, answered: true, isCorrect: false },
          { userId: match.player2Id, answered: true, isCorrect: false },
        ],
      },
    };

    wsManager.broadcastToMatch(matchId, timeoutMsg);

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
  reason: 'health_depleted' | 'all_questions_answered',
): Promise<void> {
  try {
    const match = wsManager.getMatch(matchId);
    if (!match) return;

    const db = getDb();

    // Determine winner
    const player1Health = match.gameState.playerHealth;
    const player2Health = match.gameState.opponentHealth;
    const player1Score = match.gameState.playerScore || 0;
    const player2Score = match.gameState.opponentScore || 0;

    let winnerId: string;
    let loserId: string;

    if (reason === 'health_depleted') {
      winnerId =
        player1Health > player2Health ? match.player1Id : match.player2Id;
      loserId =
        winnerId === match.player1Id ? match.player2Id : match.player1Id;
    } else {
      winnerId =
        player1Score > player2Score ? match.player1Id : match.player2Id;
      loserId =
        winnerId === match.player1Id ? match.player2Id : match.player1Id;
    }

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
        player1Score,
        player2Score,
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
            finalScore: player1Score,
            correctAnswers: player1Correct,
            totalAnswers: player1Answers.length,
            averageTime: player1AvgTime,
          }
        : {
            userId: match.player2Id,
            username: match.player2.username,
            finalHealth: player2Health,
            finalScore: player2Score,
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
            finalScore: player1Score,
            correctAnswers: player1Correct,
            totalAnswers: player1Answers.length,
            averageTime: player1AvgTime,
          }
        : {
            userId: match.player2Id,
            username: match.player2.username,
            finalHealth: player2Health,
            finalScore: player2Score,
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

    wsManager.broadcastToMatch(matchId, gameOverMsg);

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
