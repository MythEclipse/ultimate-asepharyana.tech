import { Elysia, t } from 'elysia';
import { OAuth2Client } from 'google-auth-library';
import {
  getDb,
  users,
  accounts,
  sessions,
  quizUserStats,
} from '@asepharyana/services';
import type {
  NewUser,
  NewAccount,
  NewSession,
  User,
} from '@asepharyana/services';
import { eq, and } from '@asepharyana/services';
import { config } from '../../config';
import { signJWT } from '../../utils/jwt';

export const googleAuth = new Elysia().post(
  '/google',
  async ({ body, set }) => {
    const { idToken } = body;
    const db = getDb();

    try {
      if (!config.googleClientId) {
        console.warn('GOOGLE_CLIENT_ID not configured');
        // Proceeding for testing, or fail? Better to fail or warn.
        // For now, let's allow it to fail later or Mock if empty (unsafe but dev friendly?)
        // actually if empty verifyIdToken will fail.
      }

      const client = new OAuth2Client(config.googleClientId);
      const ticket = await client.verifyIdToken({
        idToken,
        audience: config.googleClientId,
      });

      const payload = ticket.getPayload();

      if (!payload) {
        set.status = 400;
        return {
          success: false,
          message: 'Invalid token payload',
        };
      }

      const { sub: googleId, email, name, picture } = payload;

      if (!email) {
        set.status = 400;
        return {
          success: false,
          message: 'Email not found in token',
        };
      }

      // 1. Check if user exists by email
      let user = (
        await db.select().from(users).where(eq(users.email, email)).limit(1)
      )[0];

      if (!user) {
        // Create new user
        const userId = `user_${Date.now()}_${Math.random().toString(36).substring(7)}`;

        const newUser: NewUser = {
          id: userId,
          email,
          name: name || 'Google User',
          image: picture || null,
          role: 'user',
          password: '', // No password for OAuth users
          emailVerified: new Date(), // Google verified this email
          refreshToken: null,
        };

        await db.insert(users).values(newUser);

        // Initialize user stats
        await db.insert(quizUserStats).values({
          id: `qus_${userId}`,
          userId: userId,
          points: 0,
          wins: 0,
          losses: 0,
          totalGames: 0,
          experience: 0,
          coins: 0,
          currentStreak: 0,
          bestStreak: 0,
          draws: 0,
          totalCorrectAnswers: 0,
          totalQuestions: 0,
          level: 1,
        });

        user = newUser as User;
      }

      if (!user) {
        throw new Error('Failed to create or find user');
      }

      // 2. Check if account link exists
      const existingAccount = (
        await db
          .select()
          .from(accounts)
          .where(
            and(
              eq(accounts.provider, 'google'),
              eq(accounts.providerAccountId, googleId),
            ),
          )
          .limit(1)
      )[0];

      if (!existingAccount) {
        // Link user to google account
        const accountId = `acc_${Date.now()}_${Math.random().toString(36).substring(7)}`;
        const newAccount: NewAccount = {
          id: accountId,
          userId: user.id,
          type: 'oauth',
          provider: 'google',
          providerAccountId: googleId,
          access_token: '',
          refresh_token: null,
          expires_at: null,
          token_type: null,
          scope: null,
          id_token: null,
          session_state: null,
        };
        await db.insert(accounts).values(newAccount);
      }

      // 3. Generate Session & JWT
      // Calculate expiry
      const tokenExpiry = 30 * 24 * 3600; // 30 days
      const refreshExpiresAt = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000);

      // Generate Access Token (JWT)
      const accessToken = await signJWT(
        {
          user_id: user.id,
          email: user.email || '',
          name: user.name || '',
        },
        tokenExpiry,
      );

      // Generate Refresh Token (Session)
      const sessionToken = `session_${user.id}_${Date.now()}`;

      const newSession: NewSession = {
        id: `sess_${Date.now()}_${user.id}`,
        userId: user.id,
        sessionToken: sessionToken,
        expires: refreshExpiresAt,
      };

      await db.insert(sessions).values(newSession);

      return {
        success: true,
        message: 'Google login successful',
        user: {
          id: user.id,
          name: user.name,
          email: user.email || '',
          role: user.role,
          image: user.image,
          emailVerified: user.emailVerified,
        },
        accessToken,
        refreshToken: sessionToken,
        tokenType: 'Bearer',
        expiresIn: tokenExpiry,
      };
    } catch (error) {
      console.error('Google Auth Error:', error);
      set.status = 401;
      return {
        success: false,
        message: 'Invalid Google token',
        error: String(error),
      };
    }
  },
  {
    body: t.Object({
      idToken: t.String(),
    }),
  },
);
