/**
 * Unified HTTP Client - Consolidated HTTP client with comprehensive functionality
 *
 * This client combines:
 * - Modern fetch-based HTTP requests (from UnifiedHttpClient)
 * - Proxy support with fallback mechanisms (from ProxyHttpClient)
 * - Image processing and proxy capabilities
 * - Authentication token management
 * - Error handling and retry logic
 * - Caching with Redis integration
 * - TypeScript generics for type safety
 */

import logger from './unified-logger';
import { DEFAULT_HEADERS } from './DHead';
import {
  HttpClientConfig,
  HttpResponse,
  HttpError,
  HttpMethod,
  FetchResult,
  ClientSideConfig,
  ServerSideConfig,
  ProxyConfig,
} from '../types/http';
import {
  getApiUrlConfig,
  buildUrl,
  getImageProxyUrlConfig,
  isValidUrl,
  sanitizeUrl,
} from './url-utils';
import {
  createHttpError,
  createTimeoutError,
  createNetworkError,
  handleFetchResponse,
  handleNetworkError,
  toAppError,
  withRetry,
  logError,
} from './error-handler';
import { ErrorCategory } from '../types/error';
import {
  ImageProcessingOptions,
  ImageProcessingResult,
  ImageFallbackOptions,
} from '../types/image';

// =============================================================================
// INTERFACES AND TYPES
// =============================================================================

export interface UnifiedHttpClientConfig extends HttpClientConfig {
  // Proxy configuration
  proxy?: {
    enabled?: boolean;
    fallback?: boolean;
    urls?: string[];
    preferDirect?: boolean; // Try direct first, then proxy
  };

  // Image processing
  imageProcessing?: {
    enabled?: boolean;
    validateContent?: boolean;
    allowedTypes?: string[];
    maxSize?: number;
  };

  // Advanced retry logic
  retry?: {
    enabled?: boolean;
    maxAttempts?: number;
    delay?: number;
    backoffMultiplier?: number;
    maxDelay?: number;
    retryOnNetworkError?: boolean;
    retryOnTimeout?: boolean;
    retryOn5xx?: boolean;
  };

  // Cache configuration
  cache?: {
    enabled?: boolean;
    ttl?: number;
    prefix?: string;
    redis?: boolean; // Use Redis for server-side caching
  };
}

export interface RequestConfig extends Omit<RequestInit, 'cache'> {
  timeout?: number;
  retry?: boolean;
  cache?: boolean;
  proxy?: boolean;
  validateStatus?: (status: number) => boolean;
}

export interface ProxyRequestResult<T = unknown> {
  success: boolean;
  data?: T;
  error?: Error;
  source: 'direct' | 'proxy' | 'cache' | 'fallback' | 'both' | 'config';
  responseTime?: number;
}

// =============================================================================
// UNIFIED HTTP CLIENT CLASS
// =============================================================================

export class UnifiedHttpClient {
  private config: UnifiedHttpClientConfig;
  private baseUrl: string;

  constructor(config: UnifiedHttpClientConfig = {}) {
    this.config = {
      baseUrl: '',
      timeout: 10000,
      headers: { 'Content-Type': 'application/json' },
      auth: { type: 'Bearer' },
      retry: {
        enabled: false,
        maxAttempts: 3,
        delay: 1000,
        backoffMultiplier: 2,
        maxDelay: 30000,
        retryOnNetworkError: true,
        retryOnTimeout: true,
        retryOn5xx: true,
      },
      cache: {
        enabled: false,
        ttl: 120,
        prefix: 'http:cache:',
        redis: typeof window === 'undefined', // Redis only on server-side
      },
      proxy: {
        enabled: false,
        fallback: true,
        preferDirect: true,
        urls: [],
      },
      imageProcessing: {
        enabled: false,
        validateContent: true,
        allowedTypes: ['image/jpeg', 'image/jpg', 'image/png', 'image/gif', 'image/webp'],
        maxSize: 10 * 1024 * 1024, // 10MB
      },
      ...config,
    };

    this.baseUrl = config.baseUrl || '';
  }

  // =============================================================================
  // URL BUILDING AND VALIDATION
  // =============================================================================

  static buildUrl(base: string, path: string): string {
    return buildUrl(base, path);
  }

  private buildFullUrl(path: string): string {
    if (path.startsWith('http://') || path.startsWith('https://')) {
      return path;
    }
    return this.baseUrl ? buildUrl(this.baseUrl, path) : path;
  }

  // =============================================================================
  // AUTHENTICATION AND HEADERS
  // =============================================================================

  private createHeaders(customHeaders?: Record<string, string>, token?: string | null): Headers {
    const headers = new Headers({
      ...this.config.headers,
      ...customHeaders,
    });

    if (token && this.config.auth?.type === 'Bearer') {
      headers.set('Authorization', `Bearer ${token}`);
    }

    return headers;
  }

  // =============================================================================
  // CACHE MANAGEMENT
  // =============================================================================

  private getCacheKey(url: string, method = 'GET'): string {
    const prefix = this.config.cache?.prefix || 'http:cache:';
    return `${prefix}${method}:${url}`;
  }

  private async getCachedResponse<T>(url: string, method = 'GET'): Promise<T | null> {
    if (!this.config.cache?.enabled) return null;

    const key = this.getCacheKey(url, method);

    try {
      if (this.config.cache?.redis && typeof window === 'undefined') {
        // Server-side Redis caching
        const { redis } = await import('../lib/redis');
        const cached = await redis.get(key);
        if (cached) {
          logger.info(`[UnifiedHttpClient] Cache hit for ${url}`);
          return JSON.parse(typeof cached === 'string' ? cached : JSON.stringify(cached));
        }
      } else {
        // Client-side memory/session storage would go here
        // For now, we'll skip client-side caching
      }
    } catch (error) {
      logger.warn(`[UnifiedHttpClient] Cache retrieval failed for ${url}:`, error);
    }

    return null;
  }

  private async setCachedResponse<T>(url: string, data: T, method = 'GET'): Promise<void> {
    if (!this.config.cache?.enabled) return;

    const key = this.getCacheKey(url, method);
    const ttl = this.config.cache?.ttl || 120;

    try {
      if (this.config.cache?.redis && typeof window === 'undefined') {
        // Server-side Redis caching
        const { redis } = await import('../lib/redis');
        await redis.set(key, JSON.stringify(data), { EX: ttl });
        logger.info(`[UnifiedHttpClient] Cached response for ${url}`);
      } else {
        // Client-side caching would go here
      }
    } catch (error) {
      logger.warn(`[UnifiedHttpClient] Cache storage failed for ${url}:`, error);
    }
  }

  // =============================================================================
  // PROXY AND FALLBACK LOGIC
  // =============================================================================

  /**
   * Check if the response data contains Internet Baik block page indicators
   */
  private isInternetBaikBlockPage(data: string | object): boolean {
    if (typeof data !== 'string') return false;
    return (
      data.includes('internetbaik.telkomsel.com') ||
      data.includes('VmaxAdManager.js') ||
      data.includes('VmaxAdHelper')
    );
  }

  /**
   * Attempt direct fetch with axios (for proxy functionality)
   */
  private async attemptDirectFetch(url: string, options: RequestConfig = {}): Promise<Response> {
    const headers = { ...DEFAULT_HEADERS, ...(options.headers as Record<string, string>) };
    const response = await fetch(url, {
      headers,
      signal: AbortSignal.timeout(options.timeout || this.config.timeout || 10000),
    });
    return response;
  }

  /**
   * Attempt Croxy Proxy fetch (server-side only)
   */
  private async attemptProxyFetch(url: string): Promise<string> {
    if (typeof window !== 'undefined') {
      throw new Error('Proxy fetch is only available on server-side');
    }

    try {
      // Dynamic import to avoid browser issues
      const { scrapeCroxyProxy } = await import('../lib/scrapeCroxyProxy');
      logger.info(`[UnifiedHttpClient] Using scrapeCroxyProxy for ${url}`);
      return scrapeCroxyProxy(url);
    } catch (error) {
      logger.error(`[UnifiedHttpClient] Failed to import scrapeCroxyProxy:`, error);
      throw new Error('Proxy functionality not available');
    }
  }

  /**
   * Fetch with proxy support and fallback logic
   */
  private async fetchWithProxySupport<T = string>(
    url: string,
    options: RequestConfig = {}
  ): Promise<ProxyRequestResult<T>> {
    const startTime = Date.now();

    // Check cache first
    const cached = await this.getCachedResponse<T>(url);
    if (cached) {
      return {
        success: true,
        data: cached,
        source: 'cache',
        responseTime: Date.now() - startTime,
      };
    }

    const proxyConfig = this.config.proxy;

    if (!proxyConfig?.enabled) {
      // No proxy, use direct fetch
      try {
        const response = await this.attemptDirectFetch(url, options);
        const responseTime = Date.now() - startTime;

        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }

        const data = await response.text();

        if (this.isInternetBaikBlockPage(data)) {
          throw new Error('Content blocked by Internet Baik');
        }

        const result = {
          success: true,
          data: data as T,
          source: 'direct' as const,
          responseTime,
        };

        await this.setCachedResponse(url, result.data);
        return result;
      } catch (error) {
        return {
          success: false,
          error: error as Error,
          source: 'direct',
          responseTime: Date.now() - startTime,
        };
      }
    }

    // Proxy is enabled
    if (proxyConfig.preferDirect) {
      // Try direct first, then proxy
      try {
        const response = await this.attemptDirectFetch(url, options);

        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }

        const data = await response.text();

        if (!this.isInternetBaikBlockPage(data)) {
          const responseTime = Date.now() - startTime;
          const result = {
            success: true,
            data: data as T,
            source: 'direct' as const,
            responseTime,
          };

          await this.setCachedResponse(url, result.data);
          return result;
        }

        logger.warn(`[UnifiedHttpClient] Direct fetch blocked, trying proxy for ${url}`);
      } catch (error) {
        logger.warn(`[UnifiedHttpClient] Direct fetch failed for ${url}:`, error);
      }
    }

    // Try proxy
    if (proxyConfig.fallback) {
      try {
        const proxyData = await this.attemptProxyFetch(url);
        const responseTime = Date.now() - startTime;

        const result = {
          success: true,
          data: proxyData as T,
          source: 'proxy' as const,
          responseTime,
        };

        await this.setCachedResponse(url, result.data);
        return result;
      } catch (proxyError) {
        logger.error(`[UnifiedHttpClient] Proxy fetch failed for ${url}:`, proxyError);

        if (proxyConfig.preferDirect) {
          // Already tried direct, return proxy error
          return {
            success: false,
            error: proxyError as Error,
            source: 'proxy',
            responseTime: Date.now() - startTime,
          };
        } else {
          // Try direct as fallback
          try {
            const response = await this.attemptDirectFetch(url, options);
            const responseTime = Date.now() - startTime;

            if (!response.ok) {
              throw new Error(`HTTP ${response.status}`);
            }

            const data = await response.text();

            const result = {
              success: true,
              data: data as T,
              source: 'direct' as const,
              responseTime,
            };

            await this.setCachedResponse(url, result.data);
            return result;
          } catch (directError) {
            return {
              success: false,
              error: proxyError as Error,
              source: 'both',
              responseTime: Date.now() - startTime,
            };
          }
        }
      }
    }

    return {
      success: false,
      error: new Error('Proxy configuration error'),
      source: 'config',
      responseTime: Date.now() - startTime,
    };
  }

  // =============================================================================
  // CORE HTTP METHODS
  // =============================================================================

  private async executeRequest<T = unknown>(
    url: string,
    options: RequestConfig = {},
    method: HttpMethod = 'GET',
    body?: unknown
  ): Promise<T> {
    const fullUrl = this.buildFullUrl(url);
    const requestOptions: RequestInit = {
      method,
      headers: this.createHeaders(options.headers as Record<string, string>, this.config.auth?.token),
      signal: options.signal || AbortSignal.timeout(options.timeout || this.config.timeout || 10000),
    };

    if (body) {
      requestOptions.body = JSON.stringify(body);
    }

    // Check cache first for GET requests
    if (method === 'GET' && this.config.cache?.enabled) {
      const cached = await this.getCachedResponse<T>(fullUrl, method);
      if (cached) return cached;
    }

    const operation = async () => {
      const response = await fetch(fullUrl, requestOptions);

      if (!response.ok && !options.validateStatus?.(response.status)) {
        throw createHttpError(`HTTP ${response.status}`, response.status, {
          statusText: response.statusText,
          url: fullUrl,
          context: { url: fullUrl, method },
        });
      }

      const result = await this.handleResponse<T>(response, fullUrl);

      // Cache successful GET responses
      if (method === 'GET' && response.ok) {
        await this.setCachedResponse(fullUrl, result, method);
      }

      return result;
    };

    // Apply retry logic if enabled
    if (this.config.retry?.enabled) {
      return withRetry(operation, {
        enabled: true,
        maxAttempts: this.config.retry.maxAttempts || 3,
        delayMs: this.config.retry.delay || 1000,
        backoffMultiplier: this.config.retry.backoffMultiplier || 2,
        maxDelayMs: this.config.retry.maxDelay || 30000,
      });
    }

    return operation();
  }

  private async handleResponse<T>(response: Response, url: string): Promise<T> {
    try {
      await handleFetchResponse(response, url);
      return response.json() as T;
    } catch (error) {
      const appError = toAppError(error, { url, method: 'handleResponse' });
      logError(appError);
      throw appError;
    }
  }

  // =============================================================================
  // PUBLIC API METHODS
  // =============================================================================

  async fetchJson<T = unknown>(url: string, options: RequestConfig = {}): Promise<T> {
    return this.executeRequest<T>(url, options, 'GET');
  }

  async fetchWithAuth<T = unknown>(
    url: string,
    token?: string | null,
    options: RequestConfig = {}
  ): Promise<T> {
    const headers = this.createHeaders(options.headers as Record<string, string>, token);
    return this.executeRequest<T>(url, { ...options, headers }, 'GET');
  }

  async request<T = unknown>(
    url: string,
    method: HttpMethod,
    body?: unknown,
    token?: string | null,
    options: RequestConfig = {}
  ): Promise<T> {
    const headers = this.createHeaders(options.headers as Record<string, string>, token);
    return this.executeRequest<T>(url, { ...options, headers }, method, body);
  }

  async post<T = unknown>(url: string, body?: unknown, token?: string | null, options: RequestConfig = {}): Promise<T> {
    return this.request<T>(url, 'POST', body, token, options);
  }

  async put<T = unknown>(url: string, body?: unknown, token?: string | null, options: RequestConfig = {}): Promise<T> {
    return this.request<T>(url, 'PUT', body, token, options);
  }

  async patch<T = unknown>(url: string, body?: unknown, token?: string | null, options: RequestConfig = {}): Promise<T> {
    return this.request<T>(url, 'PATCH', body, token, options);
  }

  async delete<T = unknown>(url: string, token?: string | null, options: RequestConfig = {}): Promise<T> {
    return this.request<T>(url, 'DELETE', undefined, token, options);
  }

  // =============================================================================
  // PROXY-SPECIFIC METHODS
  // =============================================================================

  async fetchWithProxy<T = string>(url: string, options: RequestConfig = {}): Promise<T> {
    const result = await this.fetchWithProxySupport<T>(url, options);

    if (!result.success) {
      throw result.error || new Error('Proxy fetch failed');
    }

    return result.data as T;
  }

  async fetchWithProxyOnly<T = string>(url: string, options: RequestConfig = {}): Promise<T> {
    const proxyConfig = { ...this.config.proxy, preferDirect: false };
    const tempConfig = { ...this.config, proxy: proxyConfig };
    const tempClient = new UnifiedHttpClient(tempConfig);

    return tempClient.fetchWithProxy<T>(url, options);
  }

  // =============================================================================
  // IMAGE PROCESSING METHODS
  // =============================================================================

  async processImage(
    url: string,
    fallbackOptions: ImageFallbackOptions = {},
    processingOptions: ImageProcessingOptions = {}
  ): Promise<ImageProcessingResult> {
    if (!this.config.imageProcessing?.enabled) {
      // Fallback to basic image processing
      return this.basicImageProcessing(url, fallbackOptions, processingOptions);
    }

    // Use advanced image processing with proxy support
    return this.advancedImageProcessing(url, fallbackOptions, processingOptions);
  }

  private async basicImageProcessing(
    url: string,
    fallbackOptions: ImageFallbackOptions = {},
    processingOptions: ImageProcessingOptions = {}
  ): Promise<ImageProcessingResult> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), processingOptions.timeout || 10000);

    try {
      const response = await fetch(url, {
        method: 'GET',
        signal: controller.signal,
        headers: processingOptions.headers || {},
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        return {
          success: false,
          url,
          source: 'direct',
          error: `HTTP ${response.status}`,
        };
      }

      const contentType = response.headers.get('content-type');
      if (!contentType || !contentType.startsWith('image/')) {
        return {
          success: false,
          url,
          source: 'direct',
          error: 'Not an image',
        };
      }

      const arrayBuffer = await response.arrayBuffer();

      return {
        success: true,
        url,
        source: 'direct',
        contentType,
        arrayBuffer,
      };
    } catch (error) {
      clearTimeout(timeoutId);
      logger.error(`[UnifiedHttpClient] Basic image processing failed:`, error);
      return {
        success: false,
        url,
        source: 'direct',
        error: (error as Error).message,
      };
    }
  }

  private async advancedImageProcessing(
    url: string,
    fallbackOptions: ImageFallbackOptions = {},
    processingOptions: ImageProcessingOptions = {}
  ): Promise<ImageProcessingResult> {
    // This would integrate with the existing image-proxy.ts functionality
    // For now, we'll use the basic processing as a starting point
    return this.basicImageProcessing(url, fallbackOptions, processingOptions);
  }

  // =============================================================================
  // STATIC FACTORY METHODS
  // =============================================================================

  static createClientSide(config: ClientSideConfig = {}): UnifiedHttpClient {
    const apiConfig = getApiUrlConfig();
    return new UnifiedHttpClient({
      baseUrl: apiConfig.client,
      cache: { enabled: false, redis: false }, // No Redis on client-side
      proxy: { enabled: false },
      ...config,
    });
  }

  static createServerSide(config: ServerSideConfig = {}): UnifiedHttpClient {
    const apiConfig = getApiUrlConfig();
    return new UnifiedHttpClient({
      baseUrl: apiConfig.server,
      cache: { enabled: true, redis: true }, // Enable Redis on server-side
      ...config,
    });
  }

  static createProxyClient(config: ProxyConfig = {}): UnifiedHttpClient {
    return new UnifiedHttpClient({
      proxy: { enabled: true, fallback: true, preferDirect: true },
      cache: { enabled: true, ttl: 300 }, // 5 minutes cache for proxy
      ...config,
    });
  }

  static createImageProxyClient(config: UnifiedHttpClientConfig = {}): UnifiedHttpClient {
    return new UnifiedHttpClient({
      imageProcessing: {
        enabled: true,
        validateContent: true,
        allowedTypes: ['image/jpeg', 'image/jpg', 'image/png', 'image/gif', 'image/webp'],
        maxSize: 10 * 1024 * 1024, // 10MB
      },
      proxy: { enabled: true, fallback: true },
      timeout: 30000, // 30 seconds for image processing
      ...config,
    });
  }
}

// =============================================================================
// CONVENIENCE FUNCTIONS (BACKWARD COMPATIBILITY)
// =============================================================================

export const createHttpClient = (config?: UnifiedHttpClientConfig) => new UnifiedHttpClient(config);

export const clientSideFetch = async <T = unknown>(
  url: string,
  token?: string | null,
  options: RequestConfig = {}
): Promise<T> => {
  const client = UnifiedHttpClient.createClientSide();
  const fullUrl = client['baseUrl'] ? UnifiedHttpClient.buildUrl(client['baseUrl'], url) : url;
  return client.fetchWithAuth<T>(fullUrl, token, options);
};

export const serverSideFetch = async <T = unknown>(
  url: string,
  options: RequestConfig = {}
): Promise<T> => {
  const client = UnifiedHttpClient.createServerSide();
  const fullUrl = client['baseUrl'] ? UnifiedHttpClient.buildUrl(client['baseUrl'], url) : url;
  return client.fetchJson<T>(fullUrl, options);
};

export const fetchWithProxy = async (url: string): Promise<FetchResult> => {
  const client = UnifiedHttpClient.createProxyClient();
  const data = await client.fetchWithProxy<string>(url);
  return { data, contentType: 'text/html' };
};

export const fetchWithProxyOnly = async (url: string): Promise<FetchResult> => {
  const client = UnifiedHttpClient.createProxyClient();
  const data = await client.fetchWithProxyOnly<string>(url);
  return { data, contentType: 'text/html' };
};

// =============================================================================
// LEGACY COMPATIBILITY EXPORTS
// =============================================================================

export const HttpClient = {
  buildUrl: UnifiedHttpClient.buildUrl,
  fetchJson: async <T = unknown>(url: string, options: RequestConfig = {}) => {
    const client = new UnifiedHttpClient();
    return client.fetchJson<T>(url, options);
  },
  fetchWithAuth: async <T = unknown>(url: string, token?: string | null, options: RequestConfig = {}) => {
    const client = new UnifiedHttpClient();
    return client.fetchWithAuth<T>(url, token, options);
  },
  request: async <T = unknown>(url: string, method: string, body?: unknown, token?: string | null) => {
    const client = new UnifiedHttpClient();
    return client.request<T>(url, method as HttpMethod, body, token);
  },
};

// Export types for backward compatibility
export type {
  UnifiedHttpClientConfig as HttpClientConfig,
  RequestConfig as RequestOptions,
  ProxyRequestResult as FetchResult,
} from './unified-http-client';

// Additional exports for proxy client compatibility
export const CroxyProxyOnly = fetchWithProxyOnly;

export interface CustomError extends Error {
  code?: string;
}

// Re-export ProxyHttpClient as UnifiedHttpClient for backward compatibility
export const ProxyHttpClient = UnifiedHttpClient;
