import { Elysia, t } from 'elysia';
import { getDb, users, sessions, eq } from '../../services';
import type { NewSession } from '../../services';
import { signJWT } from '../../utils/jwt';

export const refreshTokenRoute = new Elysia().post(
  '/refresh-token',
  async ({ body, set }) => {
    const db = getDb();
    const { refresh_token } = body as { refresh_token: string };

    const sessionResult = await db
      .select()
      .from(sessions)
      .where(eq(sessions.sessionToken, refresh_token))
      .limit(1);

    const session = sessionResult[0];

    if (!session) {
      set.status = 401;
      throw new Error('Invalid refresh token');
    }

    if (session.expires < new Date()) {
      set.status = 401;
      throw new Error('Refresh token has expired');
    }

    const userResult = await db
      .select()
      .from(users)
      .where(eq(users.id, session.userId))
      .limit(1);

    const user = userResult[0];

    if (!user) {
      set.status = 401;
      throw new Error('User not found');
    }

    const tokenExpiry = 24 * 3600;

    const accessToken = await signJWT(
      {
        user_id: user.id,
        email: user.email || '',
        name: user.name || '',
      },
      tokenExpiry,
    );

    const refreshExpiresAt = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000);
    const newSessionToken = `session_${user.id}_${Date.now()}`;

    await db.delete(sessions).where(eq(sessions.id, session.id));

    const newSession: NewSession = {
      id: `sess_${Date.now()}_${user.id}`,
      userId: user.id,
      sessionToken: newSessionToken,
      expires: refreshExpiresAt,
    };

    await db.insert(sessions).values(newSession);

    return {
      success: true,
      accessToken,
      refreshToken: newSessionToken,
      tokenType: 'Bearer',
      expiresIn: tokenExpiry,
    };
  },
  {
    body: t.Object({
      refresh_token: t.String(),
    }),
  },
);
