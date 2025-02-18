import * as cheerio from 'cheerio';
import { NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger';

async function fetchHtml(url: string): Promise<string> {
  const response = await fetchWithProxy(url);

  if (!response.data) {
    throw new Error(`Failed to fetch data from ${url}`);
  }

  if (typeof response.data !== 'string') {
    throw new Error(
      `Expected response data to be a string but got ${typeof response.data}`
    );
  }
  return response.data;
}

function parseAnime(html: string) {
  const $ = cheerio.load(html);
  const animeList: {
    title: string;
    slug: string;
    poster: string;
    episode: string;
    anime_url: string;
  }[] = [];

  $('.misha_posts_wrap article').each((index, element) => {
    const title = $(element).find('h3.title span').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[3] || '';
    const poster = $(element).find('img').attr('src') || '';
    const episode = $(element).find('.types.episodes').text().trim() || 'N/A';
    const anime_url = $(element).find('a').attr('href') || '';

    animeList.push({
      title,
      slug,
      poster,
      episode,
      anime_url,
    });
  });

  return animeList;
}

export async function GET(request: Request) {
  const ip =
    request.headers.get('x-forwarded-for') ||
    request.headers.get('remote-addr') ||
    'unknown';
  const url = request.url;

  try {
    const html = await fetchHtml('https://s4.nontonanimeid.boats/');

    const animeList = parseAnime(html);

    logger.info('Request processed', {
      ip,
      url,
      animeCount: animeList.length,
    });

    return NextResponse.json({
      status: 'Ok',
      data: {
        anime_list: animeList,
      },
    });
  } catch (error) {
    logger.error('Failed to scrape data', { error, url });
    return NextResponse.json(
      { message: 'Failed to scrape data' },
      { status: 500 }
    );
  }
}
