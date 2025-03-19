import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger';

const fetchAnimePage = async (slug: string) => {
  const { data, contentType } = await fetchWithProxy(
    `https://alqanime.net/${slug}/`
  );

  if (!contentType || !contentType.includes('text/html')) {
    throw new Error('Failed to fetch anime detail data: Invalid content type');
  }

  return data;
};

const parseAnimeData = (html: string) => {
  const $ = cheerio.load(html);

  const extractText = (selector: string) => $(selector).text().trim();

  const title = extractText('.entry-title');
  const alternative_title = extractText('.alter');
  const poster =
    $('.thumb[itemprop="image"] img.lazyload').attr('data-src') || '';
  const poster2 =
    $('.bixbox.animefull .bigcover .ime img.lazyload').attr('data-src') || '';
  const type = extractText('.info-content .spe span:contains("Tipe:") a');
  const release_date = extractText(
    '.info-content .spe span:contains("Dirilis:")'
  );
  const status = extractText('.info-content .spe span:contains("Status:")');
  const synopsis = $('.entry-content p').text().trim();
  const studio = extractText('.info-content .spe span:contains("Studio:") a');

  const genres: { name: string; slug: string; anime_url: string }[] = [];
  $('.genxed a').each((_, element) => {
    const name = $(element).text().trim();
    const anime_url = $(element).attr('href') || '';
    const slug = anime_url.split('/').filter(Boolean).pop() || '';
    genres.push({ name, slug, anime_url });
  });

  const batch: {
    resolution: string;
    links: { name: string; url: string }[];
  }[] = [];
  const ova: { resolution: string; links: { name: string; url: string }[] }[] =
    [];
  const downloads: {
    resolution: string;
    links: { name: string; url: string }[];
  }[] = [];

  $('.soraddl.dlone .soraurl').each((_, element) => {
    const resolution = $(element).find('.res').text().trim();
    const links: { name: string; url: string }[] = [];
    $(element)
      .find('.slink a')
      .each((_, linkElement) => {
        const name = $(linkElement).text().trim();
        const url = $(linkElement).attr('href') || '';
        links.push({ name, url });
      });

    if (
      $(element)
        .closest('.soraddl')
        .find('.sorattl h3')
        .text()
        .toLowerCase()
        .includes('batch')
    ) {
      batch.push({ resolution, links });
    } else if (
      $(element)
        .closest('.soraddl')
        .find('.sorattl h3')
        .text()
        .toLowerCase()
        .includes('ova')
    ) {
      ova.push({ resolution, links });
    } else {
      downloads.push({ resolution, links });
    }
  });

  const producers: string[] = [];

  const recommendations: {
    title: string;
    slug: string;
    poster: string;
    status: string;
    type: string;
  }[] = [];
  $('.listupd .bs').each((_, element) => {
    const title = $(element).find('.ntitle').text().trim();
    const anime_url = $(element).find('a').attr('href') || '';
    const slug = anime_url.split('/').filter(Boolean).pop() || '';
    const poster =
      $(element).find('img').attr('data-src') ||
      $(element).find('img').attr('src') ||
      '';
    const status = $(element).find('.status').text().trim();
    const type = $(element).find('.typez').text().trim();
    recommendations.push({ title, slug, poster, status, type });
  });

  return {
    title,
    alternative_title,
    poster,
    poster2,
    type,
    release_date,
    status,
    synopsis,
    studio,
    genres,
    producers,
    recommendations,
    batch,
    ova,
    downloads,
  };
};

export async function GET(
  req: NextRequest,
  props: { params: Promise<{ slug: string }> }
) {
  const ip =
    req.headers.get('x-forwarded-for') ||
    req.headers.get('remote-addr') ||
    'unknown';
  const url = req.url;

  try {
    const { slug } = await props.params;
    const html = (await fetchAnimePage(slug)) as string;
    const animeData = parseAnimeData(html);

    logger.info('Request processed', {
      ip,
      url,
      slug,
    });

    return NextResponse.json({ status: 'Ok', data: animeData });
  } catch (error) {
    console.error('Error fetching anime data:', error);
    const errorMessage =
      error instanceof Error ? error.message : 'Failed to scrape data';
    return NextResponse.json({ message: errorMessage }, { status: 500 });
  }
}
