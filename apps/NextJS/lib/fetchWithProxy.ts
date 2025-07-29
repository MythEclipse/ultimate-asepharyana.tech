import axios from 'axios';
import { DEFAULT_HEADERS } from '@/lib/DHead';
import { HttpsProxyAgent } from 'https-proxy-agent';
import logger from '@/lib/logger';
import https from 'https';

const DEFAULT_PROXY_LIST_URL =
  'https://raw.githubusercontent.com/clarketm/proxy-list/master/proxy-list-raw.txt';

function getProxyListUrl(): string {
  return process.env.PROXY_LIST_URL || DEFAULT_PROXY_LIST_URL;
}

async function getProxies(): Promise<string[]> {
  try {
    const res = await fetch(getProxyListUrl());
    if (!res.ok) {
      const error = new Error(`Failed to fetch proxy list: ${res.statusText}`);
      logger.error('Error fetching proxy list:', error);
      throw error;
    }
    const data = await res.text();
    return data
      .split('\n')
      .filter((line) => {
        const trimmed = line.trim();
        // Only allow lines that look like host:port
        return trimmed !== '' && !trimmed.startsWith('#') && /^[^:]+:\d+$/.test(trimmed);
      })
      .map((line) => line.split(' ')[0].trim());
  } catch (error) {
    logger.error('Network or unexpected error while fetching proxy list:', error);
    throw error;
  }
}

let cachedProxies: string[] | null = null;
let cacheTimestamp = 0;
const CACHE_DURATION = 6 * 60 * 1000;

async function getCachedProxies(): Promise<string[]> {
  const now = Date.now();
  if (!cachedProxies || now - cacheTimestamp > CACHE_DURATION) {
    cachedProxies = await getProxies();
    cacheTimestamp = now;
  }
  return cachedProxies;
}

export async function fetchWithProxy(
  slug: string,
  useProxies: boolean = true
): Promise<{ data: string | object; contentType: string | null }> {
  try {
    const res = await fetch(slug, {
      headers: DEFAULT_HEADERS,
      cache: 'no-store',
    });
    logger.info(`[fetchWithProxy] Direct fetch response:`, {
      url: slug,
      status: res.status,
      headers: Object.fromEntries(res.headers.entries())
    });
    if (res.ok) {
      const contentType = res.headers.get('content-type');
      if (contentType && contentType.includes('application/json')) {
        const jsonData = await res.json();
        logger.info(`[fetchWithProxy] Direct fetch JSON body:`, jsonData);
        return { data: jsonData, contentType };
      }
      const textData = await res.text();
      logger.info(`[fetchWithProxy] Direct fetch text body:`, textData);
      return { data: textData, contentType };
    }
    const error = new Error(`Direct fetch failed with status ${res.status}`);
    logger.error(`Direct fetch failed for ${slug}: Status ${res.status}`, error);
    logger.error('Direct fetch failed, trying proxies');
    return await fetchFromProxies(slug);
  } catch (error) {
    logger.warn(`Direct fetch failed for ${slug}:`, error);
    logger.error('Direct fetch failed, trying proxies');
    return await fetchFromProxies(slug);
  }
}

async function fetchFromProxies(
  slug: string
): Promise<{ data: string | object; contentType: string | null }> {
  let lastError: Error | null = null;
  const proxies = await getCachedProxies();
  for (const proxy of proxies) {
    // Validate proxy format again for safety
    if (!proxy || !/^[^:]+:\d+$/.test(proxy)) {
      logger.warn(`Skipping invalid proxy entry: "${proxy}"`);
      continue;
    }
    const [host, port] = proxy.split(':');
    if (!host || !port) {
      logger.warn(`Skipping malformed proxy: "${proxy}"`);
      continue;
    }
    try {
      const proxyUrl = `http://${host}:${port}`;
      const agent = new HttpsProxyAgent(proxyUrl);

      // Create HTTPS agent to bypass SSL verification
      const httpsAgent = new https.Agent({
        rejectUnauthorized: false
      });

      // Configure axios with both proxy agent and https agent
      const axiosConfig = {
        headers: DEFAULT_HEADERS,
        httpsAgent,
        httpAgent: agent,
        timeout: 6000
      };

      const response = await axios.get(slug, axiosConfig);

      logger.info(`[fetchWithProxy] Proxy fetch response:`, {
        url: slug,
        proxy: proxyUrl,
        status: response.status,
        headers: response.headers
      });

      if (response.status === 200) {
        const contentType = response.headers['content-type'] || null;
        if (typeof response.data === 'string') {
          logger.info(`[fetchWithProxy] Proxy fetch text body:`, response.data);
        } else {
          logger.info(`[fetchWithProxy] Proxy fetch JSON body:`, response.data);
        }
        logger.info(`Fetched from ${host}:${port} for ${slug}`);
        return { data: response.data, contentType };
      }
    } catch (error) {
      lastError = error as Error;
      logger.warn(`Proxy fetch failed for ${slug} via ${host}:${port}:`, error);
    }
  }
  logger.error(`Failed to fetch from all proxies for ${slug}:`, lastError);
  throw new Error(lastError?.message || 'Failed to fetch from all proxies');
}
