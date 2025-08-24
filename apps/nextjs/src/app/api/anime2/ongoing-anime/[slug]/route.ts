import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import { corsHeaders } from '@/lib/corsHeaders';
import { withLogging } from '@/lib/api-wrapper';

async function fetchAnimePage(slug: string): Promise<string> {
  const response = await fetchWithProxy(
    `https://alqanime.net/advanced-search/page/${slug}/?status=ongoing&order=update`
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
    score: string;
    anime_url: string;
  }[] = [];

  $('.listupd .bs').each((index, element) => {
    const title = $(element).find('.ntitle').text().trim() || '';
    const slug = $(element).find('a').attr('href')?.split('/')[3] || '';
    const poster = $(element).find('img').attr('data-src') || '';
    const score = $(element).find('.numscore').text().trim() || 'N/A';
    const anime_url = $(element).find('a').attr('href') || '';

    animeList.push({
      title,
      slug,
      poster,
      score,
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
