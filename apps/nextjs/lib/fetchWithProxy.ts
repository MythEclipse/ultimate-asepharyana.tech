import axios, { AxiosResponse } from 'axios';
import logger from '../utils/logger';
import { DEFAULT_HEADERS } from '../utils/DHead';
import { scrapeCroxyProxy } from './scrapeCroxyProxy';
import { redis } from './redis';
// --- CroxyProxyOnly Export (moved to top to fix hoisting issue) ---
export async function CroxyProxyOnly(
  slug: string,
): Promise<{ data: string | object; contentType: string | null }> {
  try {
    logger.info(`[CroxyProxyOnly] Using scrapeCroxyProxy for ${slug}`);
    const html = await scrapeCroxyProxy(slug);
    return { data: html, contentType: 'text/html' };
  } catch (error) {
    logger.error('[CroxyProxyOnly] scrapeCroxyProxy failed:', error);
    throw new Error('CroxyProxyOnly failed: ' + (error as Error).message);
  }
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

async function getCachedFetch(
  slug: string,
): Promise<{ data: string | object; contentType: string | null } | null> {
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
  } catch (redisError) {
    logger.warn(`[fetchWithProxy] Redis get failed for ${slug}:`, redisError);
    // ignore Redis error, fallback to fetch
  }
  return null;
}

async function setCachedFetch(
  slug: string,
  value: { data: string | object; contentType: string | null },
) {
  try {
    const key = getFetchCacheKey(slug);
    await redis.set(key, JSON.stringify(value), { ex: 120 });
  } catch (redisError) {
    logger.warn(`[fetchWithProxy] Redis set failed for ${slug}:`, redisError);
    // ignore Redis error, continue without caching
  }
}
// --- REDIS CACHE WRAPPER END ---

export async function fetchWithProxy(
  slug: string,
): Promise<{ data: string | object; contentType: string | null }> {
  // Try Redis cache first
  const cached = await getCachedFetch(slug);
  if (cached) return cached;

  try {
    const res: AxiosResponse = await axios.get(slug, {
      headers: DEFAULT_HEADERS,
      timeout: 10000, // 10 second timeout
    });
    logger.info(`[fetchWithProxy] Direct axios fetch response:`, {
      url: slug,
      status: res.status,
      headers: res.headers,
    });
    if (res.status >= 200 && res.status < 300) {
      logger.info(`[fetchWithProxy] Direct axios fetch successful for ${slug}, status: ${res.status}`);
      const contentType = res.headers['content-type'] || res.headers['Content-Type'];
      if (contentType?.includes('application/json')) {
        const jsonData = res.data;
        if (isInternetBaikBlockPage(JSON.stringify(jsonData))) {
          logger.warn(
            'Blocked by internetbaik (direct fetch), trying scrapeCroxyProxy',
          );
          const croxyResult = await CroxyProxyOnly(slug);
          await setCachedFetch(slug, croxyResult);
          return croxyResult;
        }
        const result = { data: jsonData, contentType };
        await setCachedFetch(slug, result);
        return result;
      }
      const textData = res.data;
      if (isInternetBaikBlockPage(textData)) {
        logger.warn(
          'Blocked by internetbaik (direct fetch), trying scrapeCroxyProxy',
        );
        const croxyResult = await CroxyProxyOnly(slug);
        await setCachedFetch(slug, croxyResult);
        return croxyResult;
      }
      const result = { data: textData, contentType };
      await setCachedFetch(slug, result);
      return result;
    }
    const error = new Error(`Direct axios fetch failed with status ${res.status}`);
    logger.error(
      `Direct axios fetch failed for ${slug}: Status ${res.status}`,
      error,
    );
    logger.error('Direct axios fetch failed, trying scrapeCroxyProxy');
    try {
      const croxyResult = await CroxyProxyOnly(slug);
      await setCachedFetch(slug, croxyResult);
      return croxyResult;
    } catch (croxyError) {
      logger.error(`ScrapeCroxyProxy also failed for ${slug}:`, croxyError);
      throw new Error(`Both direct axios fetch and proxy failed for ${slug}. Direct error: ${(error as Error).message}. Proxy error: ${(croxyError as Error).message}`);
    }
  } catch (error) {
    logger.warn(`Direct axios fetch failed for ${slug}:`, error);
    logger.error('Direct axios fetch failed, trying scrapeCroxyProxy');
    try {
      const croxyResult = await CroxyProxyOnly(slug);
      await setCachedFetch(slug, croxyResult);
      return croxyResult;
    } catch (croxyError) {
      logger.error(`ScrapeCroxyProxy also failed for ${slug}:`, croxyError);
      throw new Error(`Both direct axios fetch and proxy failed for ${slug}. Direct error: ${(error as Error).message}. Proxy error: ${(croxyError as Error).message}`);
    }
  }
}

export async function fetchWithProxyOnly(
  slug: string,
): Promise<{ data: string | object; contentType: string | null }> {
  // Try Redis cache first
  const cached = await getCachedFetch(slug);
  if (cached) return cached;

  // Try scrapeCroxyProxy directly
  try {
    logger.info(`[fetchWithProxyOnly] Using scrapeCroxyProxy for ${slug}`);
    const croxyResult = await CroxyProxyOnly(slug);
    await setCachedFetch(slug, croxyResult);
    return croxyResult;
  } catch (croxyError) {
    logger.warn(`[fetchWithProxyOnly] scrapeCroxyProxy failed:`, {
      url: slug,
      error: (croxyError as Error).message,
    });
    // Try direct axios fetch as final fallback
    try {
      logger.warn('Trying direct axios fetch as final fallback...', { url: slug });
      const finalRes: AxiosResponse = await axios.get(slug, {
        headers: DEFAULT_HEADERS,
        timeout: 10000, // 10 second timeout
      });
      if (finalRes.status >= 200 && finalRes.status < 300) {
        const contentType = finalRes.headers['content-type'] || finalRes.headers['Content-Type'];
        if (contentType?.includes('application/json')) {
          const jsonData = finalRes.data;
          const result = { data: jsonData, contentType };
          await setCachedFetch(slug, result);
          return result;
        }
        const textData = finalRes.data;
        const result = { data: textData, contentType };
        await setCachedFetch(slug, result);
        return result;
      }
      throw new Error(
        `Final direct axios fetch failed with status ${finalRes.status}`,
      );
    } catch (finalError) {
      logger.error('Final direct axios fetch fallback also failed:', {
        finalError: (finalError as Error).message,
        url: slug,
      });
      throw new Error(
        `All fetch methods failed for ${slug}. Scrape error: ${(croxyError as Error).message}. Final axios fetch error: ${(finalError as Error).message}`,
      );
    }
  }
}

