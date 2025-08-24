// Refactored to use correct dynamic route signature for Next.js 15

import { NextResponse, NextRequest } from 'next/server';
import * as cheerio from 'cheerio';
import { fetchWithProxy, ProxyListOnly,CroxyProxyOnly } from '@/lib/fetchWithProxy';
import logger from '@/utils/logger';
import { corsHeaders } from '@/lib/corsHeaders';
import { getCachedKomikBaseUrl } from '@/lib/komikBaseUrl';

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

    if (chapters.length === 0) {
      logger.error('[getDetail] Empty chapters parsed', { raw: body.slice(0, 500) });
    }

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

    if (images.length === 0) {
      logger.error('[getChapter] Empty images parsed', { raw: body.slice(0, 500) });
    }

    logger.info('[getChapter] Success', { chapter_url, title, imagesCount: images.length });
    return { title, next_chapter_id, prev_chapter_id, images, list_chapter };
  } catch (error) {
    logger.error('[getChapter] Error', { chapter_url, error: (error as Error).message });
    throw new Error('Failed to fetch manga chapter');
  }
};

// Helper to get baseURL with cache refresh on failure
const getBaseUrlWithRetry = async (): Promise<string> => {
  try {
    return await getCachedKomikBaseUrl();
  } catch {
    logger.warn('[API][komik] Cached base URL failed, retrying with refresh');
    return await getCachedKomikBaseUrl(true);
  }
};

// Extracted handlers for each type
const handleDetail = async (urlObj: URL, baseURL: string) => {
  const komik_id = urlObj.searchParams.get('komik_id') || 'one-piece';
  return {
    data: await getDetail(komik_id, baseURL),
    params: { komik_id }
  };
};

const handleChapter = async (urlObj: URL, baseURL: string) => {
  const chapter_url = urlObj.searchParams.get('chapter_url') || '';
  return {
    data: await getChapter(chapter_url, baseURL),
    params: { chapter_url }
  };
};

const handleListOrSearch = async (urlObj: URL, baseURL: string, type: string, page: string) => {
  let apiUrl = `${baseURL}/${type}/page/${page}/`;
  let params: Record<string, string> = {};
  if (type === 'search') {
    const query = urlObj.searchParams.get('query') || '';
    apiUrl = `${baseURL}/page/${page}/?s=${query}`;
    params = { query };
  }
  // First attempt
  let body = await fetchWithProxyOnlyWrapper(apiUrl);
  let $ = cheerio.load(body);
  let parsedData = parseMangaData(body);
  if (parsedData.length === 0) {
    logger.error('[handleListOrSearch] Empty data after parsing', { raw: body.slice(0, 500) });
  }
  const currentPage =
    parseInt($('.pagination .current').text().trim()) || 1;
  const totalPages =
    parseInt($('.pagination a:not(.next):last').text().trim()) ||
    currentPage;

  let pagination: Pagination = {
    current_page: currentPage,
    last_visible_page: totalPages,
    has_next_page: $('.pagination .next').length > 0,
    next_page: currentPage < totalPages ? currentPage + 1 : null,
    has_previous_page: $('.pagination .prev').length > 0,
    previous_page: currentPage > 1 ? currentPage - 1 : null,
  };

  // If data is empty, try with a refreshed proxy
  if (parsedData.length === 0) {
    const refreshedBaseUrl = await getCachedKomikBaseUrl(true);
    let retryApiUrl = `${refreshedBaseUrl}/${type}/page/${page}/`;
    if (type === 'search') {
      const query = urlObj.searchParams.get('query') || '';
      retryApiUrl = `${refreshedBaseUrl}/page/${page}/?s=${query}`;
    }
    const proxyResult = await ProxyListOnly(retryApiUrl, 10);
    body = typeof proxyResult.data === 'string' ? proxyResult.data : JSON.stringify(proxyResult.data);
    $ = cheerio.load(body);
    parsedData = parseMangaData(body);
    if (parsedData.length === 0) {
      logger.error('[handleListOrSearch] Empty data after parsing (retry)', { raw: body.slice(0, 500) });
    }
    // Update pagination after retry
    const retryCurrentPage =
      parseInt($('.pagination .current').text().trim()) || 1;
    const retryTotalPages =
      parseInt($('.pagination a:not(.next):last').text().trim()) ||
      retryCurrentPage;
    pagination = {
      current_page: retryCurrentPage,
      last_visible_page: retryTotalPages,
      has_next_page: $('.pagination .next').length > 0,
      next_page: retryCurrentPage < retryTotalPages ? retryCurrentPage + 1 : null,
      has_previous_page: $('.pagination .prev').length > 0,
      previous_page: retryCurrentPage > 1 ? retryCurrentPage - 1 : null,
    };
  }

  // If still empty, try CroxyProxyOnly
  if (parsedData.length === 0) {
    let croxyHtml: string | null = null;
    try {
      const croxyResult = await CroxyProxyOnly(apiUrl);
      croxyHtml = typeof croxyResult.data === 'string' ? croxyResult.data : '';
      if (croxyHtml) {
        $ = cheerio.load(croxyHtml);
        parsedData = parseMangaData(croxyHtml);
        if (parsedData.length === 0) {
          logger.error('[handleListOrSearch] CroxyProxyOnly returned empty data after parsing', { raw: croxyHtml.slice(0, 500) });
        }
        // Update pagination after CroxyProxyOnly
        const croxyCurrentPage =
          parseInt($('.pagination .current').text().trim()) || 1;
        const croxyTotalPages =
          parseInt($('.pagination a:not(.next):last').text().trim()) ||
          croxyCurrentPage;
        pagination = {
          current_page: croxyCurrentPage,
          last_visible_page: croxyTotalPages,
          has_next_page: $('.pagination .next').length > 0,
          next_page: croxyCurrentPage < croxyTotalPages ? croxyCurrentPage + 1 : null,
          has_previous_page: $('.pagination .prev').length > 0,
          previous_page: croxyCurrentPage > 1 ? croxyCurrentPage - 1 : null,
        };
      }
    } catch (err) {
      logger.error('[handleListOrSearch] CroxyProxyOnly failed', { err, raw: croxyHtml ? croxyHtml.slice(0, 500) : undefined });
    }
  }

  return {
    data: {
      data: parsedData,
      pagination,
    },
    params
  };
};

const handleExternalLink = async (baseURL: string) => {
  return {
    data: { link: baseURL },
    params: {}
  };
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

    logger.info('[API][komik] Incoming request', { ip, url, type, page });

    let handlerResult: { data: unknown; params: Record<string, string> } | undefined;

    if (type === 'detail' || type === 'chapter' || type === 'manga' || type === 'manhwa' || type === 'manhua' || type === 'search') {
      const baseURL = await getBaseUrlWithRetry();
      if (type === 'detail') {
        handlerResult = await handleDetail(urlObj, baseURL);
      } else if (type === 'chapter') {
        handlerResult = await handleChapter(urlObj, baseURL);
      } else {
        handlerResult = await handleListOrSearch(urlObj, baseURL, type, page);
      }
    } else if (type === 'external-link') {
      const baseURL = await getBaseUrlWithRetry();
      handlerResult = await handleExternalLink(baseURL);
    } else {
      logger.error('[API][komik] Invalid type parameter', { type });
      throw new Error('Invalid type parameter');
    }

    const data = handlerResult.data;
    params = handlerResult.params;

    // === EMPTY DATA CHECK FOR LIST/SEARCH ===
    if (
      (type === 'manga' || type === 'manhwa' || type === 'manhua' || type === 'search') &&
      typeof data === 'object' &&
      data !== null &&
      'data' in data &&
      Array.isArray((data as { data: unknown } & Record<string, unknown>).data) &&
      ((data as { data: unknown[] } & Record<string, unknown>).data.length === 0)
    ) {
      const duration = Date.now() - start;
      logger.warn('[API][komik] Empty data array after all proxy attempts', { ip, url, type, params, durationMs: duration });
      return NextResponse.json(
        {
          status: false,
          message: 'No data found',
          data: [],
          pagination: typeof data === 'object' && data !== null && 'pagination' in data ? (data as { pagination?: unknown }).pagination ?? null : null,
        },
        {
          status: 404,
          headers: corsHeaders,
        }
      );
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
