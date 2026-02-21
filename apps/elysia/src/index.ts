import Elysia from 'elysia';
import cors from '@elysiajs/cors';
import jwt from '@elysiajs/jwt';
import { swagger } from '@elysiajs/swagger';
import { apiRoutes } from './routes/api';
import { authRoutes } from './routes/auth';
import { sosmedRoutes } from './routes/sosmed';
import { chatRoutes } from './routes/chat';
import { quizBattleWS } from './routes/quiz-battle';
import { userAvatarRoutes } from './routes/user-avatar';
import { imageCacheRoutes } from './routes/image-cache';
import { historyRoutes } from './routes/history';
import { logger } from './middleware';
import { errorHandler } from './middleware/errorHandler';
import { rateLimit } from './middleware/rateLimit';
import { config } from './config';
import { initializeDb, closeDb } from './services';
import { getRedis } from './utils/redis';

let isDbInitialized = false;

// Initialize database and Redis connections
const initializeConnections = async () => {
  try {
    // Connect to database
    if (!isDbInitialized) {
      initializeDb(config.databaseUrl);
      isDbInitialized = true;
      console.log('✅ Database connected successfully');
    }

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
  await closeDb();
  process.exit(0);
});

export const app = new Elysia()
  .use(errorHandler) // Global error handling
  .use(
    rateLimit({
      // Global rate limit: 100 requests per minute
      max: 100,
      window: 60 * 1000,
    }),
  )
  .use(
    cors({
      origin: true,
      methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS', 'PATCH'],
      allowedHeaders: ['Content-Type', 'Authorization', 'X-Requested-With'],
      credentials: true,
    }),
  )
  .use(
    swagger({
      path: '/docs',
      documentation: {
        info: {
          title: 'ElysiaJS Auth API Documentation',
          version: '1.0.0',
          description:
            'API documentation for ElysiaJS authentication service with Redis caching',
        },
        tags: [
          { name: 'Health', description: 'Health check endpoints' },
          { name: 'Auth', description: 'Authentication endpoints' },
          { name: 'API', description: 'General API endpoints' },
          {
            name: 'Social Media',
            description: 'Social media posts, comments, and likes',
          },
          { name: 'Chat', description: 'Chat rooms and messages' },
          {
            name: 'Quiz Battle',
            description: 'Quiz Battle game WebSocket and REST endpoints',
          },
        ],
        servers: [
          {
            url: 'https://elysia.asepharyana.tech',
            description: 'Production server',
          },
          {
            url: `http://localhost:${config.port}`,
            description: 'Development server',
          },
        ],
        components: {
          securitySchemes: {
            bearerAuth: {
              type: 'http',
              scheme: 'bearer',
              bearerFormat: 'JWT',
              description: 'Enter your JWT token',
            },
          },
        },
        security: [
          {
            bearerAuth: [],
          },
        ],
      },
    }),
  )
  .use(
    jwt({
      name: 'jwt',
      secret: config.jwtSecret,
    }),
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
  .use(userAvatarRoutes)
  .use(sosmedRoutes)
  .use(chatRoutes)
  .use(imageCacheRoutes)
  .use(quizBattleWS) // Added quizBattleWS
  .use(historyRoutes) // Added historyRoutes
  // Serve AsyncAPI YAML
  .get('/docs-ws/asyncapi.yaml', () => {
    return Bun.file('./docs/asyncapi/asyncapi.yaml');
  })
  // Serve AsyncAPI Viewer
  .get('/docs-ws', () => {
    return new Response(
      `
      <!DOCTYPE html>
      <html>
        <head>
          <title>AsyncAPI Documentation</title>
          <link rel="stylesheet" href="https://unpkg.com/@asyncapi/react-component@latest/styles/default.min.css">
        </head>
        <body>
          <div id="asyncapi"></div>
          <script src="https://unpkg.com/@asyncapi/react-component@latest/browser/standalone.js"></script>
          <script>
            AsyncApiStandalone.render({
              schema: {
                url: '/docs-ws/asyncapi.yaml',
                refParser: {
                  mode: 'dereference'
                }
              },
              config: {
                show: {
                  sidebar: true,
                }
              }
            }, document.getElementById('asyncapi'));
          </script>
        </body>
      </html>
      `,
      {
        headers: {
          'Content-Type': 'text/html',
        },
      },
    );
  });

// Graceful shutdown handler
process.on('SIGTERM', async () => {
  console.log('SIGTERM received, closing gracefully...');
  try {
    await closeDb();
    process.exit(0);
  } catch (error) {
    console.error('Error during shutdown:', error);
    process.exit(1);
  }
});

// Start the server
initializeConnections().then(() => {
  app.listen({
    port: config.port,
    hostname: '127.0.0.1',
  });

  console.log(
    `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`,
  );
  console.log(`📝 Environment: ${config.env}`);
  console.log(
    `🔐 Auth endpoints: http://${app.server?.hostname}:${app.server?.port}/api/auth`,
  );
  console.log(
    `🎮 Quiz Battle WS: ws://${app.server?.hostname}:${app.server?.port}/api/quiz/battle`,
  );
  console.log(
    `📚 Swagger docs: http://${app.server?.hostname}:${app.server?.port}/docs`,
  );
  console.log(
    `📚 AsyncAPI docs: http://${app.server?.hostname}:${app.server?.port}/docs-ws`,
  );
});
