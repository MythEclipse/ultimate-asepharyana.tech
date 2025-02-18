import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger'; // Import your logger

async function fetchAnimeData(slug: string) {
  const response = await fetchWithProxy(
    `https://s4.nontonanimeid.boats/?s=${slug}`
  );

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

  $('.result ul li').each((_, element) => {
    const title = $(element).find('h2').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[4] || '';
    const poster = $(element).find('img').attr('src') || '';
    const description = $(element).find('.descs').text().trim() || '';
    const anime_url = $(element).find('a').attr('href') || '';
    const genres = $(element)
      .find('.genrebatas .genre')
      .map((_, el) => $(el).text())
      .get();
    const rating = $(element).find('.nilaiseries').text().trim() || '';
    const type = $(element).find('.typeseries').text().trim() || '';
    const season = $(element).find('.rsrated').text().trim() || '';

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

  const pagination = {
    current_page: 1,
    last_visible_page: 1,
    has_next_page: false,
    next_page: null,
    has_previous_page: false,
    previous_page: null,
  };

  return { animeList, pagination };
}

export async function GET(req: NextRequest) {
  const ip =
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown';
  const url = req.url;
  const slug = new URL(req.url).searchParams.get('q') || 'log+horizon';

  try {
    const html = await fetchAnimeData(slug);
    const { animeList, pagination } = parseAnimeData(html);

    logger.info('Request processed', {
      ip,
      url,
      animeCount: animeList.length,
    });

    return NextResponse.json({
      status: 'Ok',
      data: animeList,
      pagination,
    });
  } catch (error) {
    console.error(error);
    return NextResponse.json(
      { message: 'Failed to scrape data' },
      { status: 500 }
    );
  }
}
