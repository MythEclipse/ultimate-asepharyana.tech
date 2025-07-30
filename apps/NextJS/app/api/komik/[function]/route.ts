// Refactored to use correct dynamic route signature for Next.js 15

import { NextResponse, NextRequest } from 'next/server';
import * as cheerio from 'cheerio';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger';
import { corsHeaders } from '@/lib/corsHeaders';
import { redis } from '@/lib/redis';

// Type Definitions
interface MangaData {
  title: string;
  poster: string;
  chapter: string;
  date: string;
  score: string;
  type: string;
  slug: string;
  pagination?: Pagination;
}
interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}
interface MangaDetail {
  title: string;
  alternativeTitle: string;
  score: string;
  poster: string;
  description: string;
  status: string;
  type: string;
  releaseDate: string;
  author: string;
  totalChapter: string;
  updatedOn: string;
  genres: string[];
  chapters: { chapter: string; date: string; chapter_id: string }[];
}

interface MangaChapter {
  title: string;
  next_chapter_id: string;
  prev_chapter_id: string;
  images: string[];
  list_chapter: string;
}

// --- SINGLE FLIGHT LOGIC WITH REDIS LOCK START ---
let komikBaseUrlPromise: Promise<string> | null = null;
const KOMIK_BASE_URL_LOCK_KEY = 'komik:baseurl:lock';
const KOMIK_BASE_URL_KEY = 'komik:baseurl';

async function acquireRedisLock(key: string, ttlMs: number): Promise<boolean> {
  
  return await redis.set(key, 'locked', { nx: true, px: ttlMs }) === 'OK';
}

async function releaseRedisLock(key: string) {
  
  await redis.del(key);
}

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export const getDynamicKomikBaseUrl = async (): Promise<string> => {
  if (komikBaseUrlPromise) {
    logger.debug('[getDynamicKomikBaseUrl] Returning in-flight promise');
    return komikBaseUrlPromise;
  }
  komikBaseUrlPromise = (async () => {
    const lockTtl = 10000; // 10 seconds
    const waitInterval = 200; // ms
    const maxWait = 10000; // 10 seconds
    let waited = 0;

    // Try to acquire lock
    while (!(await acquireRedisLock(KOMIK_BASE_URL_LOCK_KEY, lockTtl))) {
      logger.debug('[getDynamicKomikBaseUrl] Waiting for Redis lock...');
      await sleep(waitInterval);
      waited += waitInterval;
      // If waited too long, break and try anyway
      if (waited >= maxWait) {
        logger.warn('[getDynamicKomikBaseUrl] Waited too long for lock, proceeding anyway');
        break;
      }
      // Check if value is already cached by other process
      const cached = await redis.get(KOMIK_BASE_URL_KEY);
      if (typeof cached === 'string' && cached && !cached.includes('.cz')) {
        logger.info('[getDynamicKomikBaseUrl] Found cached base URL while waiting for lock', { cached });
        return cached;
      }
    }

    try {
      logger.debug('[getDynamicKomikBaseUrl] Fetching komik base URL');
      const body = await fetchWithProxyOnlyWrapper('https://komikindo.cz/');
      const $ = cheerio.load(body);

      // Cari tombol WEBSITE yang mengandung link asli (bukan .cz)
      const websiteBtn = $('a.elementskit-btn')
        .filter((_, el) => {
          const href = $(el).attr('href') || '';
          // Cari href yang mengandung komikindo dan bukan .cz
          return /komikindo\.(?!cz)/.test(href) || /komikindo\./.test($(el).attr('__cporiginalvalueofhref') || '');
        })
        .first();

      // Cek di atribut __cporiginalvalueofhref jika ada, jika tidak pakai href
      let orgLink = websiteBtn.attr('__cporiginalvalueofhref') || websiteBtn.attr('href') || '';

      // Jika link berupa IP, decode dari query string __cpo
      if (/^\d+\.\d+\.\d+\.\d+/.test(orgLink)) {
        const urlObj = new URL(orgLink);
        const cpo = urlObj.searchParams.get('__cpo');
        if (cpo) {
          try {
            const decoded = Buffer.from(cpo, 'base64').toString('utf-8');
            orgLink = decoded;
          } catch {
            logger.error('[getDynamicKomikBaseUrl] Failed to decode __cpo', { cpo });
          }
        }
      }

      if (!orgLink || orgLink.includes('.cz')) {
        logger.error('[getDynamicKomikBaseUrl] Failed to fetch komik base URL selain cz');
        throw new Error('Failed to fetch komik base URL selain cz');
      }
      logger.info('[getDynamicKomikBaseUrl] Got base URL', { orgLink });
      // Cache the result immediately for other waiters
      await redis.set(KOMIK_BASE_URL_KEY, orgLink.replace(/\/$/, ''), { ex: 60 * 60 * 24 * 30 });
      return orgLink.replace(/\/$/, '');
    } finally {
      await releaseRedisLock(KOMIK_BASE_URL_LOCK_KEY);
      komikBaseUrlPromise = null;
    }
  })();
  return komikBaseUrlPromise;
};
// --- SINGLE FLIGHT LOGIC WITH REDIS LOCK END ---

export const getCachedKomikBaseUrl = async (forceRefresh = false): Promise<string> => {
  if (!forceRefresh) {
    const cached = await redis.get(KOMIK_BASE_URL_KEY);
    if (typeof cached === 'string' && cached && !cached.includes('.cz')) {
      logger.info('[getCachedKomikBaseUrl] Using cached base URL', { cached });
      return cached;
    }
  }
  // Fetch new value and cache it
  const url = await getDynamicKomikBaseUrl();
  await redis.set(KOMIK_BASE_URL_KEY, url, { ex: 60 * 60 * 24 * 30 });
  logger.info('[getCachedKomikBaseUrl] Refreshed and cached base URL', { url });
  return url;
};
// Utility function to parse manga data
const parseMangaData = (body: string): MangaData[] => {
  const $ = cheerio.load(body);
  const data: MangaData[] = [];

  $('.animposx').each((_, e) => {
    const title = $(e).find('.tt h4').text().trim() || '';
    let poster = $(e).find('img').attr('src') || '';
    poster = poster.split('?')[0]; // Remove query parameters from poster URL
    const chapter =
      $(e)
        .find('.lsch a')
        .text()
        .trim()
        .replace('Ch.', '')
        .match(/\d+(\.\d+)?/g)?.[0] || ''; // Keep only the numeric part
    const score = $(e).find('i').text().trim() || ''; // Extract score from the specified element
    const date = $(e).find('.datech').text().trim() || '';
    const type = $(e).find('.typeflag').attr('class')?.split(' ')[1] || '';
    const slug = $(e).find('a').attr('href')?.split('/')[4] || '';

    data.push({
      title,
      poster,
      chapter,
      score,
      date,
      type,
      slug,
    });
  });

  return data;
};

// Always-proxy fetch wrapper for Komik API
const fetchWithProxyOnlyWrapper = async (url: string): Promise<string> => {
  try {
    logger.debug('[fetchWithProxyOnlyWrapper] Fetching', { url });
    const response = await fetchWithProxy(url);
    logger.info('[fetchWithProxyOnlyWrapper] Fetched', { url });
    return typeof response.data === 'string'
      ? response.data
      : JSON.stringify(response.data);
  } catch (error) {
    logger.error('[fetchWithProxyOnlyWrapper] Error', { url, error: (error as Error).message });
    throw new Error('Failed to fetch data');
  }
};

// Function to get manga detail
const getDetail = async (komik_id: string, baseURL: string): Promise<MangaDetail> => {
  try {
    logger.debug('[getDetail] Fetching detail', { komik_id, baseURL });
    const body = await fetchWithProxyOnlyWrapper(`${baseURL}/komik/${komik_id}`);
    const $ = cheerio.load(body);

    // Title
    const title = $('h1.entry-title').text().trim() || '';

    // Alternative Title
    const alternativeTitle =
      $(".spe span:contains('Judul Alternatif:')")
        .text()
        .replace('Judul Alternatif:', '')
        .trim() || '';

    // Score
    const score = $('.rtg > div > i').text().trim() || '';

    // Image
    let poster = $('.thumb img').attr('src') || '';
    poster = poster.split('?')[0]; // Remove query parameters from image URL

    // Description
    const description =
      $('#sinopsis > section > div > div.entry-content.entry-content-single > p')
        .text()
        .trim() || '';

    // Status
    const status =
      $(".spe span:contains('Status:')").text().replace('Status:', '').trim() ||
      '';

    // Genres
    const genres: string[] = [];
    $('.genre-info a').each((_, el) => {
      genres.push($(el).text().trim());
    });

    // Release Date (if available, assuming it's in the same location as before)
    const releaseDate =
      $('#chapter_list > ul > li:last-child > span.dt').text().trim() || ''; // Not provided in the new HTML, keep empty or fetch from another source

    // Author
    const author = $(".spe span:contains('Pengarang:')")
      .text()
      .replace('Pengarang:', '')
      .trim();

    // Type
    const type = $(".spe span:contains('Jenis Komik:') a").text().trim();

    // Total Chapter (if available, otherwise remove this field)
    const totalChapter =
      $('#chapter_list > ul > li:nth-child(1) > span.lchx').text().trim() || '';

    // Updated On (if available, otherwise remove this field)
    const updatedOn =
      $('#chapter_list > ul > li:nth-child(1) > span.dt').text().trim() || '';

    // Chapters list from the `#chapter_list` element
    const chapters: { chapter: string; date: string; chapter_id: string }[] =
      [];
    $('#chapter_list ul li').each((_, el) => {
      const chapter = $(el).find('.lchx a').text().trim();
      const date = $(el).find('.dt a').text().trim();
      const chapter_id =
        $(el).find('.lchx a').attr('href')?.split('/')[3] || '';
      chapters.push({ chapter, date, chapter_id });
    });

    logger.info('[getDetail] Success', { komik_id, title });
    return {
      title,
      alternativeTitle,
      score,
      poster,
      description,
      status,
      type,
      releaseDate,
      author,
      totalChapter,
      updatedOn,
      genres,
      chapters,
    };
  } catch (error) {
    logger.error('[getDetail] Error', { komik_id, error: (error as Error).message });
    throw new Error('Failed to fetch manga detail');
  }
};

// Function to get manga chapter
const getChapter = async (chapter_url: string, baseURL: string): Promise<MangaChapter> => {
  try {
    logger.debug('[getChapter] Fetching chapter', { chapter_url, baseURL });
    const body = await fetchWithProxyOnlyWrapper(
      `${baseURL}/chapter/${chapter_url}`
    );
    const $ = cheerio.load(body);

    const title = $('.entry-title').text().trim() || '';

    // Handling previous chapter ID
    const prev_chapter_element = $('.nextprev a[rel="prev"]');
    const prev_chapter_id = prev_chapter_element.length
      ? prev_chapter_element.attr('href')?.split('/')[3] || ''
      : '';

    const list_chapter_element = $('.nextprev a:has(.icol.daftarch)');
    const list_chapter = list_chapter_element.length
      ? list_chapter_element.attr('href')?.split('/')[4] || ''
      : '';

    const next_chapter_element = $('.nextprev a[rel="next"]');
    const next_chapter_id = next_chapter_element.length
      ? next_chapter_element.attr('href')?.split('/')[3] || ''
      : '';

    // Extracting images
    const images: string[] = [];
    $('#chimg-auh img').each((_, el) => {
      const image = $(el).attr('src') || '';
      images.push(image);
    });

    logger.info('[getChapter] Success', { chapter_url, title, imagesCount: images.length });
    return { title, next_chapter_id, prev_chapter_id, images, list_chapter };
  } catch (error) {
    logger.error('[getChapter] Error', { chapter_url, error: (error as Error).message });
    throw new Error('Failed to fetch manga chapter');
  }
};

// Handler function for GET (dynamic route)
export async function GET(
  req: NextRequest) {
  const start = Date.now();
  const ip =
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown';
  const url = req.url;
  let type = '';
  let params: Record<string, string> = {};

  try {
    const urlObj = new URL(req.url);
    const page = urlObj.searchParams.get('page') || '1';
    type = urlObj.pathname.split('/')[3] as
      | 'manga'
      | 'manhwa'
      | 'manhua'
      | 'search'
      | 'detail'
      | 'chapter'
      | 'external-link';

    let data: unknown;
    logger.info('[API][komik] Incoming request', { ip, url, type, page });

    // Helper to get baseURL with cache refresh on failure
    const getBaseUrlWithRetry = async (): Promise<string> => {
      try {
        return await getCachedKomikBaseUrl();
      } catch {
        logger.warn('[API][komik] Cached base URL failed, retrying with refresh');
        return await getCachedKomikBaseUrl(true);
      }
    };

    if (type === 'detail' || type === 'chapter' || type === 'manga' || type === 'manhwa' || type === 'manhua' || type === 'search') {
      // Always get the dynamic base URL for all komik requests
      const baseURL = await getBaseUrlWithRetry();

      if (type === 'detail') {
        const komik_id = urlObj.searchParams.get('komik_id') || 'one-piece';
        params = { komik_id };
        data = await getDetail(komik_id, baseURL);
      } else if (type === 'chapter') {
        const chapter_url = urlObj.searchParams.get('chapter_url') || '';
        params = { chapter_url };
        data = await getChapter(chapter_url, baseURL);
      } else {
        let apiUrl = `${baseURL}/${type}/page/${page}/`;
        if (type === 'search') {
          const query = urlObj.searchParams.get('query') || '';
          apiUrl = `${baseURL}/page/${page}/?s=${query}`;
          params = { query };
        }
        const body = await fetchWithProxyOnlyWrapper(apiUrl);
        const $ = cheerio.load(body);
        const currentPage =
          parseInt($('.pagination .current').text().trim()) || 1;
        const totalPages =
          parseInt($('.pagination a:not(.next):last').text().trim()) ||
          currentPage;

        const pagination: Pagination = {
          current_page: currentPage,
          last_visible_page: totalPages,
          has_next_page: $('.pagination .next').length > 0,
          next_page: currentPage < totalPages ? currentPage + 1 : null,
          has_previous_page: $('.pagination .prev').length > 0,
          previous_page: currentPage > 1 ? currentPage - 1 : null,
        };

        data = {
          data: parseMangaData(body),
          pagination,
        };
      }
    } else if (type === 'external-link') {
      // Fetch and parse external link from komikindo.cz
      const baseURL = await getBaseUrlWithRetry();
      data = { link: baseURL };
    } else {
      logger.error('[API][komik] Invalid type parameter', { type });
      throw new Error('Invalid type parameter');
    }

    const duration = Date.now() - start;
    logger.info('[API][komik] Success', { ip, url, type, params, durationMs: duration });

    return NextResponse.json(data, {
      status: 200,
      headers: corsHeaders,
    });
  } catch (error) {
    const duration = Date.now() - start;
    logger.error('[API][komik] Error', {
      ip,
      url,
      type,
      params,
      error: (error as Error).message,
      durationMs: duration,
    });
    return NextResponse.json(
      {
        status: false,
        message: (error as { message: string }).message,
      },
      {
        status: 500,
        headers: corsHeaders,
      }
    );
  }
}
