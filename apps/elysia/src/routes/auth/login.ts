import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { prisma } from '../../utils/prisma';
import { signJWT } from '../../utils/jwt';

interface LoginBody {
  email: string;
  password: string;
  rememberMe?: boolean;
}

interface LoginResponse {
  success: boolean;
  user: {
    id: string;
    email: string;
    name: string | null;
    isVerified: boolean;
  };
  accessToken: string;
  refreshToken: string;
  tokenType: string;
  expiresIn: number;
}

export const loginRoute = new Elysia()
  .post(
    '/api/auth/login',
    async ({ body, set }): Promise<LoginResponse> => {
      const { email, password, rememberMe } = body as LoginBody;

      // Find user by email
      const user = await prisma.user.findUnique({
        where: { email },
      });

      if (!user) {
        set.status = 401;
        throw new Error('Invalid credentials');
      }

      // Verify password
      const passwordValid = await bcrypt.compare(password, user.password);
      if (!passwordValid) {
        set.status = 401;
        throw new Error('Invalid credentials');
      }

      // Generate JWT tokens
      const tokenExpiry = rememberMe ? 30 * 24 * 3600 : 24 * 3600; // 30 days or 24 hours

      const accessToken = await signJWT({
        user_id: user.id,
        email: user.email,
        name: user.name || '',
      }, tokenExpiry);

      // Generate refresh token
      const refreshExpiresAt = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000); // 30 days

      // Store refresh token in database
      const session = await prisma.session.create({
        data: {
          userId: user.id,
          token: accessToken,
          expiresAt: refreshExpiresAt,
        },
      });

      return {
        success: true,
        user: {
          id: user.id,
          email: user.email,
          name: user.name,
          isVerified: user.isVerified,
        },
        accessToken,
        refreshToken: session.token,
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
    }
  );
