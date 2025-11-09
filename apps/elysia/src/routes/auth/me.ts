import { Elysia } from 'elysia';
import { prisma } from '../../utils/prisma';
import { verifyJWT } from '../../utils/jwt';
import { isTokenBlacklisted } from '../../utils/redis';

export const meRoute = new Elysia()
  .get('/me', async ({ headers, set }) => {
    const authHeader = headers.authorization;
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      set.status = 401;
      throw new Error('No token provided');
    }

    const token = authHeader.substring(7);

    // Check if token is blacklisted
    const isBlacklisted = await isTokenBlacklisted(token);
    if (isBlacklisted) {
      set.status = 401;
      throw new Error('Token has been revoked');
    }

    // Verify JWT
    const payload = await verifyJWT(token);
    if (!payload) {
      set.status = 401;
      throw new Error('Invalid token');
    }

    // Fetch user using Prisma
    const user = await prisma.user.findUnique({
      where: { id: payload.user_id },
      select: {
        id: true,
        email: true,
        name: true,
        isVerified: true,
        createdAt: true,
        updatedAt: true,
      },
    });

    if (!user) {
      set.status = 404;
      throw new Error('User not found');
    }

    return {
      success: true,
      user,
    };
  });
