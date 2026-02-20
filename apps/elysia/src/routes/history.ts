import { Elysia } from 'elysia';
import { getDb, quizMatches, eq, desc, or } from '../services';
import { authMiddleware } from '../middleware/auth';
import { historyLogger } from '../utils/logger';

export const historyRoutes = new Elysia({ prefix: '/api/history' })
  .use(authMiddleware)
  .get('/', async (context) => {
    const ctx = context as unknown as {
      user: { id: string };
      set: { status: number };
    };
    const { user, set } = ctx;
    try {
      if (!user || !user.id) {
        historyLogger.fetchError('unknown', 'User not authenticated');
        set.status = 401;
        return {
          success: false,
          error: 'Unauthorized: User not authenticated',
        };
      }

      const db = getDb();
      const userId = user.id;

      const history = await db.query.quizMatches.findMany({
        where: or(
          eq(quizMatches.player1Id, userId),
          eq(quizMatches.player2Id, userId),
        ),
        orderBy: desc(quizMatches.createdAt),
        limit: 50,
        with: {
          player1: {
            columns: {
              id: true,
              name: true,
              image: true,
            },
          },
          player2: {
            columns: {
              id: true,
              name: true,
              image: true,
            },
          },
          winner: {
            columns: {
              id: true,
              name: true,
            },
          },
        },
      });

      historyLogger.fetch(userId, history.length);

      return {
        success: true,
        data: history.map((match) => ({
          ...match,
          isWinner: match.winnerId === userId,
          opponent: match.player1Id === userId ? match.player2 : match.player1,
        })),
      };
    } catch (error) {
      historyLogger.fetchError(user?.id || 'unknown', error);
      set.status = 500;
      return {
        success: false,
        error: 'Failed to fetch game history',
      };
    }
  });
