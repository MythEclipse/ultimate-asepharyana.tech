import axios, { AxiosResponse } from 'axios';
import logger from './unified-logger';
import { DEFAULT_HEADERS } from './DHead';
import { scrapeCroxyProxy } from '../lib/scrapeCroxyProxy';
import { redis } from '../lib/redis';
import { HttpClientConfig, FetchResult } from '../types/http';
import { AppError } from '../types/error';
import {
  createNetworkError,
  createHttpError,
  toAppError,
  logError,
} from './error-handler';

export interface CustomError extends Error {
  code?: string;
}

/**
 * ProxyHttpClient - HTTP client with proxy capabilities for bypassing network restrictions
 *
 * This class provides proxy functionality for server-side use only, including:
 * - Direct HTTP requests with Axios
 * - Fallback to Croxy Proxy for blocked content
 * - Redis caching for improved performance
 * - Internet Baik block page detection
 */
export class ProxyHttpClient {
  private config: HttpClientConfig;
  private proxyConfig: HttpClientConfig['proxy'];

  constructor(config: HttpClientConfig = {}) {
    this.config = {
      timeout: 10000,
      ...config,
    };
    this.proxyConfig = config.proxy || { enabled: true, fallback: true };
  }

  /**
   * Check if the response data contains Internet Baik block page indicators
   */
  private isInternetBaikBlockPage(data: string | object): boolean {
    if (typeof data !== 'string') return false;
    return (
      data.includes('internetbaik.telkomsel.com') ||
      data.includes('VmaxAdManager.js') ||
      data.includes('VmaxAdHelper')
    );
  }

  /**
   * Get cache key for Redis storage
   */
  private getFetchCacheKey(slug: string): string {
    return `fetch:proxy:${slug}`;
  }

  /**
   * Get cached fetch result from Redis
   */
  private async getCachedFetch(slug: string): Promise<FetchResult | null> {
    try {
      const key = this.getFetchCacheKey(slug);
      const cached = await redis.get(key);
      if (cached) {
        try {
          const parsed = JSON.parse(
            typeof cached === 'string' ? cached : JSON.stringify(cached),
          );
          logger.info(
            `[ProxyHttpClient] Returning cached response for ${slug}`,
          );
          return parsed;
        } catch {
          // ignore parse error, fallback to fetch
        }
      }
    } catch (redisError: unknown) {
      const networkError = createNetworkError('Redis connection failed', {
        originalError: redisError,
        context: {
          operation: 'getCachedFetch',
          slug,
          url: process.env.UPSTASH_REDIS_REST_URL,
          hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
          tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length,
        },
      });
      logger.warn(
        `[ProxyHttpClient] Redis get failed for ${slug}:`,
        networkError,
      );
      // ignore Redis error, fallback to fetch
    }
    return null;
  }

  /**
   * Cache fetch result in Redis
   */
  private async setCachedFetch(slug: string, value: FetchResult) {
    try {
      const key = this.getFetchCacheKey(slug);
      await redis.set(key, JSON.stringify(value), {
        EX: this.config.cache?.ttl || 120,
      });
    } catch (redisError) {
      const networkError = createNetworkError('Redis connection failed', {
        originalError: redisError,
        context: {
          operation: 'setCachedFetch',
          slug,
          url: process.env.UPSTASH_REDIS_REST_URL,
          hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
          tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length,
        },
      });
      logger.warn(
        `[ProxyHttpClient] Redis set failed for ${slug}:`,
        networkError,
      );
      // ignore Redis error, continue without caching
    }
  }

  /**
   * Handle Axios response and check for blocked content
   */
  private async handleAxiosResponse(
    slug: string,
    res: AxiosResponse,
    source: string,
  ): Promise<FetchResult> {
    logger.info(`[ProxyHttpClient] ${source} fetch response:`, {
      url: slug,
      status: res.status,
      headers: res.headers,
    });

    if (res.status >= 200 && res.status < 300) {
      logger.info(
        `[ProxyHttpClient] ${source} fetch successful for ${slug}, status: ${res.status}`,
      );
      const contentType =
        res.headers['content-type'] || res.headers['Content-Type'];
      let data = res.data;

      if (contentType?.includes('application/json')) {
        data = JSON.stringify(data); // Convert to string for isInternetBaikBlockPage check
      }

      if (this.isInternetBaikBlockPage(data)) {
        logger.warn(
          `Blocked by internetbaik (${source} fetch), trying scrapeCroxyProxy`,
        );
        const croxyResult = await this.attemptCroxyProxyFetch(slug);
        await this.setCachedFetch(slug, croxyResult);
        return croxyResult;
      }

      const result = { data: res.data, contentType };
      await this.setCachedFetch(slug, result);
      return result;
    }

    throw createHttpError(
      `${source} fetch failed with status ${res.status}`,
      res.status,
      {
        statusText: res.statusText,
        url: slug,
        context: { source },
      },
    );
  }

  /**
   * Attempt direct Axios fetch
   */
  private async attemptAxiosFetch(slug: string): Promise<FetchResult> {
    const res: AxiosResponse = await axios.get(slug, {
      headers: DEFAULT_HEADERS,
      timeout: this.config.timeout || 10000,
    });
    return this.handleAxiosResponse(slug, res, 'Direct axios');
  }

  /**
   * Attempt Croxy Proxy fetch
   */
  async attemptCroxyProxyFetch(slug: string): Promise<FetchResult> {
    logger.info(`[ProxyHttpClient] Using scrapeCroxyProxy for ${slug}`);
    const html = await scrapeCroxyProxy(slug);
    const result = { data: html, contentType: 'text/html' };
    await this.setCachedFetch(slug, result);
    return result;
  }

  /**
   * Fetch with proxy support (try direct first, fallback to proxy)
   */
  async fetchWithProxy(slug: string): Promise<FetchResult> {
    const cached = await this.getCachedFetch(slug);
    if (cached) return cached;

    let directError: AppError | undefined;
    try {
      return await this.attemptAxiosFetch(slug);
    } catch (error: unknown) {
      directError = toAppError(error, {
        url: slug,
        method: 'attemptAxiosFetch',
      });
      logError(directError);
      logger.warn(`Direct axios fetch failed for ${slug}:`, directError);
      logger.error('Direct axios fetch failed, trying scrapeCroxyProxy');
    }

    try {
      return await this.attemptCroxyProxyFetch(slug);
    } catch (croxyError: unknown) {
      const croxyAppError = toAppError(croxyError, {
        url: slug,
        method: 'attemptCroxyProxyFetch',
      });
      logError(croxyAppError);

      const combinedError = createNetworkError(
        `Both direct axios fetch and proxy failed for ${slug}`,
        {
          originalError: croxyError,
          context: {
            directError: directError?.message,
            croxyError: croxyAppError.message,
            url: slug,
          },
        },
      );
      throw combinedError;
    }
  }

  /**
   * Fetch with proxy only (no direct attempt)
   */
  async fetchWithProxyOnly(slug: string): Promise<FetchResult> {
    const cached = await this.getCachedFetch(slug);
    if (cached) return cached;

    let croxyError: AppError | undefined;
    try {
      return await this.attemptCroxyProxyFetch(slug);
    } catch (error: unknown) {
      croxyError = toAppError(error, {
        url: slug,
        method: 'attemptCroxyProxyFetch',
      });
      logError(croxyError);
      logger.warn(`[ProxyHttpClient] scrapeCroxyProxy failed:`, croxyError);
      logger.warn('Trying direct axios fetch as final fallback...', {
        url: slug,
      });
    }

    try {
      const res: AxiosResponse = await axios.get(slug, {
        headers: DEFAULT_HEADERS,
        timeout: this.config.timeout || 10000,
      });
      return this.handleAxiosResponse(slug, res, 'Final direct axios fallback');
    } catch (finalError: unknown) {
      const finalAppError = toAppError(finalError, {
        url: slug,
        method: 'finalAxiosFallback',
      });
      logError(finalAppError);

      const combinedError = createNetworkError(
        `All fetch methods failed for ${slug}`,
        {
          originalError: finalError,
          context: {
            croxyError: croxyError?.message,
            finalError: finalAppError.message,
            url: slug,
          },
        },
      );
      logger.error(
        'Final direct axios fetch fallback also failed:',
        combinedError,
      );
      throw combinedError;
    }
  }
}

// Convenience functions for backward compatibility
export const fetchWithProxy = async (slug: string): Promise<FetchResult> => {
  const client = new ProxyHttpClient();
  return client.fetchWithProxy(slug);
};

export const fetchWithProxyOnly = async (
  slug: string,
): Promise<FetchResult> => {
  const client = new ProxyHttpClient();
  return client.fetchWithProxyOnly(slug);
};

export const CroxyProxyOnly = async (slug: string): Promise<FetchResult> => {
  const client = new ProxyHttpClient();
  return client.attemptCroxyProxyFetch(slug);
};
