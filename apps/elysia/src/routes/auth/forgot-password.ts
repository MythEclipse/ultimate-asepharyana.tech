import { Elysia, t } from 'elysia';
import { prisma } from '../../utils/prisma';
import { sendPasswordResetEmail } from '../../utils/email';

// Generate secure random token
function generateToken(): string {
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);
  return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');
}

export const forgotPasswordRoute = new Elysia()
  .post(
    '/api/auth/forgot-password',
    async ({ body }) => {
      const { email } = body as { email: string };

      // Find user
      const user = await prisma.user.findUnique({
        where: { email },
        select: {
          id: true,
          name: true,
          email: true,
        },
      });

      // Always return success to prevent email enumeration
      if (!user) {
        return {
          success: true,
          message: 'If the email exists, a password reset link has been sent',
        };
      }

      // Generate reset token (secure random token)
      const resetToken = generateToken();
      const expiresAt = new Date(Date.now() + 60 * 60 * 1000); // 1 hour

      // Create password reset token
      await prisma.passwordResetToken.create({
        data: {
          userId: user.id,
          token: resetToken,
          expiresAt,
          used: false,
        },
      });

      // Send reset email
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
