import { HttpClient } from '../utils/httpClient';
import { APIURLSERVER } from '../lib/url';

export class KomikService {
  static async searchKomik(query: string): Promise<unknown> {
    const url = `/api/komik/search?q=${encodeURIComponent(query)}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getKomikDetail(slug: string): Promise<unknown> {
    const url = `/api/komik/detail/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getKomikChapter(slug: string): Promise<unknown> {
    const url = `/api/komik/chapter/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getKomik2Manga(slug: string): Promise<unknown> {
    const url = `/api/komik2/manga/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }
}
