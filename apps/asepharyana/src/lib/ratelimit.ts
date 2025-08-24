import { Ratelimit } from '@upstash/ratelimit';
import { redis } from './redis';

// Ensure Upstash env vars are set in redis.ts

export const ratelimit = new Ratelimit({
  redis,
  limiter: Ratelimit.slidingWindow(5, '10 s'),
  analytics: true,
});
