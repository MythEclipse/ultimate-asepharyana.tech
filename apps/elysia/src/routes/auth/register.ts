import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { v4 as uuidv4 } from 'uuid';
import { getDatabase } from '../../utils/database';
import { sendVerificationEmail } from '../../utils/email';
import { toUserResponse, type User, type RegisterResponse } from '../../models/user';
import type { RowDataPacket } from 'mysql2';

interface RegisterBody {
  email: string;
  username: string;
  password: string;
  full_name?: string;
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
    '/api/auth/register',
    async ({ body, set }): Promise<RegisterResponse> => {
      const { email, username, password, full_name } = body as RegisterBody;

      // Validate password strength
      const passwordError = validatePassword(password);
      if (passwordError) {
        set.status = 400;
        throw new Error(passwordError);
      }

      const db = await getDatabase();

      // Check if email exists
      const [emailCheck] = await db.query<RowDataPacket[]>(
        'SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) as exists',
        [email]
      );
      if (emailCheck[0].exists) {
        set.status = 400;
        throw new Error('Email already exists');
      }

      // Check if username exists
      const [usernameCheck] = await db.query<RowDataPacket[]>(
        'SELECT EXISTS(SELECT 1 FROM users WHERE username = ?) as exists',
        [username]
      );
      if (usernameCheck[0].exists) {
        set.status = 400;
        throw new Error('Username already exists');
      }

      // Hash password
      const password_hash = await bcrypt.hash(password, 10);

      // Generate user ID
      const user_id = uuidv4();
      const now = new Date();

      // Insert user
      await db.query(
        `INSERT INTO users (
          id, email, username, password_hash, full_name,
          email_verified, is_active, role, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
        [user_id, email, username, password_hash, full_name || null, false, true, 'user', now, now]
      );

      // Generate verification token
      const verification_token = uuidv4();
      const expires_at = new Date(Date.now() + 24 * 60 * 60 * 1000); // 24 hours

      await db.query(
        `INSERT INTO email_verification_tokens (id, user_id, token, expires_at, created_at)
         VALUES (?, ?, ?, ?, ?)`,
        [uuidv4(), user_id, verification_token, expires_at, now]
      );

      // Send verification email
      try {
        await sendVerificationEmail(email, username, verification_token);
      } catch (error) {
        console.error('Failed to send verification email:', error);
      }

      // Fetch created user
      const [users] = await db.query<(User & RowDataPacket)[]>(
        `SELECT id, email, username, password_hash, full_name, avatar_url,
                email_verified, is_active, role, last_login_at, created_at, updated_at
         FROM users WHERE id = ?`,
        [user_id]
      );

      const user = users[0];

      return {
        success: true,
        message: 'User registered successfully. Please check your email to verify your account.',
        user: toUserResponse(user),
        verification_token: verification_token,
      };
    },
    {
      body: t.Object({
        email: t.String({ format: 'email' }),
        username: t.String({ minLength: 3, maxLength: 50 }),
        password: t.String({ minLength: 8 }),
        full_name: t.Optional(t.String()),
      }),
    }
  );
