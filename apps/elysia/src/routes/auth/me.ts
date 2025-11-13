import { Elysia } from 'elysia';
import { getDb, users, eq } from '@asepharyana/services';
import { verifyJWT } from '../../utils/jwt';
import { isTokenBlacklisted } from '../../utils/redis';

export const meRoute = new Elysia()
  .get('/me', async ({ headers, set }) => {
    const authHeader = headers.authorization;
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      set.status = 401;
      throw new Error('No token provided');
    }

    const token = authHeader.substring(7);

    const isBlacklisted = await isTokenBlacklisted(token);
    if (isBlacklisted) {
      set.status = 401;
      throw new Error('Token has been revoked');
    }

    const payload = await verifyJWT(token);
    if (!payload) {
      set.status = 401;
      throw new Error('Invalid token');
    }

    const db = getDb();
    const result = await db
      .select({
        id: users.id,
        email: users.email,
        name: users.name,
        emailVerified: users.emailVerified,
        image: users.image,
        role: users.role,
      })
      .from(users)
      .where(eq(users.id, payload.user_id))
      .limit(1);

    const user = result[0];

    if (!user) {
      set.status = 404;
      throw new Error('User not found');
    }

    return {
      success: true,
      user,
    };
  });
