import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { prisma } from '../../utils/prisma';
import { sendVerificationEmail } from '../../utils/email';

// Generate secure random token
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
    isVerified: boolean;
    createdAt: Date;
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
  .post(
    '/register',
    async ({ body, set }): Promise<RegisterResponse> => {
      const { email, name, password } = body as RegisterBody;

      // Validate password strength
      const passwordError = validatePassword(password);
      if (passwordError) {
        set.status = 400;
        throw new Error(passwordError);
      }

      // Check if email exists
      const existingUser = await prisma.user.findUnique({
        where: { email },
      });

      if (existingUser) {
        set.status = 400;
        throw new Error('Email already exists');
      }

      // Hash password
      const hashedPassword = await bcrypt.hash(password, 10);

      // Create user
      const user = await prisma.user.create({
        data: {
          email,
          name: name || null,
          password: hashedPassword,
          isVerified: false,
        },
      });

      // Generate verification token
      const verificationToken = generateToken();
      const expiresAt = new Date(Date.now() + 24 * 60 * 60 * 1000); // 24 hours

      // Create email verification token
      await prisma.emailVerificationToken.create({
        data: {
          userId: user.id,
          token: verificationToken,
          expiresAt,
        },
      });

      // Send verification email
      try {
        await sendVerificationEmail(email, name || 'User', verificationToken);
      } catch (error) {
        console.error('Failed to send verification email:', error);
      }

      return {
        success: true,
        message: 'User registered successfully. Please check your email to verify your account.',
        user: {
          id: user.id,
          email: user.email,
          name: user.name,
          isVerified: user.isVerified,
          createdAt: user.createdAt,
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
