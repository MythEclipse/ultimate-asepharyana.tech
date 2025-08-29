import { db } from '../db';
import { User } from '../types';

export const updateUserImage = async (id: string, image: string) => {
  try {
    const user = await db
      .updateTable('User')
      .set({ image })
      .where('id', '=', id)
      .returningAll()
      .executeTakeFirstOrThrow() as User;
    return user;
  } catch (error) {
    console.error('Error updating user image:', error);
    throw new Error('Failed to update user image');
  }
};

export const getUserById = async (id: string) => {
  try {
    const user = await db
      .selectFrom('User')
      .selectAll()
      .where('id', '=', id)
      .executeTakeFirst() as User | undefined;
    return user;
  } catch (error) {
    console.error('Error fetching user by ID:', error);
    throw new Error('Failed to fetch user by ID');
  }
};

export const upsertGoogleUser = async (
  email: string,
  name: string | null,
  image: string | null,
) => {
  try {
    let user: User | undefined;
    const existingUser = await db
      .selectFrom('User')
      .selectAll()
      .where('email', '=', email)
      .executeTakeFirst() as User | undefined;

    if (existingUser) {
      user = await db
        .updateTable('User')
        .set({ name, image })
        .where('email', '=', email)
        .returningAll()
        .executeTakeFirstOrThrow() as User;
    } else {
      user = await db
        .insertInto('User')
        .values({ id: crypto.randomUUID(), email, name, image, role: 'member' })
        .returningAll()
        .executeTakeFirstOrThrow() as User;
    }
    return user;
  } catch (error) {
    console.error('Error upserting Google user:', error);
    throw new Error('Failed to upsert Google user');
  }
};
