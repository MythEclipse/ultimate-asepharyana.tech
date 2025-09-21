export interface HttpClientConfig {
  baseUrl?: string;
  timeout?: number;
  headers?: Record<string, string>;
  auth?: {
    token?: string | null;
    type?: 'Bearer' | 'Basic';
  };
  retry?: {
    enabled?: boolean;
    maxAttempts?: number;
    delay?: number;
  };
  cache?: {
    enabled?: boolean;
    ttl?: number; // Time to live in seconds
  };
  proxy?: {
    enabled?: boolean;
    fallback?: boolean;
  };
}

export interface HttpResponse<T = unknown> {
  data: T;
  status: number;
  headers: Headers;
  contentType?: string | null;
}

export interface FetchResult {
  data: string | object;
  contentType: string | null;
}

export interface HttpError extends Error {
  status?: number;
  statusText?: string;
  url?: string;
  response?: Response;
}

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH' | 'HEAD' | 'OPTIONS';

export interface RequestOptions extends Omit<RequestInit, 'method' | 'body' | 'cache'> {
  timeout?: number;
  retry?: boolean;
  cache?: boolean;
  proxy?: boolean;
}

export interface ClientSideConfig extends HttpClientConfig {
  tokenSource?: 'localStorage' | 'sessionStorage' | 'cookie';
}

export interface ServerSideConfig extends HttpClientConfig {
  userAgent?: string;
  serverTimeout?: number;
}

export interface ProxyConfig extends HttpClientConfig {
  proxyUrls?: string[];
  fallbackEnabled?: boolean;
}
