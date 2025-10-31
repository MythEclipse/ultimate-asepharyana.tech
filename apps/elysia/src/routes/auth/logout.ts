import { Elysia } from 'elysia';
import { blacklistToken } from '../../utils/redis';

export const logoutRoute = new Elysia()
  .post(
    '/api/auth/logout',
    async ({ headers, set }) => {
      const authHeader = headers.authorization;
      if (!authHeader || !authHeader.startsWith('Bearer ')) {
        set.status = 401;
        throw new Error('No token provided');
      }

      const token = authHeader.substring(7);

      try {
        // Blacklist the token (expires in 24 hours)
        await blacklistToken(token, 24 * 3600);

        return {
          success: true,
          message: 'Logged out successfully',
        };
      } catch (error) {
        console.error('Logout error:', error);
        return {
          success: true,
          message: 'Logged out successfully',
        };
      }
    }
  );
