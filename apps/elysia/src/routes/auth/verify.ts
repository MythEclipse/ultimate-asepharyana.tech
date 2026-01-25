import { Elysia, t } from 'elysia';
import {
  getDb,
  users,
  emailVerificationTokens,
  eq,
} from '@asepharyana/services';

export const verifyRoute = new Elysia().get(
  '/verify',
  async ({ query, set }) => {
    const db = getDb();
    const { token } = query;

    if (!token) {
      set.status = 400;
      throw new Error('Verification token is required');
    }

    const result = await db
      .select()
      .from(emailVerificationTokens)
      .where(eq(emailVerificationTokens.token, token))
      .limit(1);

    const verificationToken = result[0];

    if (!verificationToken) {
      set.status = 400;
      throw new Error('Invalid verification token');
    }

    if (verificationToken.expiresAt < new Date()) {
      set.status = 400;
      throw new Error('Verification token has expired');
    }

    await db
      .update(users)
      .set({ emailVerified: new Date() })
      .where(eq(users.id, verificationToken.userId));

    await db
      .delete(emailVerificationTokens)
      .where(eq(emailVerificationTokens.id, verificationToken.id));

    return {
      success: true,
      message: 'Email verified successfully',
    };
  },
  {
    query: t.Object({
      token: t.String(),
    }),
  },
);
