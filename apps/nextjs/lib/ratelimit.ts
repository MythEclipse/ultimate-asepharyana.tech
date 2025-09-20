import { redis } from './redis';

// Simple rate limiter implementation for regular Redis
export class SimpleRateLimit {
  private redis: typeof redis;
  private windowMs: number;
  private maxRequests: number;

  constructor(redisClient: typeof redis, maxRequests = 5, windowMs = 10000) {
    this.redis = redisClient;
    this.maxRequests = maxRequests;
    this.windowMs = windowMs;
  }

  async check(
    identifier: string,
  ): Promise<{ success: boolean; remaining: number; resetTime: number }> {
    const key = `ratelimit:${identifier}`;
    const now = Date.now();
    const windowStart = now - this.windowMs;

    try {
      // Remove old entries and add new one atomically
      const multi = this.redis.multi();
      multi.zRemRangeByScore(key, 0, windowStart);
      multi.zAdd(key, { score: now, value: now.toString() });
      multi.zCard(key);
      multi.pExpire(key, this.windowMs);

      const results = await multi.exec();
      const requestCount = results ? Number(results[2]) : 0;

      const remaining = Math.max(0, this.maxRequests - requestCount);
      const success = requestCount <= this.maxRequests;

      return {
        success,
        remaining,
        resetTime: now + this.windowMs,
      };
    } catch (error) {
      // If Redis fails, allow the request (fail open)
      console.warn('Rate limit check failed, allowing request:', error);
      return {
        success: true,
        remaining: this.maxRequests - 1,
        resetTime: now + this.windowMs,
      };
    }
  }
  // Bulk rate limit check for multiple identifiers
  async checkBulk(
    identifiers: string[],
  ): Promise<
    Record<
      string,
      { success: boolean; remaining: number; resetTime: number }
    >
  > {
    const now = Date.now();
    const windowStart = now - this.windowMs;
    const results: Record<
      string,
      { success: boolean; remaining: number; resetTime: number }
    > = {};

    try {
      const multi = this.redis.multi();
      identifiers.forEach((identifier) => {
        const key = `ratelimit:${identifier}`;
        multi.zRemRangeByScore(key, 0, windowStart);
        multi.zAdd(key, { score: now, value: now.toString() });
        multi.zCard(key);
        multi.pExpire(key, this.windowMs);
      });
      const execResults = await multi.exec();
      for (let i = 0; i < identifiers.length; i++) {
        const requestCount = execResults ? Number(execResults[i * 4 + 2]) : 0;
        const remaining = Math.max(0, this.maxRequests - requestCount);
        const success = requestCount <= this.maxRequests;
        results[identifiers[i]] = {
          success,
          remaining,
          resetTime: now + this.windowMs,
        };
      }
      return results;
    } catch (error) {
      identifiers.forEach((identifier) => {
        results[identifier] = {
          success: true,
          remaining: this.maxRequests - 1,
          resetTime: now + this.windowMs,
        };
      });
      return results;
    }
  }
}

export const ratelimit = new SimpleRateLimit(redis, 5, 10000); // 5 requests per 10 seconds
