// Utility functions for Komik base URL logic

import logger from '../utils/logger';
import { redis } from './redis';
import * as cheerio from 'cheerio';
import { fetchWithProxyOnly } from './fetchWithProxy';

// --- SINGLE FLIGHT LOGIC WITH REDIS LOCK START ---
let komikBaseUrlPromise: Promise<string> | null = null;
const KOMIK_BASE_URL_LOCK_KEY = 'komik:baseurl:lock';
const KOMIK_BASE_URL_KEY = 'komik:baseurl';

async function acquireRedisLock(key: string, ttlMs: number): Promise<boolean> {
  return (await redis.set(key, 'locked', { NX: true, PX: ttlMs })) === 'OK';
}

async function releaseRedisLock(key: string) {
  await redis.del(key);
}

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const fetchWithProxyOnlyWrapper = async (url: string): Promise<string> => {
  try {
    logger.debug('[fetchWithProxyOnlyWrapper] Fetching', { url });
    const response = await fetchWithProxyOnly(url);
    logger.info('[fetchWithProxyOnlyWrapper] Fetched', { url });
    return typeof response.data === 'string'
      ? response.data
      : JSON.stringify(response.data);
  } catch (error) {
    logger.error('[fetchWithProxyOnlyWrapper] Error', {
      url,
      error: (error as Error).message,
      stack: (error as Error).stack,
      name: (error as Error).name,
    });
    throw new Error(
      `Failed to fetch data from ${url}: ${(error as Error).message}`,
    );
  }
};

export const getDynamicKomikBaseUrl = async (): Promise<string> => {
  if (komikBaseUrlPromise) {
    logger.debug('[getDynamicKomikBaseUrl] Returning in-flight promise');
    return komikBaseUrlPromise;
  }
  komikBaseUrlPromise = (async () => {
    const lockTtl = 10000; // 10 seconds
    const waitInterval = 200; // ms
    const maxWait = 10000; // 10 seconds
    let waited = 0;

    // Try to acquire lock
    let pollCount = 0;
    const maxPolls = Math.ceil(maxWait / waitInterval);
    while (!(await acquireRedisLock(KOMIK_BASE_URL_LOCK_KEY, lockTtl))) {
      logger.debug('[getDynamicKomikBaseUrl] Waiting for Redis lock...');
      await sleep(waitInterval);
      waited += waitInterval;
      pollCount++;
      // If waited too long or polled too many times, break and try anyway
      if (waited >= maxWait || pollCount >= maxPolls) {
        logger.warn(
          '[getDynamicKomikBaseUrl] Waited too long for lock, proceeding anyway',
        );
        return ''; // Return empty string or throw error as appropriate
      }
      // Check if value is already cached by other process
      const cached = await redis.get(KOMIK_BASE_URL_KEY);
      if (typeof cached === 'string' && cached && !cached.includes('.cz')) {
        logger.info(
          '[getDynamicKomikBaseUrl] Found cached base URL while waiting for lock',
          { cached },
        );
        return cached;
      }
    }

    try {
      logger.debug('[getDynamicKomikBaseUrl] Fetching komik base URL');
      const body = await fetchWithProxyOnlyWrapper('https://komikindo.cz/');
      const $ = cheerio.load(body);

      // Cari tombol WEBSITE yang mengandung link asli (bukan .cz)
      const websiteBtn = $('a.elementskit-btn')
        .filter((_, el) => {
          const href = $(el).attr('href') || '';
          // Cari href yang mengandung komikindo dan bukan .cz
          return (
            /komikindo\.(?!cz)/.test(href) ||
            /komikindo\./.test($(el).attr('__cporiginalvalueofhref') || '')
          );
        })
        .first();

      // Cek di atribut __cporiginalvalueofhref jika ada, jika tidak pakai href
      let orgLink =
        websiteBtn.attr('__cporiginalvalueofhref') ||
        websiteBtn.attr('href') ||
        '';

      // Jika link berupa IP, decode dari query string __cpo
      if (/^\d+\.\d+\.\d+\.\d+/.test(orgLink)) {
        const urlObj = new URL(orgLink);
        const cpo = urlObj.searchParams.get('__cpo');
        if (cpo) {
          try {
            const decoded = Buffer.from(cpo, 'base64').toString('utf-8');
            orgLink = decoded;
          } catch {
            logger.error('[getDynamicKomikBaseUrl] Failed to decode __cpo', {
              cpo,
            });
          }
        }
      }

      if (!orgLink || orgLink.includes('.cz')) {
        logger.error(
          '[getDynamicKomikBaseUrl] Failed to fetch komik base URL selain cz',
        );
        throw new Error('Failed to fetch komik base URL selain cz');
      }
      logger.info('[getDynamicKomikBaseUrl] Got base URL', { orgLink });
      // Cache the result immediately for other waiters
      await redis.set(KOMIK_BASE_URL_KEY, orgLink.replace(/\/$/, ''), {
        EX: 60 * 60 * 24 * 30,
      });
      return orgLink.replace(/\/$/, '');
    } finally {
      await releaseRedisLock(KOMIK_BASE_URL_LOCK_KEY);
      komikBaseUrlPromise = null;
    }
  })();
  return komikBaseUrlPromise;
};
// --- SINGLE FLIGHT LOGIC WITH REDIS LOCK END ---

export const getCachedKomikBaseUrl = async (
  forceRefresh = false,
): Promise<string> => {
  if (!forceRefresh) {
    const cached = await redis.get(KOMIK_BASE_URL_KEY);
    if (typeof cached === 'string' && cached && !cached.includes('.cz')) {
      logger.info('[getCachedKomikBaseUrl] Using cached base URL', { cached });
      return cached;
    }
  }
  // Fetch new value and cache it
  const url = await getDynamicKomikBaseUrl();
  await redis.set(KOMIK_BASE_URL_KEY, url, { EX: 60 * 60 * 24 * 30 });
  logger.info('[getCachedKomikBaseUrl] Refreshed and cached base URL', { url });
  return url;
};
