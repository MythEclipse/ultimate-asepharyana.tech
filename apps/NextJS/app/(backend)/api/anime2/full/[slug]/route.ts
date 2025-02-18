import { NextRequest, NextResponse } from 'next/server';
import * as cheerio from 'cheerio';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger';

interface AnimeResponse {
  status: string;
  data: AnimeData;
}

interface AnimeData {
  episode: string;
  episode_number: string;
  anime: AnimeInfo;
  has_next_episode: boolean;
  next_episode: EpisodeInfo | null;
  has_previous_episode: boolean;
  previous_episode: EpisodeInfo | null;
  stream_url: object;
  download_urls: Record<string, { server: string; url: string }[]>;
  image_url: string;
}

interface AnimeInfo {
  slug: string;
}

interface EpisodeInfo {
  slug: string;
}

const fetchAnimePage = async (slug: string): Promise<string> => {
  const url = `https://s4.nontonanimeid.boats/${slug}/`;
  const response = await fetchWithProxy(url);

  if (!response.data) {
    throw new Error('Failed to fetch page');
  }

  if (typeof response.data !== 'string') {
    throw new Error('Expected response data to be a string');
  }
  return response.data;
};

const parseAnimePage = (html: string, slug: string): AnimeData => {
  const $ = cheerio.load(html);

  const episode = $('#arealinker > h2').text();
  const episode_number = episode.match(/Episode (\d+)/)?.[1] || '';
  const image_url = $('.cukder img').attr('src') || '';
  const stream_url = $('#videoku > iframe').data('src') || '';

  const downloadUrls: Record<string, { server: string; url: string }[]> = {};

  $('.listlink').each((_, element) => {
    const resolution = $(element).find('span').text().trim();
    const links = $(element)
      .find('a')
      .map((_, link) => ({
        server: $(link).text().trim(),
        url: $(link).attr('href') || '',
      }))
      .get();

    if (resolution && links.length > 0) {
      downloadUrls[resolution] = links;
    }
  });

  const nextEpisodeElement = $(
    '#navigation-episode .nvs a[title*="Episode"][href*="episode-"]'
  );
  const prevEpisodeElement = $(
    '#navigation-episode .nvs a[title*="Episode"][href*="episode-"]'
  );

  const next_episode_url = nextEpisodeElement.last().attr('href') || null;
  const previous_episode_url = prevEpisodeElement.first().attr('href') || null;

  const next_episode_slug = next_episode_url
    ? next_episode_url.split('/').slice(-2).join('/')
    : null;
  const previous_episode_slug = previous_episode_url
    ? previous_episode_url.split('/').slice(-2).join('/')
    : null;

  return {
    episode,
    episode_number,
    anime: { slug },
    has_next_episode: !!next_episode_slug,
    next_episode: next_episode_slug ? { slug: next_episode_slug } : null,
    has_previous_episode: !!previous_episode_slug,
    previous_episode: previous_episode_slug
      ? { slug: previous_episode_slug }
      : null,
    stream_url,
    download_urls: downloadUrls,
    image_url,
  };
};

export async function GET(
  request: NextRequest,
  props: { params: Promise<{ slug: string }> }
) {
  const params = await props.params;
  const { slug } = params;

  const ip =
    request.headers.get('x-forwarded-for') ||
    request.headers.get('remote-addr') ||
    'unknown';
  const url = request.url;

  try {
    const html = await fetchAnimePage(slug);
    const data = parseAnimePage(html, slug);

    const animeResponse: AnimeResponse = {
      status: 'Ok',
      data,
    };

    logger.info('Request processed', {
      ip,
      url,
      slug,
    });

    return NextResponse.json(animeResponse, { status: 200 });
  } catch (error: unknown) {
    const err = error as { message: string };
    logger.error('Error processing request', {
      ip,
      url,
      slug,
      error: err.message,
    });
    return NextResponse.json(
      {
        status: 'Error',
        message: (error as { message: string }).message,
      },
      { status: 500 }
    );
  }
}
