import { Elysia, t } from 'elysia';
import { v4 as uuidv4 } from 'uuid';
import { getDatabase } from '../../utils/database';
import { signJWT } from '../../utils/jwt';
import type { RowDataPacket } from 'mysql2';

export const refreshTokenRoute = new Elysia()
  .post(
    '/api/auth/refresh-token',
    async ({ body, set }) => {
      const { refresh_token } = body as { refresh_token: string };

      const db = await getDatabase();

      // Find refresh token
      const [tokens] = await db.query<RowDataPacket[]>(
        `SELECT user_id, expires_at, revoked FROM refresh_tokens
         WHERE token = ?`,
        [refresh_token]
      );

      if (tokens.length === 0) {
        set.status = 401;
        throw new Error('Invalid refresh token');
      }

      const token = tokens[0];

      if (token.revoked) {
        set.status = 401;
        throw new Error('Refresh token has been revoked');
      }

      if (new Date(token.expires_at) < new Date()) {
        set.status = 401;
        throw new Error('Refresh token has expired');
      }

      // Fetch user
      const [users] = await db.query<RowDataPacket[]>(
        'SELECT id, email, username, is_active FROM users WHERE id = ?',
        [token.user_id]
      );

      if (users.length === 0 || !users[0].is_active) {
        set.status = 401;
        throw new Error('User not found or inactive');
      }

      const user = users[0];

      // Generate new access token
      const token_expiry = 24 * 3600; // 24 hours

      const access_token = await signJWT({
        user_id: user.id,
        email: user.email,
        name: user.username,
      }, token_expiry);

      // Generate new refresh token
      const new_refresh_token = uuidv4();
      const refresh_expires_at = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000);

      // Revoke old refresh token
      await db.query(
        'UPDATE refresh_tokens SET revoked = TRUE WHERE token = ?',
        [refresh_token]
      );

      // Store new refresh token
      await db.query(
        `INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
         VALUES (?, ?, ?, ?, NOW())`,
        [uuidv4(), user.id, new_refresh_token, refresh_expires_at]
      );

      return {
        access_token,
        refresh_token: new_refresh_token,
        token_type: 'Bearer',
        expires_in: token_expiry,
      };
    },
    {
      body: t.Object({
        refresh_token: t.String(),
      }),
    }
  );
