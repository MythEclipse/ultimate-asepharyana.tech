/**
 * Unified Proxy Manager
 * Provides centralized proxy management with fallback mechanisms and caching
 */

import logger from '../utils/unified-logger';
import { redis } from './redis';
import { HttpClient } from '../utils/unified-http-client';
import { toAppError, logError } from '../utils/error-handler';

export interface ProxyConfig {
  enabled: boolean;
  preferDirect: boolean;
  fallbackEnabled: boolean;
  maxRetries: number;
  timeout: number;
  cacheTtl: number;
}

export interface ProxyResult<T = string> {
  success: boolean;
  data?: T;
  error?: Error;
  source: 'direct' | 'proxy' | 'cache';
  responseTime: number;
}

export class ProxyManager {
  private config: ProxyConfig;
  private httpClient: typeof HttpClient;

  constructor(config: Partial<ProxyConfig> = {}) {
    this.config = {
      enabled: true,
      preferDirect: true,
      fallbackEnabled: true,
      maxRetries: 3,
      timeout: 30000,
      cacheTtl: 3600, // 1 hour
      ...config,
    };
    this.httpClient = HttpClient;
  }

  /**
   * Generate cache key for proxy requests
   */
  private getCacheKey(url: string): string {
    return `proxy:${url}`;
  }

  /**
   * Check if content is blocked by Internet Baik
   */
  private isInternetBaikBlocked(content: string): boolean {
    return (
      content.includes('internetbaik.telkomsel.com') ||
      content.includes('VmaxAdManager.js') ||
      content.includes('VmaxAdHelper')
    );
  }

  /**
   * Attempt direct fetch
   */
  private async attemptDirectFetch(url: string): Promise<ProxyResult<string>> {
    const startTime = Date.now();

    try {
      const response = await this.httpClient.fetchJson<string>(url, {
        timeout: this.config.timeout,
      });

      const responseTime = Date.now() - startTime;

      // Check if content is blocked
      if (this.isInternetBaikBlocked(response)) {
        return {
          success: false,
          error: new Error('Content blocked by Internet Baik'),
          source: 'direct',
          responseTime,
        };
      }

      return {
        success: true,
        data: response,
        source: 'direct',
        responseTime,
      };
    } catch (error) {
      return {
        success: false,
        error: toAppError(error),
        source: 'direct',
        responseTime: Date.now() - startTime,
      };
    }
  }

  /**
   * Attempt proxy fetch using CroxyProxy
   */
  private async attemptProxyFetch(url: string): Promise<ProxyResult<string>> {
    const startTime = Date.now();

    try {
      // Dynamic import to avoid browser issues
      const { scrapeCroxyProxyCached } = await import('./scrapeCroxyProxy');
      logger.info(`[ProxyManager] Using CroxyProxy for ${url}`);

      const data = await scrapeCroxyProxyCached(url);
      const responseTime = Date.now() - startTime;

      return {
        success: true,
        data,
        source: 'proxy',
        responseTime,
      };
    } catch (error) {
      logError(toAppError(error), { url, source: 'proxy' });
      return {
        success: false,
        error: toAppError(error),
        source: 'proxy',
        responseTime: Date.now() - startTime,
      };
    }
  }

  /**
   * Fetch content with proxy support and fallback mechanisms
   */
  async fetchWithProxy(url: string): Promise<string> {
    if (!this.config.enabled) {
      // Direct fetch only
      const result = await this.attemptDirectFetch(url);
      if (result.success) {
        return result.data!;
      }
      throw result.error || new Error('Direct fetch failed');
    }

    const cacheKey = this.getCacheKey(url);

    // Check cache first
    try {
      const cached = await redis.get(cacheKey);
      if (cached && typeof cached === 'string') {
        logger.info(`[ProxyManager] Cache hit for ${url}`);
        return cached;
      }
    } catch (error) {
      logger.warn(`[ProxyManager] Cache check failed for ${url}`, { error });
    }

    let result: ProxyResult<string>;

    if (this.config.preferDirect) {
      // Try direct first, then proxy
      result = await this.attemptDirectFetch(url);

      if (!result.success && this.config.fallbackEnabled) {
        logger.info(`[ProxyManager] Direct fetch failed, trying proxy for ${url}`);
        result = await this.attemptProxyFetch(url);
      }
    } else {
      // Try proxy first, then direct
      result = await this.attemptProxyFetch(url);

      if (!result.success && this.config.fallbackEnabled) {
        logger.info(`[ProxyManager] Proxy fetch failed, trying direct for ${url}`);
        result = await this.attemptDirectFetch(url);
      }
    }

    if (result.success && result.data) {
      // Cache successful result
      try {
        await redis.set(cacheKey, result.data, { EX: this.config.cacheTtl });
        logger.info(`[ProxyManager] Cached result for ${url}`);
      } catch (error) {
        logger.warn(`[ProxyManager] Cache storage failed for ${url}`, { error });
      }

      return result.data;
    }

    // All attempts failed
    throw result.error || new Error('All proxy attempts failed');
  }

  /**
   * Fetch with proxy only (no direct attempts)
   */
  async fetchWithProxyOnly(url: string): Promise<string> {
    const result = await this.attemptProxyFetch(url);

    if (result.success && result.data) {
      return result.data;
    }

    throw result.error || new Error('Proxy fetch failed');
  }

  /**
   * Check if a URL is accessible directly
   */
  async checkDirectAccess(url: string): Promise<boolean> {
    try {
      const result = await this.attemptDirectFetch(url);
      return result.success;
    } catch {
      return false;
    }
  }

  /**
   * Get proxy statistics
   */
  async getStats(): Promise<{
    totalRequests: number;
    directSuccessRate: number;
    proxySuccessRate: number;
    cacheHitRate: number;
    avgResponseTime: number;
  }> {
    // This would integrate with a metrics system
    // For now, return placeholder values
    return {
      totalRequests: 0,
      directSuccessRate: 0,
      proxySuccessRate: 0,
      cacheHitRate: 0,
      avgResponseTime: 0,
    };
  }

  /**
   * Clear cache for a specific URL
   */
  async clearCache(url: string): Promise<void> {
    const cacheKey = this.getCacheKey(url);
    try {
      await redis.del(cacheKey);
      logger.info(`[ProxyManager] Cache cleared for ${url}`);
    } catch (error) {
      logger.warn(`[ProxyManager] Cache clear failed for ${url}`, { error });
    }
  }

  /**
   * Clear all proxy cache
   */
  async clearAllCache(): Promise<void> {
    try {
      const pattern = 'proxy:*';
      const keys = await redis.keys(pattern);
      if (keys.length > 0) {
        await redis.del(keys);
        logger.info(`[ProxyManager] All proxy cache cleared`, { keysCount: keys.length });
      }
    } catch (error) {
      logger.warn(`[ProxyManager] Failed to clear all proxy cache`, { error });
    }
  }
}

/**
 * Factory function to create proxy manager instances
 */
export function createProxyManager(config?: Partial<ProxyConfig>): ProxyManager {
  return new ProxyManager(config);
}

/**
 * Pre-configured proxy manager instances
 */
export const defaultProxyManager = createProxyManager();

export const aggressiveProxyManager = createProxyManager({
  preferDirect: false,
  fallbackEnabled: true,
  maxRetries: 5,
  timeout: 45000,
});

export const conservativeProxyManager = createProxyManager({
  preferDirect: true,
  fallbackEnabled: false,
  maxRetries: 2,
  timeout: 15000,
});

export default ProxyManager;
