import { Elysia, t } from 'elysia';
import { v4 as uuidv4 } from 'uuid';
import { getDatabase } from '../../utils/database';
import type { RowDataPacket } from 'mysql2';

export const verifyRoute = new Elysia()
  .get(
    '/api/auth/verify',
    async ({ query, set }) => {
      const { token } = query;

      if (!token) {
        set.status = 400;
        throw new Error('Verification token is required');
      }

      const db = await getDatabase();

      // Find verification token
      const [tokens] = await db.query<RowDataPacket[]>(
        `SELECT user_id, expires_at FROM email_verification_tokens
         WHERE token = ? AND expires_at > NOW()`,
        [token]
      );

      if (tokens.length === 0) {
        set.status = 400;
        throw new Error('Invalid or expired verification token');
      }

      const { user_id } = tokens[0];

      // Update user email_verified
      await db.query(
        'UPDATE users SET email_verified = TRUE, updated_at = NOW() WHERE id = ?',
        [user_id]
      );

      // Delete used token
      await db.query('DELETE FROM email_verification_tokens WHERE token = ?', [token]);

      return {
        success: true,
        message: 'Email verified successfully',
      };
    },
    {
      query: t.Object({
        token: t.String(),
      }),
    }
  );
