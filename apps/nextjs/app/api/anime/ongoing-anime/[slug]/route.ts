import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '../../../../../lib/fetchWithProxy';
import { corsHeaders } from '../../../../../lib/corsHeaders';
import { withLogging } from '../../../../../lib/api-wrapper';

async function fetchAnimePage(slug: string): Promise<string> {
  const response = await fetchWithProxy(
    `https://otakudesu.cloud/ongoing-anime/page/${slug}/`
  );

  if (!response.data) {
    throw new Error('Failed to fetch data');
  }

  return typeof response.data === 'string'
    ? response.data
    : JSON.stringify(response.data);
}

function parseAnimePage(html: string, slug: string) {
  const $ = cheerio.load(html);

  const animeList: {
    title: string;
    slug: string;
    poster: string;
    episode: string;
    anime_url: string;
  }[] = [];

  $('.venz ul li').each((index, element) => {
    const title = $(element).find('.thumbz h2.jdlflm').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[4] || '';
    const poster = $(element).find('img').attr('src') || '';
    const episode = $(element).find('.epz').text().trim() || 'Ongoing';
    const anime_url = $(element).find('a').attr('href') || '';

    animeList.push({
      title,
      slug,
      poster,
      episode,
      anime_url,
    });
  });

  const pagination = {
    current_page: parseInt(slug, 10) || 1,
    last_visible_page:
      parseInt($('.pagination .page-numbers:not(.next):last').text(), 10) || 1,
    has_next_page: $('.pagination .next').length > 0,
    next_page:
      $('.pagination .next').length > 0 ? parseInt(slug, 10) + 1 : null,
    has_previous_page: parseInt(slug, 10) > 1,
    previous_page: parseInt(slug, 10) > 1 ? parseInt(slug, 10) - 1 : null,
  };

  return { animeList, pagination };
}

async function handler(
  req: NextRequest,
  { params }: { params: Promise<{ slug: string }> }
): Promise<NextResponse> {
  const { slug } = await params;
  const html = await fetchAnimePage(slug);
  const { animeList, pagination } = parseAnimePage(html, slug);

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
