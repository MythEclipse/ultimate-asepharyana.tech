import Elysia from 'elysia';
import cors from '@elysiajs/cors';
import jwt from '@elysiajs/jwt';
import { swagger } from '@elysiajs/swagger';
import { apiRoutes } from './routes/api';
import { authRoutes } from './routes/auth';
import { sosmedRoutes } from './routes/sosmed';
import { chatRoutes } from './routes/chat';
import { logger } from './middleware';
import { errorHandler } from './middleware/errorHandler';
import { rateLimit } from './middleware/rateLimit';
import { config } from './config';
import { connectDatabase, prisma } from './utils/prisma';
import { getRedis } from './utils/redis';

// Initialize database and Redis connections
const initializeConnections = async () => {
  try {
    // Connect to Prisma database
    await connectDatabase();

    // Connect to Redis
    const redis = getRedis();
    await redis.connect();
  } catch (error) {
    console.error('Failed to initialize connections:', error);
  }
};

// Graceful shutdown
process.on('SIGINT', async () => {
  console.log('\n🛑 Shutting down gracefully...');
  await prisma.$disconnect();
  process.exit(0);
});

export const app = new Elysia()
  .use(errorHandler) // Global error handling
  .use(
    rateLimit({
      // Global rate limit: 100 requests per minute
      max: 100,
      window: 60 * 1000,
    })
  )
  .use(cors())
  .use(
    swagger({
      path: '/docs',
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
          { name: 'Social Media', description: 'Social media posts, comments, and likes' },
          { name: 'Chat', description: 'Chat rooms and messages' },
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
  .use(sosmedRoutes)
  .use(chatRoutes);

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
  console.log(`📚 Swagger docs: http://${app.server?.hostname}:${app.server?.port}/docs`);
});
