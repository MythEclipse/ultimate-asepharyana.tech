import { Elysia } from 'elysia';
import { getDb, users, eq } from '../../services';
import { verifyJWT } from '../../utils/jwt';
import { isTokenBlacklisted } from '../../utils/redis';
import { authLogger } from '../../utils/logger';

export const meRoute = new Elysia().get('/me', async ({ headers, set }) => {
  const authHeader = headers.authorization;
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    authLogger.tokenInvalid('No token provided');
    set.status = 401;
    throw new Error('No token provided');
  }

  const token = authHeader.substring(7);

  const isBlacklisted = await isTokenBlacklisted(token);
  if (isBlacklisted) {
    authLogger.tokenInvalid('Token has been revoked');
    set.status = 401;
    throw new Error('Token has been revoked');
  }

  const payload = await verifyJWT(token);
  if (!payload) {
    authLogger.tokenInvalid('Invalid token signature');
    set.status = 401;
    throw new Error('Invalid token');
  }

  authLogger.tokenVerified(payload.user_id);

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
    authLogger.tokenInvalid('User not found in database');
    set.status = 404;
    throw new Error('User not found');
  }

  return {
    success: true,
    user,
  };
});
