import { Elysia } from 'elysia';
import { getDatabase } from '../utils/prisma';
import { users } from '@asepharyana/services';
import { eq, desc } from 'drizzle-orm';

export const apiRoutes = new Elysia({ prefix: '/api' })
  .get('/users', async () => {
    const db = getDatabase();
    const allUsers = await db
      .select({
        id: users.id,
        email: users.email,
        name: users.name,
        emailVerified: users.emailVerified,
        image: users.image,
        role: users.role,
      })
      .from(users)
      .limit(50)
      .orderBy(desc(users.id));

    return {
      success: true,
      count: allUsers.length,
      users: allUsers,
    };
  })
  .get('/users/:id', async ({ params: { id }, set }) => {
    const db = getDatabase();
    const result = await db
      .select({
        id: users.id,
        email: users.email,
        name: users.name,
        emailVerified: users.emailVerified,
        image: users.image,
        role: users.role,
      })
      .from(users)
      .where(eq(users.id, id))
      .limit(1);

    const user = result[0];

    if (!user) {
      set.status = 404;
      throw new Error('User not found');
    }

    return {
      success: true,
      user,
    };
  });
