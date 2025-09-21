import logger from './logger';
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
    return url.startsWith('/') ? `${base}${url}` : url;
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
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({ message: response.statusText }));
      const error: HttpError = new Error(errorData.message || 'Request failed');
      error.status = response.status;
      error.statusText = response.statusText;
      error.url = url;
      error.response = response;

      logger.error(`HTTP Error: ${url} - Status: ${response.status}, Message: ${error.message}`);
      throw error;
    }

    return response.json() as T;
  }

  async fetchJson<T = unknown>(
    url: string,
    options: RequestInit = {},
    configOverride?: HttpClientConfig
  ): Promise<T> {
    const config = { ...this.config, ...configOverride };
    const response = await fetch(url, {
      ...options,
      headers: this.createHeaders(
        options.headers as Record<string, string>,
        config.auth?.token
      ),
      signal: this.createAbortSignal(config.timeout),
    });

    return this.handleResponse<T>(response, url);
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
    return new UnifiedHttpClient({
      baseUrl: process.env.NEXT_PUBLIC_API_URL || 'https://ws.asepharyana.tech',
      ...config,
    });
  }

  static createServerSide(config: ServerSideConfig = {}): UnifiedHttpClient {
    return new UnifiedHttpClient({
      baseUrl: process.env.API_URL_SERVER || 'http://localhost:4091',
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
