import { HttpClient } from '../utils/httpClient';
import { APIURLSERVER } from '../lib/url';
import { AnimeData, CompleteAnimeData } from '../types/anime';
import { buildAnimeUrl } from '../utils/url-utils';

export class AnimeAPI {
  static async searchAnime(query: string): Promise<AnimeData> {
    const url = buildAnimeUrl('search', undefined, query);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }

  static async getAnimeDetail(slug: string): Promise<AnimeData> {
    const url = buildAnimeUrl('detail', slug);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }

  static async getCompleteAnime(slug: string): Promise<CompleteAnimeData> {
    const url = buildAnimeUrl('complete-anime', slug);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }

  static async getOngoingAnime(): Promise<AnimeData> {
    const url = buildAnimeUrl('ongoing-anime');
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }
}

// Anime2 API endpoints
export class Anime2API {
  static async searchAnime2(query: string): Promise<AnimeData> {
    const url = `/api/anime2/search?q=${encodeURIComponent(query)}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
    });
  }

  static async getAnime2Detail(slug: string): Promise<AnimeData> {
    const url = `/api/anime2/detail/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
    });
  }

  static async getCompleteAnime2(slug: string): Promise<CompleteAnimeData> {
    const url = `/api/anime2/complete-anime/${slug}`;
    const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
    return HttpClient.fetchJson(fullUrl, {
      next: { revalidate: 60 },
    });
  }
}
