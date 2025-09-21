import axios, { AxiosResponse } from 'axios';
import logger from './logger';
import { DEFAULT_HEADERS } from './DHead';
import { scrapeCroxyProxy } from '../lib/scrapeCroxyProxy';
import { redis } from '../lib/redis';
import { UnifiedHttpClient } from './http-client';
import { HttpClientConfig, FetchResult, HttpError } from '../types/http';

export interface CustomError extends Error {
  code?: string;
}

export class ProxyHttpClient extends UnifiedHttpClient {
  private proxyConfig: HttpClientConfig['proxy'];

  constructor(config: HttpClientConfig = {}) {
    super({
      timeout: 10000,
      ...config,
    });
    this.proxyConfig = config.proxy || { enabled: true, fallback: true };
  }

  private isInternetBaikBlockPage(data: string | object): boolean {
    if (typeof data !== 'string') return false;
    return (
      data.includes('internetbaik.telkomsel.com') ||
      data.includes('VmaxAdManager.js') ||
      data.includes('VmaxAdHelper')
    );
  }

  // --- REDIS CACHE WRAPPER START ---
  private getFetchCacheKey(slug: string): string {
    return `fetch:proxy:${slug}`;
  }

  private async getCachedFetch(slug: string): Promise<FetchResult | null> {
    try {
      const key = this.getFetchCacheKey(slug);
      const cached = await redis.get(key);
      if (cached) {
        try {
          const parsed = JSON.parse(
            typeof cached === 'string' ? cached : JSON.stringify(cached),
          );
          logger.info(`[ProxyHttpClient] Returning cached response for ${slug}`);
          return parsed;
        } catch {
          // ignore parse error, fallback to fetch
        }
      }
    } catch (redisError: unknown) {
      const err = redisError as CustomError;
      logger.warn(`[ProxyHttpClient] Redis get failed for ${slug}:`, {
        message: err?.message,
        code: err?.code,
        stack: err?.stack,
        url: process.env.UPSTASH_REDIS_REST_URL,
        hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
        tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length,
      });
      // ignore Redis error, fallback to fetch
    }
    return null;
  }

  private async setCachedFetch(slug: string, value: FetchResult) {
    try {
      const key = this.getFetchCacheKey(slug);
      await redis.set(key, JSON.stringify(value), { EX: 120 });
    } catch (redisError) {
      logger.warn(`[ProxyHttpClient] Redis set failed for ${slug}:`, {
        error: redisError,
        url: process.env.UPSTASH_REDIS_REST_URL,
        hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
        tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length,
      });
      // ignore Redis error, continue without caching
    }
  }
  // --- REDIS CACHE WRAPPER END ---

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

    throw new Error(`${source} fetch failed with status ${res.status}`);
  }

  private async attemptAxiosFetch(slug: string): Promise<FetchResult> {
    const res: AxiosResponse = await axios.get(slug, {
      headers: DEFAULT_HEADERS,
      timeout: this.config.timeout || 10000,
    });
    return this.handleAxiosResponse(slug, res, 'Direct axios');
  }

  async attemptCroxyProxyFetch(slug: string): Promise<FetchResult> {
    logger.info(`[ProxyHttpClient] Using scrapeCroxyProxy for ${slug}`);
    const html = await scrapeCroxyProxy(slug);
    const result = { data: html, contentType: 'text/html' };
    await this.setCachedFetch(slug, result);
    return result;
  }

  async fetchWithProxy(slug: string): Promise<FetchResult> {
    const cached = await this.getCachedFetch(slug);
    if (cached) return cached;

    let directError: Error | undefined;
    try {
      return await this.attemptAxiosFetch(slug);
    } catch (error: unknown) {
      directError = error as CustomError;
      logger.warn(`Direct axios fetch failed for ${slug}:`, directError);
      logger.error('Direct axios fetch failed, trying scrapeCroxyProxy');
    }

    try {
      return await this.attemptCroxyProxyFetch(slug);
    } catch (croxyError: unknown) {
      logger.error(`ScrapeCroxyProxy also failed for ${slug}:`, croxyError);
      throw new Error(
        `Both direct axios fetch and proxy failed for ${slug}. Direct error: ${directError?.message}. Proxy error: ${(croxyError as CustomError).message}`,
      );
    }
  }

  async fetchWithProxyOnly(slug: string): Promise<FetchResult> {
    const cached = await this.getCachedFetch(slug);
    if (cached) return cached;

    let croxyError: Error | undefined;
    try {
      return await this.attemptCroxyProxyFetch(slug);
    } catch (error: unknown) {
      croxyError = error as CustomError;
      logger.warn(`[ProxyHttpClient] scrapeCroxyProxy failed:`, {
        url: slug,
        error: croxyError.message,
      });
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
      logger.error('Final direct axios fetch fallback also failed:', {
        finalError: (finalError as CustomError).message,
        url: slug,
      });
      throw new Error(
        `All fetch methods failed for ${slug}. Scrape error: ${croxyError?.message}. Final axios fetch error: ${(finalError as CustomError).message}`,
      );
    }
  }
}

// Convenience functions for backward compatibility
export const fetchWithProxy = async (slug: string): Promise<FetchResult> => {
  const client = new ProxyHttpClient();
  return client.fetchWithProxy(slug);
};

export const fetchWithProxyOnly = async (slug: string): Promise<FetchResult> => {
  const client = new ProxyHttpClient();
  return client.fetchWithProxyOnly(slug);
};

export const CroxyProxyOnly = async (slug: string): Promise<FetchResult> => {
  const client = new ProxyHttpClient();
  return client.attemptCroxyProxyFetch(slug);
};
