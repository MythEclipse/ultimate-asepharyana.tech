import { Elysia } from 'elysia';
import { prisma } from '../utils/prisma';

export const apiRoutes = new Elysia({ prefix: '/api' })
  .get('/users', async () => {
    // Get all users (excluding password)
    const users = await prisma.user.findMany({
      select: {
        id: true,
        email: true,
        name: true,
        isVerified: true,
        createdAt: true,
      },
      take: 50, // Limit to 50 users
      orderBy: {
        createdAt: 'desc',
      },
    });

    return {
      success: true,
      count: users.length,
      users,
    };
  })
  .get('/users/:id', async ({ params: { id }, set }) => {
    const user = await prisma.user.findUnique({
      where: { id },
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
