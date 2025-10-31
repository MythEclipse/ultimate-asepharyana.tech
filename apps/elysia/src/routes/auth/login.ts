import { Elysia, t } from 'elysia';
import bcrypt from 'bcryptjs';
import { v4 as uuidv4 } from 'uuid';
import { getDatabase } from '../../utils/database';
import { signJWT } from '../../utils/jwt';
import { toUserResponse, type User, type LoginResponse } from '../../models/user';
import type { RowDataPacket } from 'mysql2';

interface LoginBody {
  login: string;
  password: string;
  remember_me?: boolean;
}

export const loginRoute = new Elysia()
  .post(
    '/api/auth/login',
    async ({ body, set }): Promise<LoginResponse> => {
      const { login, password, remember_me } = body as LoginBody;

      const db = await getDatabase();

      // Find user by email or username
      const [users] = await db.query<(User & RowDataPacket)[]>(
        `SELECT id, email, username, password_hash, full_name, avatar_url,
                email_verified, is_active, role, last_login_at, created_at, updated_at
         FROM users
         WHERE email = ? OR username = ?`,
        [login, login]
      );

      if (users.length === 0) {
        // Log failed login attempt
        await logLoginAttempt(db, null, false, 'User not found');
        set.status = 401;
        throw new Error('Invalid credentials');
      }

      const user = users[0];

      // Verify password
      const passwordValid = await bcrypt.compare(password, user.password_hash);
      if (!passwordValid) {
        await logLoginAttempt(db, user.id, false, 'Invalid password');
        set.status = 401;
        throw new Error('Invalid credentials');
      }

      // Check if account is active
      if (!user.is_active) {
        set.status = 403;
        throw new Error('Account is inactive');
      }

      // Generate JWT tokens
      const token_expiry = remember_me ? 30 * 24 * 3600 : 24 * 3600; // 30 days or 24 hours

      const access_token = await signJWT({
        user_id: user.id,
        email: user.email,
        name: user.username,
      }, token_expiry);

      // Generate refresh token
      const refresh_token = uuidv4();
      const refresh_expires_at = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000); // 30 days

      // Store refresh token
      await db.query(
        `INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
         VALUES (?, ?, ?, ?, ?)`,
        [uuidv4(), user.id, refresh_token, refresh_expires_at, new Date()]
      );

      // Update last login
      await db.query(
        'UPDATE users SET last_login_at = ? WHERE id = ?',
        [new Date(), user.id]
      );

      // Log successful login
      await logLoginAttempt(db, user.id, true, null);

      return {
        user: toUserResponse(user),
        access_token,
        refresh_token,
        token_type: 'Bearer',
        expires_in: token_expiry,
      };
    },
    {
      body: t.Object({
        login: t.String(),
        password: t.String(),
        remember_me: t.Optional(t.Boolean()),
      }),
    }
  );

async function logLoginAttempt(
  db: Awaited<ReturnType<typeof getDatabase>>,
  user_id: string | null,
  success: boolean,
  failure_reason: string | null
): Promise<void> {
  if (!user_id) return;

  await db.query(
    `INSERT INTO login_history (id, user_id, success, failure_reason, created_at)
     VALUES (?, ?, ?, ?, ?)`,
    [uuidv4(), user_id, success, failure_reason, new Date()]
  );
}
