import logger from './logger';

export class HttpClient {
  static buildUrl(base: string, url: string): string {
    return url.startsWith('/') ? `${base}${url}` : url;
  }

  static async fetchJson<T = unknown>(url: string, options: RequestInit = {}): Promise<T> {
    const response = await fetch(url, {
      ...options,
      headers: { 'Content-Type': 'application/json', ...options.headers },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({ message: response.statusText }));
      const error = new Error(errorData.message || 'Request failed');
      logger.error(`HTTP Error: ${url} - Status: ${response.status}, Message: ${error.message}`);
      throw error;
    }
    return response.json() as T;
  }

  static async fetchWithAuth<T = unknown>(url: string, token?: string | null, options: RequestInit = {}): Promise<T> {
    const headers = new Headers(options.headers);
    headers.set('Content-Type', 'application/json');
    if (token) {
      headers.set('Authorization', `Bearer ${token}`);
    }
    return this.fetchJson<T>(url, { ...options, headers });
  }

  static async request<T = unknown>(url: string, method: string, body?: unknown, token?: string | null): Promise<T> {
    const options: RequestInit = { method };
    if (body) {
      options.body = JSON.stringify(body);
    }
    return this.fetchWithAuth<T>(url, token, options);
  }
}
