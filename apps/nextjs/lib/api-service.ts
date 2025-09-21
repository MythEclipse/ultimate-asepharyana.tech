/**
 * Unified API Service Generator
 * Provides a generic, type-safe way to create API services with consistent error handling,
 * logging, and caching patterns across the application.
 */

import logger from '../utils/unified-logger';
import { HttpClient } from '../utils/unified-http-client';
import { toAppError, logError } from '../utils/error-handler';
import { redis } from './redis';

export interface ApiServiceConfig {
  baseUrl: string;
  serviceName: string;
  defaultTimeout?: number;
  enableCache?: boolean;
  cacheTtl?: number; // in seconds
  enableRetry?: boolean;
  maxRetries?: number;
  headers?: Record<string, string>;
}

export interface ApiRequestOptions {
  timeout?: number;
  cache?: boolean;
  cacheTtl?: number;
  retry?: boolean;
  headers?: Record<string, string>;
}

export class ApiService {
  private config: Required<ApiServiceConfig>;
  private httpClient: typeof HttpClient;

  constructor(config: ApiServiceConfig) {
    this.config = {
      defaultTimeout: 30000,
      enableCache: true,
      cacheTtl: 3600, // 1 hour default
      enableRetry: true,
      maxRetries: 3,
      headers: {},
      ...config,
    };
    this.httpClient = HttpClient;
  }

  /**
   * Generate cache key for a specific request
   */
  private getCacheKey(endpoint: string, params?: Record<string, any>): string {
    const paramsStr = params ? `:${JSON.stringify(params)}` : '';
    return `api:${this.config.serviceName}:${endpoint}${paramsStr}`;
  }

  /**
   * Generic GET method with caching and error handling
   */
  async get<T>(
    endpoint: string,
    params?: Record<string, any>,
    options?: ApiRequestOptions
  ): Promise<T> {
    const startTime = Date.now();
    const cacheKey = this.getCacheKey(endpoint, params);
    const shouldCache = options?.cache ?? this.config.enableCache;
    const cacheTtl = options?.cacheTtl ?? this.config.cacheTtl;

    try {
      // Check cache first
      if (shouldCache && redis) {
        try {
          const cached = await redis.get(cacheKey);
          if (cached) {
            logger.info(`[${this.config.serviceName}] Cache hit for ${endpoint}`, {
              endpoint,
              cacheKey,
              duration: Date.now() - startTime,
            });
            return JSON.parse(typeof cached === 'string' ? cached : JSON.stringify(cached));
          }
        } catch (error) {
          logger.warn(`[${this.config.serviceName}] Cache check failed for ${endpoint}:`, error);
        }
      }

      // Build URL with query parameters
      let url = `${this.config.baseUrl}${endpoint}`;
      if (params && Object.keys(params).length > 0) {
        const queryString = new URLSearchParams(params).toString();
        url += `?${queryString}`;
      }

      // Make HTTP request
      const timeout = options?.timeout ?? this.config.defaultTimeout;
      const headers = {
        ...this.config.headers,
        ...options?.headers,
      };

      const response = await this.httpClient.fetchJson<T>(url, {
        timeout,
        headers,
      });

      // Cache successful response
      if (shouldCache && redis) {
        try {
          await redis.set(cacheKey, JSON.stringify(response), {
            EX: cacheTtl,
          });
        } catch (error) {
          logger.warn(`[${this.config.serviceName}] Cache storage failed for ${endpoint}:`, error);
        }
      }

      logger.info(`[${this.config.serviceName}] GET ${endpoint} successful`, {
        endpoint,
        duration: Date.now() - startTime,
        cacheHit: false,
      });

      return response;
    } catch (error) {
      const duration = Date.now() - startTime;
      const appError = toAppError(error);
      logError(appError, { service: this.config.serviceName, endpoint, duration });

      throw appError;
    }
  }

  /**
   * Generic POST method with error handling
   */
  async post<T, R = T>(
    endpoint: string,
    data?: any,
    options?: ApiRequestOptions
  ): Promise<R> {
    const startTime = Date.now();

    try {
      const url = `${this.config.baseUrl}${endpoint}`;
      const timeout = options?.timeout ?? this.config.defaultTimeout;
      const headers = {
        'Content-Type': 'application/json',
        ...this.config.headers,
        ...options?.headers,
      };

      const response = await this.httpClient.request<R>(url, 'POST', data, undefined);

      logger.info(`[${this.config.serviceName}] POST ${endpoint} successful`, {
        endpoint,
        duration: Date.now() - startTime,
      });

      return response;
    } catch (error) {
      const duration = Date.now() - startTime;
      const appError = toAppError(error);
      logError(appError, { service: this.config.serviceName, endpoint, duration });

      throw appError;
    }
  }

  /**
   * Generic PUT method with error handling
   */
  async put<T, R = T>(
    endpoint: string,
    data?: any,
    options?: ApiRequestOptions
  ): Promise<R> {
    const startTime = Date.now();

    try {
      const url = `${this.config.baseUrl}${endpoint}`;
      const timeout = options?.timeout ?? this.config.defaultTimeout;
      const headers = {
        'Content-Type': 'application/json',
        ...this.config.headers,
        ...options?.headers,
      };

      const response = await this.httpClient.request<R>(url, 'PUT', data, undefined);

      logger.info(`[${this.config.serviceName}] PUT ${endpoint} successful`, {
        endpoint,
        duration: Date.now() - startTime,
      });

      // Invalidate cache for this endpoint
      await this.invalidateCache(endpoint);

      return response;
    } catch (error) {
      const duration = Date.now() - startTime;
      const appError = toAppError(error);
      logError(appError, { service: this.config.serviceName, endpoint, duration });

      throw appError;
    }
  }

  /**
   * Generic DELETE method with error handling
   */
  async delete<T>(
    endpoint: string,
    options?: ApiRequestOptions
  ): Promise<T> {
    const startTime = Date.now();

    try {
      const url = `${this.config.baseUrl}${endpoint}`;
      const timeout = options?.timeout ?? this.config.defaultTimeout;
      const headers = {
        ...this.config.headers,
        ...options?.headers,
      };

      const response = await this.httpClient.request<T>(url, 'DELETE', undefined, undefined);

      logger.info(`[${this.config.serviceName}] DELETE ${endpoint} successful`, {
        endpoint,
        duration: Date.now() - startTime,
      });

      // Invalidate cache for this endpoint
      await this.invalidateCache(endpoint);

      return response;
    } catch (error) {
      const duration = Date.now() - startTime;
      const appError = toAppError(error);
      logError(appError, { service: this.config.serviceName, endpoint, duration });

      throw appError;
    }
  }

  /**
   * Invalidate cache for specific endpoint
   */
  async invalidateCache(endpoint: string, params?: Record<string, any>): Promise<void> {
    if (!redis) {
      logger.warn(`[${this.config.serviceName}] Redis not available for cache invalidation`);
      return;
    }

    const cacheKey = this.getCacheKey(endpoint, params);
    try {
      await redis.del(cacheKey);
      logger.info(`[${this.config.serviceName}] Cache invalidated for ${endpoint}`, {
        endpoint,
        cacheKey,
      });
    } catch (error) {
      logger.warn(`[${this.config.serviceName}] Failed to invalidate cache`, {
        endpoint,
        cacheKey,
        error,
      });
    }
  }

  /**
   * Invalidate all cache for this service
   */
  async invalidateAllCache(): Promise<void> {
    if (!redis) {
      logger.warn(`[${this.config.serviceName}] Redis not available for cache invalidation`);
      return;
    }

    try {
      const pattern = `api:${this.config.serviceName}:*`;
      const keys = await redis.keys(pattern);
      if (keys.length > 0) {
        await redis.del(keys);
        logger.info(`[${this.config.serviceName}] All cache invalidated`, {
          keysCount: keys.length,
        });
      }
    } catch (error) {
      logger.warn(`[${this.config.serviceName}] Failed to invalidate all cache`, {
        error,
      });
    }
  }

  /**
   * Get service statistics
   */
  async getStats(): Promise<{
    cacheHitRate: number;
    totalRequests: number;
    avgResponseTime: number;
  }> {
    // This would typically integrate with a metrics system
    // For now, return placeholder values
    return {
      cacheHitRate: 0,
      totalRequests: 0,
      avgResponseTime: 0,
    };
  }
}

/**
 * Factory function to create API service instances
 */
export function createApiService(config: ApiServiceConfig): ApiService {
  return new ApiService(config);
}

/**
 * Pre-configured API service instances for common use cases
 */
export const animeApiService = createApiService({
  baseUrl: process.env.NEXT_PUBLIC_API_URL || 'https://api.asepharyana.tech',
  serviceName: 'anime',
  cacheTtl: 1800, // 30 minutes
});

export const komikApiService = createApiService({
  baseUrl: process.env.NEXT_PUBLIC_KOMIK_API_URL || 'https://komikindo.cz',
  serviceName: 'komik',
  cacheTtl: 3600, // 1 hour
});

export const sosmedApiService = createApiService({
  baseUrl: process.env.NEXT_PUBLIC_SOSMED_API_URL || 'https://sosmed.asepharyana.tech',
  serviceName: 'sosmed',
  cacheTtl: 300, // 5 minutes
});

export default ApiService;
