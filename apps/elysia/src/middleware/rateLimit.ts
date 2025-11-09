import { Elysia } from 'elysia';

interface RateLimitOptions {
  max: number;
  window: number; // in milliseconds
  message?: string;
}

const rateLimitStore = new Map<string, { count: number; resetTime: number }>();

/**
 * Rate limiting middleware
 * @param options - Configuration options
 * @param options.max - Maximum number of requests per window
 * @param options.window - Time window in milliseconds
 * @param options.message - Custom error message
 */
export const rateLimit = ({ max, window, message }: RateLimitOptions) => {
  return new Elysia().derive(async ({ request, set }) => {
    const ip = request.headers.get('x-forwarded-for') ||
               request.headers.get('x-real-ip') ||
               'unknown';

    const key = `ratelimit:${ip}`;
    const now = Date.now();

    const record = rateLimitStore.get(key);

    if (!record || now > record.resetTime) {
      rateLimitStore.set(key, {
        count: 1,
        resetTime: now + window,
      });
      return {};
    }

    if (record.count >= max) {
      set.status = 429;
      throw new Error(message || 'Too many requests, please try again later');
    }

    record.count++;
    return {};
  });
};

/**
 * Cleanup old rate limit records
 * Should be called periodically (e.g., every 5 minutes)
 */
export const cleanupRateLimitStore = () => {
  const now = Date.now();
  for (const [key, record] of rateLimitStore.entries()) {
    if (now > record.resetTime) {
      rateLimitStore.delete(key);
    }
  }
};

// Cleanup every 5 minutes
setInterval(cleanupRateLimitStore, 5 * 60 * 1000);
