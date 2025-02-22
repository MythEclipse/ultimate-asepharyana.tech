import axios from 'axios';
import { DEFAULT_HEADERS } from '@/lib/DHead';
import { HttpsProxyAgent } from 'https-proxy-agent';
import logger from '@/lib/logger';

const PROXY_LIST_URL =
  'https://raw.githubusercontent.com/MythEclipse/proxy-auto-ts/refs/heads/main/proxies.txt';

async function getProxies(): Promise<string[]> {
  const res = await fetch(PROXY_LIST_URL);
  if (!res.ok) {
    throw new Error(`Failed to fetch proxy list: ${res.statusText}`);
  }
  const data = await res.text();
  return data
    .split('\n')
    .filter((line) => line.trim() !== '' && !line.startsWith('#'))
    .map((line) => line.split(' ')[0].trim());
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
  useProxies: boolean = false
): Promise<{ data: string | object; contentType: string | null }> {
  let attempts = 0;
  const maxAttempts = 3;

  while (attempts < maxAttempts) {
    try {
      const res = await fetch(slug, {
        headers: DEFAULT_HEADERS,
        next: { revalidate: 360 },
      });
      if (res.ok) {
        const contentType = res.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
          const jsonData = await res.json();
          return { data: jsonData, contentType };
        }
        const textData = await res.text();
        return { data: textData, contentType };
      }
      throw new Error(`Direct fetch failed with status ${res.status}`);
    } catch {
      attempts++;
      if (attempts >= maxAttempts) {
        if (useProxies) {
          logger.error('Direct fetch failed, trying proxies');
          return await fetchFromProxies(slug);
        } else {
          throw new Error('Direct fetch failed and proxy usage is disabled');
        }
      }
    }
  }
  throw new Error('Failed to fetch after maximum attempts');
}

async function fetchFromProxies(
  slug: string
): Promise<{ data: string | object; contentType: string | null }> {
  let lastError: Error | null = null;
  const proxies = await getCachedProxies();
  for (const proxy of proxies) {
    const [host, port] = proxy.split(':');
    try {
      const proxyUrl = `http://${host}:${port}`;
      const agent = new HttpsProxyAgent(proxyUrl);
      const response = await axios.get(slug, {
        headers: DEFAULT_HEADERS,
        httpsAgent: agent,
        timeout: 6000,
      });
      if (response.status === 200) {
        const contentType = response.headers['content-type'] || null;
        logger.info(`Fetched from ${host}:${port}`);
        return { data: response.data, contentType };
      }
    } catch (error) {
      lastError = error as Error;
    }
  }
  throw new Error(lastError?.message || 'Failed to fetch from all proxies');
}
