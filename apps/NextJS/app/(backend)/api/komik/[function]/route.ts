import { NextResponse } from 'next/server';
import * as cheerio from 'cheerio';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger';
import { corsHeaders } from '@/lib/corsHeaders';

const baseUrl = {
  komik: 'https://komikindo2.com',
};
const baseURL = baseUrl.komik;

// Logging Function
const logError = (error: { message: string }) => {
  console.error('Error:', error.message);
};

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

// Function to fetch data with proxy
const fetchWithProxyWrapper = async (url: string): Promise<string> => {
  try {
    const response = await fetchWithProxy(url);
    return typeof response.data === 'string'
      ? response.data
      : JSON.stringify(response.data);
  } catch (error) {
    logError(error as { message: string });
    throw new Error('Failed to fetch data');
  }
};

// Function to get manga detail
const getDetail = async (komik_id: string): Promise<MangaDetail> => {
  try {
    const body = await fetchWithProxyWrapper(`${baseURL}/komik/${komik_id}`);
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
      $(
        '#sinopsis > section > div > div.entry-content.entry-content-single > p'
      )
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
    logError(error as { message: string });
    throw new Error('Failed to fetch manga detail');
  }
};

// Function to get manga chapter
const getChapter = async (chapter_url: string): Promise<MangaChapter> => {
  try {
    const body = await fetchWithProxyWrapper(
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

    return { title, next_chapter_id, prev_chapter_id, images, list_chapter };
  } catch (error) {
    logError(error as { message: string });
    throw new Error('Failed to fetch manga chapter');
  }
};

export const GET = async (req: Request) => {
  const ip =
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown';
  const url = req.url;

  try {
    const urlObj = new URL(req.url);
    const page = urlObj.searchParams.get('page') || '1';
    const type = urlObj.pathname.split('/')[3] as
      | 'manga'
      | 'manhwa'
      | 'manhua'
      | 'search'
      | 'detail'
      | 'chapter';

    let data: unknown;
    if (type === 'detail') {
      const komik_id = urlObj.searchParams.get('komik_id') || 'one-piece';
      data = await getDetail(komik_id);
    } else if (type === 'chapter') {
      const chapter_url = urlObj.searchParams.get('chapter_url') || '';
      data = await getChapter(chapter_url);
    } else {
      let apiUrl = `${baseURL}/${type}/page/${page}/`;
      if (type === 'search') {
        const query = urlObj.searchParams.get('query') || '';
        apiUrl = `${baseURL}/page/${page}/?s=${query}`;
      }
      const body = await fetchWithProxyWrapper(apiUrl);
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

    logger.info('Request processed', {
      ip,
      url,
      type,
      page,
    });

    return NextResponse.json(data, {
      status: 200,
      headers: corsHeaders,
    });
  } catch (error) {
    logError(error as { message: string });
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
};
