import logger from './unified-logger';
import {
  HttpClientConfig,
  HttpResponse,
  HttpError,
  RequestOptions,
  FetchResult,
  HttpMethod,
  ClientSideConfig,
  ServerSideConfig,
  ProxyConfig,
} from '../types/http';
import { getApiUrlConfig, buildUrl } from './url-utils';
import {
  createHttpError,
  createTimeoutError,
  handleFetchResponse,
  handleNetworkError,
  toAppError,
  withRetry,
  logError,
} from './error-handler';
import { ErrorCategory } from '../types/error';

export class UnifiedHttpClient {
  protected config: HttpClientConfig;

  constructor(config: HttpClientConfig = {}) {
    this.config = {
      timeout: 10000,
      headers: { 'Content-Type': 'application/json' },
      auth: { type: 'Bearer' },
      retry: { enabled: false, maxAttempts: 3, delay: 1000 },
      cache: { enabled: false, ttl: 120 },
      proxy: { enabled: false, fallback: false },
      ...config,
    };
  }

  static buildUrl(base: string, url: string): string {
    return buildUrl(base, url);
  }

  private createAbortSignal(timeout?: number): AbortSignal {
    const timeoutMs = timeout || this.config.timeout || 10000;
    return AbortSignal.timeout(timeoutMs);
  }

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

  private async handleResponse<T>(response: Response, url: string): Promise<T> {
    try {
      await handleFetchResponse(response, url);
      return response.json() as T;
    } catch (error) {
      // Log the error using centralized logging
      const appError = toAppError(error, { url, method: 'handleResponse' });
      logError(appError);
      throw appError;
    }
  }

  async fetchJson<T = unknown>(
    url: string,
    options: RequestInit = {},
    configOverride?: HttpClientConfig
  ): Promise<T> {
    const config = { ...this.config, ...configOverride };

    // Implement retry logic if enabled
    const operation = async () => {
      const response = await fetch(url, {
        ...options,
        headers: this.createHeaders(
          options.headers as Record<string, string>,
          config.auth?.token
        ),
        signal: this.createAbortSignal(config.timeout),
      });

      return this.handleResponse<T>(response, url);
    };

    if (config.retry?.enabled) {
      return withRetry(operation, {
        enabled: true,
        maxAttempts: config.retry.maxAttempts || 3,
        delayMs: config.retry.delay || 1000,
        backoffMultiplier: 2,
        maxDelayMs: 30000,
      });
    }

    return operation();
  }

  async fetchWithAuth<T = unknown>(
    url: string,
    token?: string | null,
    options: RequestInit = {},
    configOverride?: HttpClientConfig
  ): Promise<T> {
    const headers = this.createHeaders(options.headers as Record<string, string>, token);

    return this.fetchJson<T>(url, {
      ...options,
      headers,
    }, configOverride);
  }

  async request<T = unknown>(
    url: string,
    method: HttpMethod,
    body?: unknown,
    token?: string | null,
    configOverride?: HttpClientConfig
  ): Promise<T> {
    const options: RequestInit = { method };

    if (body) {
      options.body = JSON.stringify(body);
    }

    return this.fetchWithAuth<T>(url, token, options, configOverride);
  }

  // Static factory methods for different use cases
  static createClientSide(config: ClientSideConfig = {}): UnifiedHttpClient {
    const apiConfig = getApiUrlConfig();
    return new UnifiedHttpClient({
      baseUrl: apiConfig.client,
      ...config,
    });
  }

  static createServerSide(config: ServerSideConfig = {}): UnifiedHttpClient {
    const apiConfig = getApiUrlConfig();
    return new UnifiedHttpClient({
      baseUrl: apiConfig.server,
      ...config,
    });
  }

  static createProxyClient(config: ProxyConfig = {}): UnifiedHttpClient {
    return new UnifiedHttpClient({
      proxy: { enabled: true, fallback: true },
      ...config,
    });
  }
}

// Convenience functions that maintain backward compatibility
export const createHttpClient = (config?: HttpClientConfig) => new UnifiedHttpClient(config);

export const clientSideFetch = async <T = unknown>(
  url: string,
  token?: string | null,
  options: RequestInit = {}
): Promise<T> => {
  const client = UnifiedHttpClient.createClientSide();
  const fullUrl = client['config']?.baseUrl ? UnifiedHttpClient.buildUrl(client['config'].baseUrl, url) : url;
  return client.fetchWithAuth<T>(fullUrl, token, options);
};

export const serverSideFetch = async <T = unknown>(
  url: string,
  options: RequestInit = {}
): Promise<T> => {
  const client = UnifiedHttpClient.createServerSide();
  const fullUrl = client['config']?.baseUrl ? UnifiedHttpClient.buildUrl(client['config'].baseUrl, url) : url;
  return client.fetchJson<T>(fullUrl, options);
};

// Backward compatibility exports
export const HttpClient = {
  buildUrl: UnifiedHttpClient.buildUrl,
  fetchJson: async <T = unknown>(url: string, options: RequestInit = {}) => {
    const client = new UnifiedHttpClient();
    return client.fetchJson<T>(url, options);
  },
  fetchWithAuth: async <T = unknown>(url: string, token?: string | null, options: RequestInit = {}) => {
    const client = new UnifiedHttpClient();
    return client.fetchWithAuth<T>(url, token, options);
  },
  request: async <T = unknown>(url: string, method: string, body?: unknown, token?: string | null) => {
    const client = new UnifiedHttpClient();
    return client.request<T>(url, method as HttpMethod, body, token);
  },
};
