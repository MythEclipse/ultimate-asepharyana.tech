// API Base URLs
const RUST_API = 'https://ws.asepharyana.tech'; // For anime, komik endpoints
const ELYSIA_API = 'https://elysia.asepharyana.tech'; // For auth, chat endpoints

interface RequestOptions {
  headers?: Record<string, string>;
  credentials?: RequestCredentials;
  signal?: AbortSignal;
}

class HttpClient {
  private rustApi: string;
  private elysiaApi: string;

  constructor(rustApi: string = RUST_API, elysiaApi: string = ELYSIA_API) {
    this.rustApi = rustApi;
    this.elysiaApi = elysiaApi;
  }

  private getBaseUrl(path: string): string {
    // Use Rust API for anime and komik endpoints
    if (path.startsWith('/api/anime') || path.startsWith('/api/komik')) {
      return this.rustApi;
    }
    // Use Elysia API for auth, chat, and other endpoints
    return this.elysiaApi;
  }

  async fetchJson<T>(path: string, options: RequestOptions = {}): Promise<T> {
    const url = path.startsWith('http')
      ? path
      : `${this.getBaseUrl(path)}${path}`;

    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      credentials: options.credentials || 'include',
    });

    if (!response.ok) {
      const error = await response
        .json()
        .catch(() => ({ message: response.statusText }));
      throw new Error(error.message || `HTTP ${response.status}`);
    }

    return response.json();
  }

  async request<T>(
    path: string,
    method: 'GET' | 'POST' | 'PUT' | 'DELETE' = 'GET',
    body?: unknown,
    headers?: Record<string, string>,
    options: RequestOptions = {},
  ): Promise<T> {
    const url = path.startsWith('http')
      ? path
      : `${this.getBaseUrl(path)}${path}`;

    const response = await fetch(url, {
      method,
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...headers,
        ...options.headers,
      },
      credentials: options.credentials || 'include',
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      const error = await response
        .json()
        .catch(() => ({ message: response.statusText }));
      throw new Error(error.message || `HTTP ${response.status}`);
    }

    return response.json();
  }
}

export const httpClient = new HttpClient();
export { HttpClient };
