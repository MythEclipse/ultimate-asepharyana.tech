import Elysia from 'elysia';
import cors from '@elysiajs/cors';
import jwt from '@elysiajs/jwt';
import { swagger } from '@elysiajs/swagger';
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
    swagger({
      documentation: {
        info: {
          title: 'ElysiaJS Auth API Documentation',
          version: '1.0.0',
          description: 'API documentation for ElysiaJS authentication service with Redis caching',
        },
        tags: [
          { name: 'Health', description: 'Health check endpoints' },
          { name: 'Auth', description: 'Authentication endpoints' },
          { name: 'API', description: 'General API endpoints' },
        ],
        servers: [
          {
            url: `http://localhost:${config.port}`,
            description: 'Development server',
          },
        ],
      },
    })
  )
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

    // Handle different error types
    const errorMessage =
      error instanceof Error
        ? error.message
        : typeof error === 'string'
        ? error
        : 'Internal server error';

    return {
      success: false,
      error: errorMessage,
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
  console.log(`📚 Swagger docs: http://${app.server?.hostname}:${app.server?.port}/swagger`);
});
