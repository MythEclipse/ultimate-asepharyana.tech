import { Elysia } from 'elysia';
import { registerRoute } from './register';
import { loginRoute } from './login';
import { logoutRoute } from './logout';
import { meRoute } from './me';
import { verifyRoute } from './verify';
import { forgotPasswordRoute } from './forgot-password';
import { resetPasswordRoute } from './reset-password';
import { refreshTokenRoute } from './refresh-token';
import { googleAuth } from './google';

export const authRoutes = new Elysia({ prefix: '/api/auth' })
  .use(registerRoute)
  .use(loginRoute)
  .use(googleAuth)
  .use(logoutRoute)
  .use(meRoute)
  .use(verifyRoute)
  .use(forgotPasswordRoute)
  .use(resetPasswordRoute)
  .use(refreshTokenRoute);
