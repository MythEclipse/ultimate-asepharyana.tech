import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger'; // Import your logger
import { corsHeaders } from '@/lib/corsHeaders'; // Import CORS headers

async function fetchAnimeData(slug: string) {
  const response = await fetchWithProxy(`https://alqanime.net/?s=${slug}`);

  if (!response.data) {
    throw new Error('Failed to fetch data');
  }

  return response.data as string;
}

function parseAnimeData(html: string) {
  const $ = cheerio.load(html);

  const animeList: {
    title: string;
    slug: string;
    poster: string;
    description: string;
    anime_url: string;
    genres: string[];
    rating: string;
    type: string;
    season: string;
  }[] = [];

  $('.listupd article.bs').each((_, element) => {
    const title = $(element).find('.ntitle').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[3] || '';
    const poster = $(element).find('img').attr('data-src') || '';
    const description = $(element).find('h2').text().trim() || '';
    const anime_url = $(element).find('a').attr('href') || '';
    const genres: string[] = []; // Genres are not available in the provided HTML
    const rating = $(element).find('.numscore').text().trim() || '';
    const type = $(element).find('.typez').text().trim() || '';
    const season = ''; // Season is not available in the provided HTML

    animeList.push({
      title,
      slug,
      poster,
      description,
      anime_url,
      genres,
      rating,
      type,
      season,
    });
  });

  const currentPage = parseInt($('.pagination .current').text()) || 1;
  const lastVisiblePage =
    parseInt($('.pagination .page-numbers').last().prev().text()) || 1;
  const hasNextPage = $('.pagination .next').length > 0;
  const nextPage = $('.pagination .next').attr('href') || null;
  const hasPreviousPage = currentPage > 1;
  const previousPage = hasPreviousPage
    ? $('.pagination .prev').attr('href') || null
    : null;

  const pagination = {
    current_page: currentPage,
    last_visible_page: lastVisiblePage,
    has_next_page: hasNextPage,
    next_page: nextPage,
    has_previous_page: hasPreviousPage,
    previous_page: previousPage,
  };

  return { animeList, pagination };
}

export async function GET(req: NextRequest) {
  const ip =
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown';
  const url = req.url;
  const slug = new URL(req.url).searchParams.get('q') || 'log';

  try {
    const html = await fetchAnimeData(slug);
    const { animeList, pagination } = parseAnimeData(html);

    logger.info('Request processed', {
      ip,
      url,
      animeCount: animeList.length,
    });

    const response = NextResponse.json({
      status: 'Ok',
      data: animeList,
      pagination,
    });

    Object.entries(corsHeaders).forEach(([key, value]) => {
      response.headers.set(key, value);
    });

    return response;
  } catch (error) {
    console.error(error);
    const response = NextResponse.json(
      { message: 'Failed to scrape data' },
      { status: 500 }
    );

    Object.entries(corsHeaders).forEach(([key, value]) => {
      response.headers.set(key, value);
    });

    return response;
  }
}
