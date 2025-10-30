import logger from '../utils/unified-logger';

interface CustomError extends Error {
  code?: string;
}

// Type definition for Redis client
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type RedisClientType = any;

// Conditional Redis client initialization
let redis: RedisClientType | null = null;

if (typeof window === 'undefined') {
  // Only initialize Redis in Node.js environment
  try {
    const redisModule = require('redis');
    const { createClient } = redisModule;

    const redisHost = process.env.REDIS_HOST || '127.0.0.1';
    const redisPort = parseInt(process.env.REDIS_PORT || '6379');
    const redisPassword = process.env.REDIS_PASSWORD || '';

    redis = createClient({
      socket: {
        host: redisHost,
        port: redisPort,
      },
      password: redisPassword || undefined,
    });

    // Handle connection events
    redis.on('error', (err: Error) => {
      logger.error('[Redis] Connection error:', err);
    });

    redis.on('connect', () => {
      logger.info('[Redis] Connected successfully');
    });

    redis.on('ready', () => {
      logger.info('[Redis] Client ready');
    });

    redis.on('end', () => {
      logger.info('[Redis] Connection ended');
    });

    // Connect to Redis
    redis.connect().catch((err: Error) => {
      logger.error('[Redis] Failed to connect:', err);
    });

    logger.info('[Redis] Redis client initialized');
  } catch (error) {
    logger.warn('[Redis] Redis module not available:', error);
    redis = null;
  }
} else {
  logger.info('[Redis] Redis client not initialized (browser environment)');
}

// Export a mock Redis client for browser environment
export { redis };

// Test Redis connection
export async function testRedisConnection(): Promise<boolean> {
  if (!redis) {
    logger.warn('[Redis] Redis client not available for testing');
    return false;
  }

  try {
    const redisHost = process.env.REDIS_HOST || '127.0.0.1';
    const redisPort = parseInt(process.env.REDIS_PORT || '6379');
    const redisPassword = process.env.REDIS_PASSWORD || '';

    logger.info('[Redis] Testing connection with config:', {
      host: redisHost,
      port: redisPort,
      hasPassword: !!redisPassword,
    });

    await redis.set('test-connection', 'ok', { EX: 10 });
    const result = await redis.get('test-connection');
    if (result === 'ok') {
      logger.info('[Redis] Connection test successful');
      return true;
    } else {
      logger.error(
        '[Redis] Connection test failed - unexpected result:',
        result,
      );
      return false;
    }
  } catch (error: unknown) {
    const err = error as CustomError;
    const redisHost = process.env.REDIS_HOST || '127.0.0.1';
    const redisPort = parseInt(process.env.REDIS_PORT || '6379');
    const redisPassword = process.env.REDIS_PASSWORD || '';

    logger.error('[Redis] Connection test failed:', {
      message: err.message,
      code: err.code,
      stack: err.stack,
      host: redisHost,
      port: redisPort,
      hasPassword: !!redisPassword,
    });
    return false;
  }
}
