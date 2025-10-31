import { Elysia } from 'elysia';
import { cors } from '@elysiajs/cors';
import { jwt } from '@elysiajs/jwt';
import { apiRoutes } from './routes/api';
import { authRoutes } from './routes/auth';
import { logger } from './middleware';
import { config } from './config';
import { getDatabase } from './utils/database';
import { getRedis } from './utils/redis';

// Initialize database and Redis connections
const initializeConnections = async () => {
  try {
    if (config.databaseUrl) {
      await getDatabase();
    }

    const redis = getRedis();
    await redis.connect();
  } catch (error) {
    console.error('Failed to initialize connections:', error);
  }
};

export const app = new Elysia()
  .use(cors())
  .use(
    jwt({
      name: 'jwt',
      secret: config.jwtSecret,
    })
  )
  .use(logger)
  .get('/', () => ({
    message: 'Welcome to ElysiaJS Auth API',
    version: '1.0.0',
    endpoints: {
      auth: '/api/auth',
      health: '/health',
    },
  }))
  .get('/health', () => ({
    status: 'ok',
    timestamp: new Date().toISOString(),
    environment: config.env,
    database: config.databaseUrl ? 'connected' : 'not configured',
  }))
  .get('/api/hello/:name', ({ params: { name } }) => ({
    message: `Hello ${name}!`,
  }))
  .post('/api/echo', ({ body }) => ({
    echo: body,
  }))
  .use(authRoutes)
  .use(apiRoutes)
  .onError(({ error, set }) => {
    console.error('Error:', error);
    set.status = 500;
    return {
      success: false,
      error: error.message || 'Internal server error',
    };
  });

// Start the server
initializeConnections().then(() => {
  app.listen({
    port: config.port,
    hostname: '0.0.0.0',
  });

  console.log(
    `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
  );
  console.log(`📝 Environment: ${config.env}`);
  console.log(`🔐 Auth endpoints: http://${app.server?.hostname}:${app.server?.port}/api/auth`);
});
