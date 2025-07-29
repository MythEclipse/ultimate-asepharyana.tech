import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import { corsHeaders } from '@/lib/corsHeaders';
import logger from '@/lib/logger';

async function fetchAnimeData(slug: string) {
  const response = await fetchWithProxy(
    `https://otakudesu.cloud/?s=${slug}&post_type=anime`
  );

  if (!response.data) {
    throw new Error('Failed to fetch data');
  }

  return response.data as string;
}

function parseAnimeData(html: string, slug: string) {
  const $ = cheerio.load(html);

  const animeList: {
    title: string;
    slug: string;
    poster: string;
    episode: string;
    anime_url: string;
    genres: string[];
    status: string;
    rating: string;
  }[] = [];

  $('#venkonten .chivsrc li').each((_, element) => {
    const title = $(element).find('h2 a').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[4] || '';
    const poster = $(element).find('img').attr('src') || '';
    const episode =
      $(element)
        .find('h2 a')
        .text()
        .match(/\(([^)]+)\)/)?.[1] || 'Ongoing';
    const anime_url = $(element).find('a').attr('href') || '';
    const genres = $(element)
      .find('.set b:contains("Genres")')
      .nextAll('a')
      .map((_, el) => $(el).text())
      .get();
    const status =
      $(element)
        .find('.set b:contains("Status")')
        .parent()
        .text()
        .replace('Status :', '')
        .trim() || '';
    const rating =
      $(element)
        .find('.set b:contains("Rating")')
        .parent()
        .text()
        .replace('Rating :', '')
        .trim() || '';

    animeList.push({
      title,
      slug,
      poster,
      episode,
      anime_url,
      genres,
      status,
      rating,
    });
  });

  const pagination = {
    current_page: parseInt(slug as string, 10) || 1,
    last_visible_page: 57,
    has_next_page: $('.hpage .r').length > 0,
    next_page:
      $('.hpage .r').length > 0 ? parseInt(slug as string, 10) + 1 : null,
    has_previous_page: parseInt(slug as string, 10) > 1,
    previous_page:
      parseInt(slug as string, 10) > 1
        ? parseInt(slug as string, 10) - 1
        : null,
  };

  return { animeList, pagination };
}

// Handler for static route: only (req: NextRequest)
export async function GET(req: NextRequest): Promise<NextResponse> {
  const ip =
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown';
  const url = req.url;
  const method = req.method;
  const requestId = req.headers.get('x-request-id') || undefined;
  const start = Date.now();

  try {
    const slug = new URL(req.url).searchParams.get('q') || 'one';
    const html = await fetchAnimeData(slug);
    const { animeList, pagination } = parseAnimeData(html, slug);


    const response = NextResponse.json(
      {
        status: 'Ok',
        data: animeList,
        pagination,
      },
      {
        headers: corsHeaders,
      }
    );

    const duration = Date.now() - start;
    
    if (requestId) {
      response.headers.set('x-request-id', requestId);
    }
    return response;
  } catch (error: unknown) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    const duration = Date.now() - start;
    logger.error(
      `[Error processing request] ip=${ip} | url=${url} | method=${method} | error=${errorMessage} | durationMs=${duration}${requestId ? ` | requestId=${requestId}` : ''}`
    );
    const response = NextResponse.json(
      {
        message: 'Failed to process request',
        error: errorMessage,
        ...(requestId ? { requestId } : {}),
      },
      { status: 500, headers: corsHeaders }
    );
    if (requestId) {
      response.headers.set('x-request-id', requestId);
    }
    return response;
  }
}
