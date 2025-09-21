import { UnifiedHttpClient } from './http-client';

// Re-export the UnifiedHttpClient methods for backward compatibility
export class HttpClient {
  static buildUrl(base: string, url: string): string {
    return UnifiedHttpClient.buildUrl(base, url);
  }

  static async fetchJson<T = unknown>(url: string, options: RequestInit = {}): Promise<T> {
    const client = new UnifiedHttpClient();
    return client.fetchJson<T>(url, options);
  }

  static async fetchWithAuth<T = unknown>(url: string, token?: string | null, options: RequestInit = {}): Promise<T> {
    const client = new UnifiedHttpClient();
    return client.fetchWithAuth<T>(url, token, options);
  }

  static async request<T = unknown>(url: string, method: string, body?: unknown, token?: string | null): Promise<T> {
    const client = new UnifiedHttpClient();
    return client.request<T>(url, method as any, body, token);
  }
}
