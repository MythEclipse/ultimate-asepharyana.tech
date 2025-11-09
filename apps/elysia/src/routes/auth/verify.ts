import { Elysia, t } from 'elysia';
import { prisma } from '../../utils/prisma';

export const verifyRoute = new Elysia()
  .get(
    '/verify',
    async ({ query, set }) => {
      const { token } = query;

      if (!token) {
        set.status = 400;
        throw new Error('Verification token is required');
      }

      // Find verification token that hasn't expired
      const verificationToken = await prisma.emailVerificationToken.findUnique({
        where: { token },
      });

      if (!verificationToken) {
        set.status = 400;
        throw new Error('Invalid verification token');
      }

      if (verificationToken.expiresAt < new Date()) {
        set.status = 400;
        throw new Error('Verification token has expired');
      }

      // Update user isVerified status
      await prisma.user.update({
        where: { id: verificationToken.userId },
        data: { isVerified: true },
      });

      // Delete used token
      await prisma.emailVerificationToken.delete({
        where: { id: verificationToken.id },
      });

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
