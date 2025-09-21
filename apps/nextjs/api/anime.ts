import { HttpClient } from '../utils/httpClient';
import { APIURLSERVER } from '../lib/url';
import { AnimeData, CompleteAnimeData } from '../types/anime';

export class AnimeService {
  static async searchAnime(query: string): Promise<AnimeData> {
    const url = `/api/anime/search?q=${encodeURIComponent(query)}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getAnimeDetail(slug: string): Promise<AnimeData> {
    const url = `/api/anime/detail/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getCompleteAnime(slug: string): Promise<CompleteAnimeData> {
    const url = `/api/anime/complete-anime/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getOngoingAnime(): Promise<AnimeData> {
    const url = `/api/anime/ongoing-anime`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async searchAnime2(query: string): Promise<AnimeData> {
    const url = `/api/anime2/search?q=${encodeURIComponent(query)}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getAnime2Detail(slug: string): Promise<AnimeData> {
    const url = `/api/anime2/detail/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }

  static async getCompleteAnime2(slug: string): Promise<CompleteAnimeData> {
    const url = `/api/anime2/complete-anime/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
  }
}
