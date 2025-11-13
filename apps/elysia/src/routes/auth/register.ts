import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { getDb, users, emailVerificationTokens } from '@asepharyana/services';
import type { NewUser, NewEmailVerificationToken } from '@asepharyana/services';
import { eq } from '@asepharyana/services';
import { sendVerificationEmail } from '../../utils/email';
import { rateLimit } from '../../middleware/rateLimit';
import { sanitizeEmail, sanitizeString } from '../../utils/validation';

function generateToken(): string {
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);
  return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');
}

interface RegisterBody {
  email: string;
  name?: string;
  password: string;
}

export interface RegisterResponse {
  success: boolean;
  message: string;
  user: {
    id: string;
    email: string;
    name: string | null;
    emailVerified: Date | null;
  };
}

function validatePassword(password: string): string | null {
  if (password.length < 8) {
    return 'Password must be at least 8 characters';
  }

  const hasUppercase = /[A-Z]/.test(password);
  const hasLowercase = /[a-z]/.test(password);
  const hasDigit = /\d/.test(password);
  const hasSpecial = /[^A-Za-z0-9]/.test(password);

  if (!hasUppercase || !hasLowercase || !hasDigit) {
    return 'Password must contain uppercase, lowercase, and numbers';
  }

  if (!hasSpecial) {
    return 'Password should contain at least one special character';
  }

  return null;
}

export const registerRoute = new Elysia()
  .use(
    rateLimit({
      max: 10,
      window: 60 * 60 * 1000,
      message: 'Too many registration attempts, please try again later',
    })
  )
  .post(
    '/register',
    async ({ body, set }): Promise<RegisterResponse> => {
      const db = getDb();
      const { email, name, password } = body as RegisterBody;

      const sanitizedEmail = sanitizeEmail(email);
      if (!sanitizedEmail) {
        set.status = 400;
        throw new Error('Invalid email format');
      }

      const sanitizedName = name ? sanitizeString(name) : null;

      const passwordError = validatePassword(password);
      if (passwordError) {
        set.status = 400;
        throw new Error(passwordError);
      }

      const existingUserResult = await db
        .select()
        .from(users)
        .where(eq(users.email, sanitizedEmail))
        .limit(1);

      if (existingUserResult.length > 0) {
        set.status = 400;
        throw new Error('Email already exists');
      }

      const hashedPassword = await bcrypt.hash(password, 10);
      const userId = `user_${Date.now()}_${Math.random().toString(36).substring(7)}`;

      const newUser: NewUser = {
        id: userId,
        email: sanitizedEmail,
        name: sanitizedName,
        password: hashedPassword,
        emailVerified: null,
        image: null,
        refreshToken: null,
        role: 'user',
      };

      await db.insert(users).values(newUser);

      const verificationToken = generateToken();
      const expiresAt = new Date(Date.now() + 24 * 60 * 60 * 1000);

      const newToken: NewEmailVerificationToken = {
        id: `evt_${Date.now()}_${userId}`,
        userId: userId,
        token: verificationToken,
        expiresAt: expiresAt,
      };

      await db.insert(emailVerificationTokens).values(newToken);

      try {
        await sendVerificationEmail(email, name || 'User', verificationToken);
      } catch (error) {
        console.error('Failed to send verification email:', error);
      }

      return {
        success: true,
        message: 'User registered successfully. Please check your email to verify your account.',
        user: {
          id: userId,
          email: sanitizedEmail,
          name: sanitizedName,
          emailVerified: null,
        },
      };
    },
    {
      body: t.Object({
        email: t.String({ format: 'email' }),
        password: t.String({ minLength: 8 }),
        name: t.Optional(t.String()),
      }),
    }
  );
