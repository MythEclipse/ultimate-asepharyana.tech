import { Elysia } from 'elysia';
import { getDb, quizMatches, eq, desc, or } from '@asepharyana/services';
import { authMiddleware } from '../middleware/auth';

export const historyRoutes = new Elysia({ prefix: '/api/history' })
  .use(authMiddleware)
  .get('/', async (context) => {
    const { user, set } = context as any;
    try {
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

      return {
        success: true,
        data: history.map((match) => ({
          ...match,
          isWinner: match.winnerId === userId,
          opponent: match.player1Id === userId ? match.player2 : match.player1,
        })),
      };
    } catch (error) {
      console.error('Error fetching history:', error);
      set.status = 500;
      return {
        success: false,
        error: 'Failed to fetch game history',
      };
    }
  });
