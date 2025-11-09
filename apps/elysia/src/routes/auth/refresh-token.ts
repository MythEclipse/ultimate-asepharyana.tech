import { Elysia, t } from 'elysia';
import { prisma } from '../../utils/prisma';
import { signJWT } from '../../utils/jwt';

export const refreshTokenRoute = new Elysia()
  .post(
    '/refresh-token',
    async ({ body, set }) => {
      const { refresh_token } = body as { refresh_token: string };

      // Find session by token
      const session = await prisma.session.findUnique({
        where: { token: refresh_token },
        include: {
          user: {
            select: {
              id: true,
              email: true,
              name: true,
              isVerified: true,
            },
          },
        },
      });

      if (!session) {
        set.status = 401;
        throw new Error('Invalid refresh token');
      }

      if (session.expiresAt < new Date()) {
        set.status = 401;
        throw new Error('Refresh token has expired');
      }

      // Generate new access token
      const tokenExpiry = 24 * 3600; // 24 hours

      const accessToken = await signJWT({
        user_id: session.user.id,
        email: session.user.email,
        name: session.user.name || '',
      }, tokenExpiry);

      // Generate new refresh token and update session
      const refreshExpiresAt = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000);

      // Delete old session and create new one
      await prisma.session.delete({
        where: { id: session.id },
      });

      const newSession = await prisma.session.create({
        data: {
          userId: session.user.id,
          token: accessToken,
          expiresAt: refreshExpiresAt,
        },
      });

      return {
        success: true,
        accessToken,
        refreshToken: newSession.token,
        tokenType: 'Bearer',
        expiresIn: tokenExpiry,
      };
    },
    {
      body: t.Object({
        refresh_token: t.String(),
      }),
    }
  );
