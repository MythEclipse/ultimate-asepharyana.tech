import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { getDatabase } from '../../utils/database';
import type { RowDataPacket } from 'mysql2';

function validatePassword(password: string): string | null {
  if (password.length < 8) {
    return 'Password must be at least 8 characters';
  }

  const hasUppercase = /[A-Z]/.test(password);
  const hasLowercase = /[a-z]/.test(password);
  const hasDigit = /\d/.test(password);
  const hasSpecial = /[^A-Za-z0-9]/.test(password);

  if (!hasUppercase || !hasLowercase || !hasDigit) {
    return 'Password must contain uppercase, lowercase, and numbers';
  }

  if (!hasSpecial) {
    return 'Password should contain at least one special character';
  }

  return null;
}

export const resetPasswordRoute = new Elysia()
  .post(
    '/api/auth/reset-password',
    async ({ body, set }) => {
      const { token, new_password } = body as { token: string; new_password: string };

      // Validate password
      const passwordError = validatePassword(new_password);
      if (passwordError) {
        set.status = 400;
        throw new Error(passwordError);
      }

      const db = await getDatabase();

      // Find reset token
      const [tokens] = await db.query<RowDataPacket[]>(
        `SELECT user_id, expires_at, used FROM password_reset_tokens
         WHERE token = ?`,
        [token]
      );

      if (tokens.length === 0) {
        set.status = 400;
        throw new Error('Invalid reset token');
      }

      const resetToken = tokens[0];

      if (resetToken.used) {
        set.status = 400;
        throw new Error('Reset token has already been used');
      }

      if (new Date(resetToken.expires_at) < new Date()) {
        set.status = 400;
        throw new Error('Reset token has expired');
      }

      // Hash new password
      const password_hash = await bcrypt.hash(new_password, 10);

      // Update password
      await db.query(
        'UPDATE users SET password_hash = ?, updated_at = NOW() WHERE id = ?',
        [password_hash, resetToken.user_id]
      );

      // Mark token as used
      await db.query(
        'UPDATE password_reset_tokens SET used = TRUE WHERE token = ?',
        [token]
      );

      return {
        success: true,
        message: 'Password has been reset successfully',
      };
    },
    {
      body: t.Object({
        token: t.String(),
        new_password: t.String({ minLength: 8 }),
      }),
    }
  );
