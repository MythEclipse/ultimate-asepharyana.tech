import { Elysia } from 'elysia';
import { getDatabase } from '../../utils/database';
import { isTokenBlacklisted } from '../../utils/redis';
import { toUserResponse, type User } from '../../models/user';
import type { RowDataPacket } from 'mysql2';

export const meRoute = new Elysia()
  .get('/api/auth/me', async ({ headers, jwt, set }) => {
    const authHeader = headers.authorization;
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      set.status = 401;
      throw new Error('No token provided');
    }

    const token = authHeader.substring(7);

    // Check if token is blacklisted
    const isBlacklisted = await isTokenBlacklisted(token);
    if (isBlacklisted) {
      set.status = 401;
      throw new Error('Token has been revoked');
    }

    // Verify JWT
    const payload = await jwt.verify(token);
    if (!payload) {
      set.status = 401;
      throw new Error('Invalid token');
    }

    const db = await getDatabase();

    // Fetch user
    const [users] = await db.query<(User & RowDataPacket)[]>(
      `SELECT id, email, username, password_hash, full_name, avatar_url,
              email_verified, is_active, role, last_login_at, created_at, updated_at
       FROM users WHERE id = ?`,
      [payload.user_id]
    );

    if (users.length === 0) {
      set.status = 404;
      throw new Error('User not found');
    }

    const user = users[0];

    if (!user.is_active) {
      set.status = 403;
      throw new Error('Account is inactive');
    }

    return {
      success: true,
      user: toUserResponse(user),
    };
  });
