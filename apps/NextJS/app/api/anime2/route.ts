import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import { corsHeaders } from '@/lib/corsHeaders';
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

function parseOngoingAnime(html: string) {
  const $ = cheerio.load(html);
  const ongoingAnime: {
    title: string;
    slug: string;
    poster: string;
    current_episode: string;
    anime_url: string;
  }[] = [];

  $('.listupd .bs').each((index, element) => {
    const title = $(element).find('.ntitle').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[3] || '';
    const poster = $(element).find('img').attr('data-src') || '';
    const current_episode = $(element).find('.epx').text().trim() || 'N/A';
    const anime_url = $(element).find('a').attr('href') || '';

    ongoingAnime.push({
      title,
      slug,
      poster,
      current_episode,
      anime_url,
    });
  });

  return ongoingAnime;
}

function parseCompleteAnime(html: string) {
  const $ = cheerio.load(html);
  const completeAnime: {
    title: string;
    slug: string;
    poster: string;
    episode_count: string;
    anime_url: string;
  }[] = [];

  $('.listupd .bs').each((index, element) => {
    const title = $(element).find('.ntitle').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[3] || '';
    const poster = $(element).find('img').attr('data-src') || '';
    const episode_count = $(element).find('.epx').text().trim() || 'N/A';
    const anime_url = $(element).find('a').attr('href') || '';

    completeAnime.push({
      title,
      slug,
      poster,
      episode_count,
      anime_url,
    });
  });

  return completeAnime;
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
    const ongoingHtml = await fetchHtml(
      'https://alqanime.net/advanced-search/?status=ongoing&order=update'
    );
    const completeHtml = await fetchHtml(
      'https://alqanime.net/advanced-search/?status=completed&order=update'
    );

    const ongoingAnime = parseOngoingAnime(ongoingHtml);
    const completeAnime = parseCompleteAnime(completeHtml);

    const response = NextResponse.json(
      {
        status: 'Ok',
        data: {
          ongoing_anime: ongoingAnime,
          complete_anime: completeAnime,
        },
      },
      { headers: corsHeaders }
    );

    const duration = Date.now() - start;
    logger.info(
      `[Request processed] ip=${ip} | url=${url} | method=${method} | status=${response.status} | durationMs=${duration}${requestId ? ` | requestId=${requestId}` : ''}`
    );
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
