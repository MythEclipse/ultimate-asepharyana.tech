import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import { corsHeaders } from '@/lib/corsHeaders';
import { withLogging } from '@/lib/api-wrapper';

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

async function handler(req: NextRequest): Promise<NextResponse> {
  const slug = new URL(req.url).searchParams.get('q') || 'one';
  const html = await fetchAnimeData(slug);
  const { animeList, pagination } = parseAnimeData(html, slug);

  const ongoingAnime = animeList.filter(
    (anime) => anime.status === 'Ongoing'
  );
  const completeAnime = animeList.filter(
    (anime) => anime.status === 'Completed'
  );

  return NextResponse.json(
    {
      status: 'Ok',
      data: animeList,
      pagination,
    },
    {
      headers: corsHeaders,
    }
  );
}

export const GET = withLogging(handler);
