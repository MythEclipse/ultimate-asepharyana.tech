import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { getDb, users, sessions } from '@asepharyana/services';
import type { NewSession } from '@asepharyana/services';
import { eq } from '@asepharyana/services';
import { signJWT } from '../../utils/jwt';
import { rateLimit } from '../../middleware/rateLimit';
import { sanitizeEmail } from '../../utils/validation';
import { authLogger } from '../../utils/logger';

interface LoginBody {
  email: string;
  password: string;
  rememberMe?: boolean;
}

export interface LoginResponse {
  success: boolean;
  user: {
    id: string;
    email: string;
    name: string | null;
    emailVerified: Date | null;
  };
  accessToken: string;
  refreshToken: string;
  tokenType: string;
  expiresIn: number;
}

export const loginRoute = new Elysia()
  .use(
    rateLimit({
      max: 20, // 20 login attempts
      window: 15 * 60 * 1000, // per 15 minutes
      message: 'Too many login attempts, please try again in 15 minutes',
    }),
  )
  .post(
    '/login',
    async ({ body, set }): Promise<LoginResponse> => {
      const db = getDb();
      const { email, password, rememberMe } = body as LoginBody;

      authLogger.loginAttempt(email);

      const sanitizedEmail = sanitizeEmail(email);
      if (!sanitizedEmail) {
        authLogger.loginFailed(email, 'Invalid email format');
        set.status = 400;
        throw new Error('Invalid email format');
      }

      const result = await db
        .select()
        .from(users)
        .where(eq(users.email, sanitizedEmail))
        .limit(1);

      const user = result[0];

      if (!user || !user.password) {
        authLogger.loginFailed(sanitizedEmail, 'User not found or no password');
        set.status = 401;
        throw new Error('Invalid credentials');
      }

      const passwordValid = await bcrypt.compare(password, user.password);
      if (!passwordValid) {
        authLogger.loginFailed(sanitizedEmail, 'Invalid password');
        set.status = 401;
        throw new Error('Invalid credentials');
      }

      const tokenExpiry = rememberMe ? 30 * 24 * 3600 : 24 * 3600;

      const accessToken = await signJWT(
        {
          user_id: user.id,
          email: user.email || '',
          name: user.name || '',
        },
        tokenExpiry,
      );

      const refreshExpiresAt = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000);
      const sessionToken = `session_${user.id}_${Date.now()}`;

      const newSession: NewSession = {
        id: `sess_${Date.now()}_${user.id}`,
        userId: user.id,
        sessionToken: sessionToken,
        expires: refreshExpiresAt,
      };

      await db.insert(sessions).values(newSession);

      authLogger.loginSuccess(user.id, sanitizedEmail);

      return {
        success: true,
        user: {
          id: user.id,
          email: user.email || '',
          name: user.name,
          emailVerified: user.emailVerified,
        },
        accessToken,
        refreshToken: sessionToken,
        tokenType: 'Bearer',
        expiresIn: tokenExpiry,
      };
    },
    {
      body: t.Object({
        email: t.String({ format: 'email' }),
        password: t.String(),
        rememberMe: t.Optional(t.Boolean()),
      }),
    },
  );
