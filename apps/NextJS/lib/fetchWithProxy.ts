import axios from 'axios';
import { DEFAULT_HEADERS } from '@/lib/DHead';
import { HttpsProxyAgent } from 'https-proxy-agent';
import { SocksProxyAgent } from 'socks-proxy-agent';
import logger from '@/lib/logger';
import https from 'https';

const DEFAULT_PROXY_LIST_URL =
  'https://www.proxy-list.download/api/v1/get?type=https';

function getProxyListUrl(): string {
  return process.env.PROXY_LIST_URL || DEFAULT_PROXY_LIST_URL;
}

// Accepts lines like host:port, adds https:// prefix for agent
function parseProxyLine(line: string): string | null {
  const trimmed = line.trim();
  if (!trimmed || trimmed.startsWith('#')) return null;
  // If already protocol-prefixed, use as is
  if (/^(http|https|socks4|socks5):\/\//.test(trimmed)) return trimmed;
  // If just host:port, treat as https proxy
  if (/^[^:]+:\d+$/.test(trimmed)) return `https://${trimmed}`;
  return null;
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
      .map(parseProxyLine)
      .filter((proxy): proxy is string => !!proxy);
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

// Helper: detect "internetbaik" block page
function isInternetBaikBlockPage(data: string | object): boolean {
  if (typeof data !== 'string') return false;
  return (
    data.includes('internetbaik.telkomsel.com') ||
    data.includes('VmaxAdManager.js') ||
    data.includes('VmaxAdHelper')
  );
}

// Direct fetch, fallback to proxy if needed
export async function fetchWithProxy(
  slug: string,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
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
        if (isInternetBaikBlockPage(JSON.stringify(jsonData))) {
          logger.warn('Blocked by internetbaik (direct fetch), trying proxies');
          return await fetchFromProxies(slug);
        }
        return { data: jsonData, contentType };
      }
      const textData = await res.text();
      logger.info(`[fetchWithProxy] Direct fetch text body:`, textData);
      if (isInternetBaikBlockPage(textData)) {
        logger.warn('Blocked by internetbaik (direct fetch), trying proxies');
        return await fetchFromProxies(slug);
      }
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

// Proxy only, never direct
export async function fetchWithProxyOnly(
  slug: string
): Promise<{ data: string | object; contentType: string | null }> {
  return await fetchFromProxies(slug);
}

function getAgent(proxyUrl: string) {
  if (proxyUrl.startsWith('socks4://') || proxyUrl.startsWith('socks5://')) {
    return new SocksProxyAgent(proxyUrl);
  }
  // Default to HTTP/HTTPS
  return new HttpsProxyAgent(proxyUrl);
}

async function fetchFromProxies(
  slug: string
): Promise<{ data: string | object; contentType: string | null }> {
  let lastError: Error | null = null;
  const proxies = await getCachedProxies();
  for (const proxyUrl of proxies) {
    try {
      const agent = getAgent(proxyUrl);

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
          if (isInternetBaikBlockPage(response.data)) {
            logger.warn(`Blocked by internetbaik (proxy ${proxyUrl}), trying next proxy`);
            continue;
          }
        } else {
          logger.info(`[fetchWithProxy] Proxy fetch JSON body:`, response.data);
          if (isInternetBaikBlockPage(JSON.stringify(response.data))) {
            logger.warn(`Blocked by internetbaik (proxy ${proxyUrl}), trying next proxy`);
            continue;
          }
        }
        logger.info(`Fetched from ${proxyUrl} for ${slug}`);
        return { data: response.data, contentType };
      }
    } catch (error) {
      lastError = error as Error;
      logger.warn(`Proxy fetch failed for ${slug} via ${proxyUrl}:`, error);
    }
  }
  logger.error(`Failed to fetch from all proxies for ${slug}:`, lastError);
  throw new Error(lastError?.message || 'Failed to fetch from all proxies');
}
