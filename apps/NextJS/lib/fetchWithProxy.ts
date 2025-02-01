import axios from 'axios';
import { DEFAULT_HEADERS } from '@/lib/DHead';
import { HttpsProxyAgent } from 'https-proxy-agent';
import logger from '@/lib/logger';

const PROXY_LIST_URL =
  'https://raw.githubusercontent.com/MythEclipse/proxy-auto-ts/refs/heads/main/proxies.txt';
const getProxies = async (): Promise<string[]> => {
  try {
    const response = await fetch(PROXY_LIST_URL);
    if (!response.ok) {
      throw new Error(`Failed to fetch proxy list: ${response.statusText}`);
    }
    const data = await response.text();
    return data
      .split('\n')
      .filter((line) => line.trim() !== '' && !line.startsWith('#'))
      .map((line) => line.split(' ')[0].trim());
  } catch (error) {
    throw new Error(
      `Failed to retrieve proxy list: ${(error as Error).message}`
    );
  }
};
const proxies = await getProxies();
export async function fetchWithProxy(
  slug: string
): Promise<{ data: string | object; contentType: string | null }> {
  try {
    // Try direct fetch first
    const response = await fetch(slug, {
      headers: DEFAULT_HEADERS,
    });

    if (response.ok) {
      const contentType = response.headers.get('content-type');

      if (contentType && contentType.includes('application/json')) {
        const jsonData = await response.json();
        return { data: jsonData, contentType };
      }

      const textData = await response.text();
      return { data: textData, contentType };
    }

    throw new Error(`Direct fetch failed`);
  } catch {
    logger.error('Direct fetch failed, trying proxies');
    return await fetchFromProxies(slug);
  }
}

async function fetchFromProxies(
  slug: string
): Promise<{ data: string | object; contentType: string | null }> {
  let lastError: Error | null = null;

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
        const contentType = response.headers['content-type'];

        if (contentType && contentType.includes('application/json')) {
          logger.info(`Fetched from ${host}:${port}`);
          return { data: response.data, contentType };
        }
        logger.info(`Fetched from ${host}:${port}`);
        return { data: response.data, contentType };
      }
    } catch (error) {
      lastError = error as Error;
      // logger.error(`Error proxying request through ${host}:${port}`);
    }
  }

  throw new Error(lastError?.message || 'Failed to fetch from all proxies');
}
