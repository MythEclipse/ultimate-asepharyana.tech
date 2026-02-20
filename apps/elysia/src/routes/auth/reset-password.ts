import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { getDb, users, passwordResetTokens, eq } from '../../services';

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

export const resetPasswordRoute = new Elysia().post(
  '/reset-password',
  async ({ body, set }) => {
    const db = getDb();
    const { token, new_password } = body as {
      token: string;
      new_password: string;
    };

    const passwordError = validatePassword(new_password);
    if (passwordError) {
      set.status = 400;
      throw new Error(passwordError);
    }

    const resetTokenResult = await db
      .select()
      .from(passwordResetTokens)
      .where(eq(passwordResetTokens.token, token))
      .limit(1);

    const resetToken = resetTokenResult[0];

    if (!resetToken) {
      set.status = 400;
      throw new Error('Invalid reset token');
    }

    if (resetToken.used !== 0) {
      set.status = 400;
      throw new Error('Reset token has already been used');
    }

    if (resetToken.expiresAt < new Date()) {
      set.status = 400;
      throw new Error('Reset token has expired');
    }

    const hashedPassword = await bcrypt.hash(new_password, 10);

    await db
      .update(users)
      .set({ password: hashedPassword })
      .where(eq(users.id, resetToken.userId));

    await db
      .update(passwordResetTokens)
      .set({ used: 1 })
      .where(eq(passwordResetTokens.id, resetToken.id));

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
  },
);
