import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { prisma } from '../../utils/prisma';

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

      // Find reset token
      const resetToken = await prisma.passwordResetToken.findUnique({
        where: { token },
      });

      if (!resetToken) {
        set.status = 400;
        throw new Error('Invalid reset token');
      }

      if (resetToken.used) {
        set.status = 400;
        throw new Error('Reset token has already been used');
      }

      if (resetToken.expiresAt < new Date()) {
        set.status = 400;
        throw new Error('Reset token has expired');
      }

      // Hash new password
      const hashedPassword = await bcrypt.hash(new_password, 10);

      // Update password and mark token as used in a transaction
      await prisma.$transaction([
        prisma.user.update({
          where: { id: resetToken.userId },
          data: { password: hashedPassword },
        }),
        prisma.passwordResetToken.update({
          where: { id: resetToken.id },
          data: { used: true },
        }),
      ]);

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
