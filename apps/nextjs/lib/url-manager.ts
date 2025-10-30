/**
 * Unified URL Manager
 * Provides centralized URL management with validation, building, and caching
 */

import logger from '../utils/unified-logger';
import { redis } from './redis';
import { buildUrl, sanitizeUrl, isValidUrl } from '../utils/url-utils';
import { toAppError, logError } from '../utils/error-handler';

export interface UrlConfig {
  baseUrl: string;
  validateSsl: boolean;
  maxRedirects: number;
  timeout: number;
  cacheTtl: number;
}

export interface UrlBuildOptions {
  params?: Record<string, string | number | boolean>;
  query?: Record<string, string | number | boolean | undefined>;
  fragment?: string;
}

export interface UrlValidationResult {
  isValid: boolean;
  errors: string[];
  warnings: string[];
  suggestions: string[];
}

export class UrlManager {
  private config: UrlConfig;

  constructor(baseUrl: string, config: Partial<UrlConfig> = {}) {
    this.config = {
      baseUrl: baseUrl.replace(/\/$/, ''), // Remove trailing slash
      validateSsl: true,
      maxRedirects: 5,
      timeout: 10000,
      cacheTtl: 3600, // 1 hour
      ...config,
    };
  }

  /**
   * Generate cache key for URL operations
   */
  private getCacheKey(operation: string, key: string): string {
    return `url:${operation}:${key}`;
  }

  /**
   * Build a complete URL with path, query parameters, and fragment
   */
  buildUrl(path: string, options: UrlBuildOptions = {}): string {
    try {
      let url = buildUrl(this.config.baseUrl, path);

      // Add query parameters
      if (options.query && Object.keys(options.query).length > 0) {
        const queryParams = new URLSearchParams();
        Object.entries(options.query).forEach(([key, value]) => {
          if (value !== undefined && value !== null) {
            queryParams.append(key, String(value));
          }
        });
        url += `?${queryParams.toString()}`;
      }

      // Add fragment
      if (options.fragment) {
        url += `#${options.fragment}`;
      }

      return url;
    } catch (error) {
      logError(toAppError(error), {
        operation: 'buildUrl',
        baseUrl: this.config.baseUrl,
        path,
      });
      throw error;
    }
  }

  /**
   * Validate a URL and return detailed validation results
   */
  validateUrl(url: string): UrlValidationResult {
    const result: UrlValidationResult = {
      isValid: true,
      errors: [],
      warnings: [],
      suggestions: [],
    };

    try {
      // Basic URL validation
      if (!isValidUrl(url)) {
        result.isValid = false;
        result.errors.push('Invalid URL format');
        return result;
      }

      const urlObj = new URL(url);

      // Protocol validation
      const validProtocols = ['http:', 'https:'];
      if (!validProtocols.includes(urlObj.protocol)) {
        result.isValid = false;
        result.errors.push(`Unsupported protocol: ${urlObj.protocol}`);
      }

      // SSL validation
      if (this.config.validateSsl && urlObj.protocol === 'http:') {
        result.warnings.push('Using HTTP instead of HTTPS');
        result.suggestions.push('Consider using HTTPS for better security');
      }

      // Domain validation
      if (!urlObj.hostname) {
        result.isValid = false;
        result.errors.push('Missing hostname');
      }

      // Path validation
      if (urlObj.pathname.includes('//')) {
        result.warnings.push('Double slash in path');
        result.suggestions.push('Remove duplicate slashes from the path');
      }

      // Query validation
      if (urlObj.search && urlObj.search.length > 2048) {
        result.warnings.push('Very long query string');
        result.suggestions.push('Consider using POST for large data');
      }

      return result;
    } catch (_error) {
      result.isValid = false;
      result.errors.push('URL parsing failed');
      return result;
    }
  }

  /**
   * Sanitize a URL to remove potentially harmful components
   */
  sanitizeUrl(url: string): string {
    try {
      return sanitizeUrl(url);
    } catch (error) {
      logError(toAppError(error), { operation: 'sanitizeUrl', url });
      return url; // Return original URL if sanitization fails
    }
  }

  /**
   * Check if a URL is accessible (makes a HEAD request)
   */
  async checkUrl(url: string): Promise<boolean> {
    const cacheKey = this.getCacheKey('check', url);

    // Check cache first
    try {
      const cached = await redis.get(cacheKey);
      if (cached !== null) {
        logger.info(`[UrlManager] Cache hit for URL check: ${url}`);
        return cached === 'true';
      }
    } catch (error) {
      logger.warn(`[UrlManager] Cache check failed for URL: ${url}`, { error });
    }

    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(
        () => controller.abort(),
        this.config.timeout,
      );

      const response = await fetch(url, {
        method: 'HEAD',
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      const isAccessible = response.ok;

      // Cache result
      try {
        await redis.set(cacheKey, String(isAccessible), {
          EX: this.config.cacheTtl,
        });
      } catch (error) {
        logger.warn(`[UrlManager] Cache storage failed for URL check: ${url}`, {
          error,
        });
      }

      return isAccessible;
    } catch (error) {
      logError(toAppError(error), { operation: 'checkUrl', url });

      // Cache failure result for shorter time
      try {
        await redis.set(cacheKey, 'false', {
          EX: Math.min(this.config.cacheTtl, 300),
        });
      } catch (cacheError) {
        logger.warn(
          `[UrlManager] Cache storage failed for URL check failure: ${url}`,
          { cacheError },
        );
      }

      return false;
    }
  }

  /**
   * Extract domain from URL
   */
  extractDomain(url: string): string {
    try {
      const urlObj = new URL(url);
      return urlObj.hostname;
    } catch (error) {
      logError(toAppError(error), { operation: 'extractDomain', url });
      return '';
    }
  }

  /**
   * Extract path from URL
   */
  extractPath(url: string): string {
    try {
      const urlObj = new URL(url);
      return urlObj.pathname;
    } catch (error) {
      logError(toAppError(error), { operation: 'extractPath', url });
      return '';
    }
  }

  /**
   * Extract query parameters from URL
   */
  extractQueryParams(url: string): Record<string, string> {
    try {
      const urlObj = new URL(url);
      const params: Record<string, string> = {};
      urlObj.searchParams.forEach((value, key) => {
        params[key] = value;
      });
      return params;
    } catch (error) {
      logError(toAppError(error), { operation: 'extractQueryParams', url });
      return {};
    }
  }

  /**
   * Compare two URLs for equality (ignoring query order and fragments)
   */
  areUrlsEqual(url1: string, url2: string): boolean {
    try {
      const obj1 = new URL(url1);
      const obj2 = new URL(url2);

      // Compare base components
      if (obj1.protocol !== obj2.protocol) return false;
      if (obj1.hostname !== obj2.hostname) return false;
      if (obj1.pathname !== obj2.pathname) return false;
      if (obj1.port !== obj2.port) return false;

      // Compare query parameters (order-independent)
      const params1 = this.extractQueryParams(url1);
      const params2 = this.extractQueryParams(url2);

      const keys1 = Object.keys(params1).sort();
      const keys2 = Object.keys(params2).sort();

      if (keys1.length !== keys2.length) return false;

      for (const key of keys1) {
        if (params1[key] !== params2[key]) return false;
      }

      return true;
    } catch (error) {
      logError(toAppError(error), { operation: 'areUrlsEqual', url1, url2 });
      return false;
    }
  }

  /**
   * Get URL manager statistics
   */
  async getStats(): Promise<{
    totalUrlsProcessed: number;
    validationSuccessRate: number;
    avgResponseTime: number;
    cacheHitRate: number;
  }> {
    // This would integrate with a metrics system
    // For now, return placeholder values
    return {
      totalUrlsProcessed: 0,
      validationSuccessRate: 0,
      avgResponseTime: 0,
      cacheHitRate: 0,
    };
  }

  /**
   * Clear URL cache for a specific URL
   */
  async clearCache(url: string): Promise<void> {
    const operations = ['check', 'validate'];

    for (const operation of operations) {
      const cacheKey = this.getCacheKey(operation, url);
      try {
        await redis.del(cacheKey);
      } catch (error) {
        logger.warn(
          `[UrlManager] Failed to clear ${operation} cache for ${url}`,
          { error },
        );
      }
    }

    logger.info(`[UrlManager] Cache cleared for ${url}`);
  }

  /**
   * Clear all URL cache
   */
  async clearAllCache(): Promise<void> {
    try {
      const pattern = 'url:*';
      const keys = await redis.keys(pattern);
      if (keys.length > 0) {
        await redis.del(keys);
        logger.info(`[UrlManager] All URL cache cleared`, {
          keysCount: keys.length,
        });
      }
    } catch (error) {
      logger.warn(`[UrlManager] Failed to clear all URL cache`, { error });
    }
  }
}

/**
 * Factory function to create URL manager instances
 */
export function createUrlManager(
  baseUrl: string,
  config?: Partial<UrlConfig>,
): UrlManager {
  return new UrlManager(baseUrl, config);
}

/**
 * Pre-configured URL manager instances for common use cases
 */
export const animeUrlManager = createUrlManager(
  'https://api.asepharyana.tech',
  {
    cacheTtl: 1800, // 30 minutes
    validateSsl: true,
  },
);

export const komikUrlManager = createUrlManager('https://komikindo.cz', {
  cacheTtl: 3600, // 1 hour
  validateSsl: false, // Some komik sites don't have SSL
});

export const sosmedUrlManager = createUrlManager(
  'https://sosmed.asepharyana.tech',
  {
    cacheTtl: 300, // 5 minutes
    validateSsl: true,
  },
);

export default UrlManager;
