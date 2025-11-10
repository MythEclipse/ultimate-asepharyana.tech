import { Elysia, t } from 'elysia';
import { getDatabase } from '../../utils/prisma';
import { users, passwordResetTokens, eq } from '@asepharyana/services';
import type { NewPasswordResetToken } from '@asepharyana/services';
import { sendPasswordResetEmail } from '../../utils/email';

function generateToken(): string {
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);
  return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');
}

export const forgotPasswordRoute = new Elysia()
  .post(
    '/forgot-password',
    async ({ body }) => {
      const db = getDatabase();
      const { email } = body as { email: string };

      const userResult = await db
        .select()
        .from(users)
        .where(eq(users.email, email))
        .limit(1);

      const user = userResult[0];

      if (!user) {
        return {
          success: true,
          message: 'If the email exists, a password reset link has been sent',
        };
      }

      const resetToken = generateToken();
      const expiresAt = new Date(Date.now() + 60 * 60 * 1000);

      const newToken: NewPasswordResetToken = {
        id: `prt_${Date.now()}_${user.id}`,
        userId: user.id,
        token: resetToken,
        expiresAt: expiresAt,
        used: 0,
      };

      await db.insert(passwordResetTokens).values(newToken);

      try {
        await sendPasswordResetEmail(email, user.name || 'User', resetToken);
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
