import { Redis } from '@upstash/redis';
import logger from '../utils/logger';

if (!process.env.UPSTASH_REDIS_REST_URL || !process.env.UPSTASH_REDIS_REST_TOKEN) {
  throw new Error('UPSTASH_REDIS_REST_URL and UPSTASH_REDIS_REST_TOKEN must be set');
}

export const redis = new Redis({
  url: process.env.UPSTASH_REDIS_REST_URL,
  token: process.env.UPSTASH_REDIS_REST_TOKEN,
});

// Test Redis connection
export async function testRedisConnection(): Promise<boolean> {
  try {
    logger.info('[Redis] Testing connection with config:', {
      url: process.env.UPSTASH_REDIS_REST_URL,
      hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
      tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length
    });

    await redis.set('test-connection', 'ok', { ex: 10 });
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
      url: process.env.UPSTASH_REDIS_REST_URL,
      hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
      tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length
    });
    return false;
  }
}
