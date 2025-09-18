import { createClient, RedisClientType } from 'redis';
import logger from '../utils/logger';

const redisHost = process.env.REDIS_HOST || '127.0.0.1';
const redisPort = parseInt(process.env.REDIS_PORT || '6379');
const redisPassword = process.env.REDIS_PASSWORD || '';

export const redis: RedisClientType = createClient({
  socket: {
    host: redisHost,
    port: redisPort,
  },
  password: redisPassword || undefined,
});

// Handle connection events
redis.on('error', (err) => {
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
redis.connect().catch((err) => {
  logger.error('[Redis] Failed to connect:', err);
});

// Test Redis connection
export async function testRedisConnection(): Promise<boolean> {
  try {
    logger.info('[Redis] Testing connection with config:', {
      host: redisHost,
      port: redisPort,
      hasPassword: !!redisPassword
    });

    await redis.set('test-connection', 'ok', { EX: 10 });
    const result = await redis.get('test-connection');
    if (result === 'ok') {
      logger.info('[Redis] Connection test successful');
      return true;
    } else {
      logger.error('[Redis] Connection test failed - unexpected result:', result);
      return false;
    }
  } catch (error: any) {
    logger.error('[Redis] Connection test failed:', {
      message: error.message,
      code: error.code,
      stack: error.stack,
      host: redisHost,
      port: redisPort,
      hasPassword: !!redisPassword
    });
    return false;
  }
}
