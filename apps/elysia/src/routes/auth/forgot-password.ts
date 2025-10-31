import { Elysia, t } from 'elysia';
import { v4 as uuidv4 } from 'uuid';
import { getDatabase } from '../../utils/database';
import { sendPasswordResetEmail } from '../../utils/email';
import type { RowDataPacket } from 'mysql2';

export const forgotPasswordRoute = new Elysia()
  .post(
    '/api/auth/forgot-password',
    async ({ body }) => {
      const { email } = body as { email: string };

      const db = await getDatabase();

      // Find user
      const [users] = await db.query<RowDataPacket[]>(
        'SELECT id, username FROM users WHERE email = ?',
        [email]
      );

      // Always return success to prevent email enumeration
      if (users.length === 0) {
        return {
          success: true,
          message: 'If the email exists, a password reset link has been sent',
        };
      }

      const user = users[0];

      // Generate reset token
      const reset_token = uuidv4();
      const expires_at = new Date(Date.now() + 60 * 60 * 1000); // 1 hour

      await db.query(
        `INSERT INTO password_reset_tokens (id, user_id, token, expires_at, used, created_at)
         VALUES (?, ?, ?, ?, FALSE, NOW())`,
        [uuidv4(), user.id, reset_token, expires_at]
      );

      // Send reset email
      try {
        await sendPasswordResetEmail(email, user.username, reset_token);
      } catch (error) {
        console.error('Failed to send password reset email:', error);
      }

      return {
        success: true,
        message: 'If the email exists, a password reset link has been sent',
      };
    },
    {
      body: t.Object({
        email: t.String({ format: 'email' }),
      }),
    }
  );
