import axios, { AxiosResponse } from 'axios';
import logger from '../utils/logger';
import { DEFAULT_HEADERS } from '../utils/DHead';
import { scrapeCroxyProxy } from './scrapeCroxyProxy';
import { redis } from './redis';

interface FetchResult {
  data: string | object;
  contentType: string | null;
}

export async function CroxyProxyOnly(slug: string): Promise<FetchResult> {
  logger.info(`[CroxyProxyOnly] Using scrapeCroxyProxy for ${slug}`);
  const html = await scrapeCroxyProxy(slug);
  return { data: html, contentType: 'text/html' };
}

function isInternetBaikBlockPage(data: string | object): boolean {
  if (typeof data !== 'string') return false;
  return (
    data.includes('internetbaik.telkomsel.com') ||
    data.includes('VmaxAdManager.js') ||
    data.includes('VmaxAdHelper')
  );
}

// --- REDIS CACHE WRAPPER START ---
function getFetchCacheKey(slug: string): string {
  return `fetch:proxy:${slug}`;
}

async function getCachedFetch(slug: string): Promise<FetchResult | null> {
  try {
    const key = getFetchCacheKey(slug);
    const cached = await redis.get(key);
    if (cached) {
      try {
        const parsed = JSON.parse(
          typeof cached === 'string' ? cached : JSON.stringify(cached),
        );
        logger.info(`[fetchWithProxy] Returning cached response for ${slug}`);
        return parsed;
      } catch {
        // ignore parse error, fallback to fetch
      }
    }
  } catch (redisError: any) {
    logger.warn(`[fetchWithProxy] Redis get failed for ${slug}:`, {
      message: redisError?.message,
      code: redisError?.code,
      stack: redisError?.stack,
      url: process.env.UPSTASH_REDIS_REST_URL,
      hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
      tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length,
    });
    // ignore Redis error, fallback to fetch
  }
  return null;
}

async function setCachedFetch(slug: string, value: FetchResult) {
  try {
    const key = getFetchCacheKey(slug);
    await redis.set(key, JSON.stringify(value), { EX: 120 });
  } catch (redisError) {
    logger.warn(`[fetchWithProxy] Redis set failed for ${slug}:`, {
      error: redisError,
      url: process.env.UPSTASH_REDIS_REST_URL,
      hasToken: !!process.env.UPSTASH_REDIS_REST_TOKEN,
      tokenLength: process.env.UPSTASH_REDIS_REST_TOKEN?.length,
    });
    // ignore Redis error, continue without caching
  }
}
// --- REDIS CACHE WRAPPER END ---

async function handleFetchResponse(
  slug: string,
  res: AxiosResponse,
  source: string,
): Promise<FetchResult> {
  logger.info(`[fetchWithProxy] ${source} fetch response:`, {
    url: slug,
    status: res.status,
    headers: res.headers,
  });

  if (res.status >= 200 && res.status < 300) {
    logger.info(
      `[fetchWithProxy] ${source} fetch successful for ${slug}, status: ${res.status}`,
    );
    const contentType = res.headers['content-type'] || res.headers['Content-Type'];
    let data = res.data;

    if (contentType?.includes('application/json')) {
      data = JSON.stringify(data); // Convert to string for isInternetBaikBlockPage check
    }

    if (isInternetBaikBlockPage(data)) {
      logger.warn(`Blocked by internetbaik (${source} fetch), trying scrapeCroxyProxy`);
      const croxyResult = await CroxyProxyOnly(slug);
      await setCachedFetch(slug, croxyResult);
      return croxyResult;
    }

    const result = { data: res.data, contentType };
    await setCachedFetch(slug, result);
    return result;
  }

  throw new Error(`${source} fetch failed with status ${res.status}`);
}

async function attemptAxiosFetch(slug: string): Promise<FetchResult> {
  const res: AxiosResponse = await axios.get(slug, {
    headers: DEFAULT_HEADERS,
    timeout: 10000, // 10 second timeout
  });
  return handleFetchResponse(slug, res, 'Direct axios');
}

async function attemptCroxyProxyFetch(slug: string): Promise<FetchResult> {
  const croxyResult = await CroxyProxyOnly(slug);
  await setCachedFetch(slug, croxyResult);
  return croxyResult;
}

export async function fetchWithProxy(slug: string): Promise<FetchResult> {
  const cached = await getCachedFetch(slug);
  if (cached) return cached;

  let directError: Error | undefined;
  try {
    return await attemptAxiosFetch(slug);
  } catch (error) {
    directError = error as Error;
    logger.warn(`Direct axios fetch failed for ${slug}:`, directError);
    logger.error('Direct axios fetch failed, trying scrapeCroxyProxy');
  }

  try {
    return await attemptCroxyProxyFetch(slug);
  } catch (croxyError) {
    logger.error(`ScrapeCroxyProxy also failed for ${slug}:`, croxyError);
    throw new Error(
      `Both direct axios fetch and proxy failed for ${slug}. Direct error: ${directError?.message}. Proxy error: ${(croxyError as Error).message}`,
    );
  }
}

export async function fetchWithProxyOnly(slug: string): Promise<FetchResult> {
  const cached = await getCachedFetch(slug);
  if (cached) return cached;

  let croxyError: Error | undefined;
  try {
    return await attemptCroxyProxyFetch(slug);
  } catch (error) {
    croxyError = error as Error;
    logger.warn(`[fetchWithProxyOnly] scrapeCroxyProxy failed:`, {
      url: slug,
      error: croxyError.message,
    });
    logger.warn('Trying direct axios fetch as final fallback...', { url: slug });
  }

  try {
    const res: AxiosResponse = await axios.get(slug, {
      headers: DEFAULT_HEADERS,
      timeout: 10000, // 10 second timeout
    });
    return handleFetchResponse(slug, res, 'Final direct axios fallback');
  } catch (finalError) {
    logger.error('Final direct axios fetch fallback also failed:', {
      finalError: (finalError as Error).message,
      url: slug,
    });
    throw new Error(
      `All fetch methods failed for ${slug}. Scrape error: ${croxyError?.message}. Final axios fetch error: ${(finalError as Error).message}`,
    );
  }
}
