import { HttpClient } from '../utils/http-client';
import { APIURLSERVER } from '../utils/url-utils';
import { buildKomikUrl, buildKomik2MangaUrl } from '../utils/url-utils';

export class KomikAPI {
  static async searchKomik(query: string): Promise<unknown> {
    const url = buildKomikUrl('search', undefined, query);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }

  static async getKomikDetail(slug: string): Promise<unknown> {
    const url = buildKomikUrl('detail', slug);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }

  static async getKomikChapter(slug: string): Promise<unknown> {
    const url = buildKomikUrl('chapter', slug);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }
}

export class Komik2API {
  static async getKomik2Manga(slug: string): Promise<unknown> {
    const url = buildKomik2MangaUrl(slug);
    return HttpClient.fetchJson(url, {
      next: { revalidate: 60 },
    });
  }
}
