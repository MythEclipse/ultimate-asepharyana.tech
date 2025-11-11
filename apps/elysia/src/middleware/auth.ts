import { Elysia } from 'elysia';
import { verifyJWT } from '../utils/jwt';

/**
 * Authentication middleware
 * Verifies JWT token and adds user info to context
 */
export const authMiddleware = new Elysia({ name: 'auth' })
  .derive(async ({ headers, set }) => {
    const authHeader = headers.authorization;

    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      set.status = 401;
      throw new Error('Unauthorized: No token provided');
    }

    const token = authHeader.substring(7);
    const payload = await verifyJWT(token);

    if (!payload) {
      set.status = 401;
      throw new Error('Unauthorized: Invalid token');
    }

    return {
      user: {
        id: payload.user_id,
        email: payload.email,
        name: payload.name,
      },
    };
  });

/**
 * Optional auth middleware
 * Adds user info if token is present, but doesn't require it
 */
export const optionalAuthMiddleware = new Elysia({ name: 'optional-auth' })
  .derive(async ({ headers }) => {
    const authHeader = headers.authorization;

    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      return { user: null };
    }

    const token = authHeader.substring(7);
    const payload = await verifyJWT(token);

    if (!payload) {
      return { user: null };
    }

    return {
      user: {
        id: payload.user_id,
        email: payload.email,
        name: payload.name,
      },
    };
  });
