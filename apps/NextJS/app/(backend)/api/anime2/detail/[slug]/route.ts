import * as cheerio from 'cheerio';
import { NextRequest, NextResponse } from 'next/server';
import { fetchWithProxy } from '@/lib/fetchWithProxy';
import logger from '@/lib/logger'; // Make sure to import your logger

const fetchAnimePage = async (slug: string) => {
  const { data, contentType } = await fetchWithProxy(
    `https://s4.nontonanimeid.boats/anime/${slug}/`
  );

  if (!contentType || !contentType.includes('text/html')) {
    throw new Error('Failed to fetch anime detail data: Invalid content type');
  }

  return data;
};

const parseAnimeData = (html: string, slug: string) => {
  const $ = cheerio.load(html);

  const extractText = (selector: string) => $(selector).text().trim();

  const title = extractText('.entry-title');
  const alternative_title =
    $('.bottomtitle .infoseries:contains("English:")')
      .text()
      .replace('English:', '')
      .trim() || '';
  const poster = $('.kotakseries .poster img').attr('src') || '';
  const type = $('.scoreseries .typeseries').text().trim() || '';
  const release_date =
    $('.bottomtitle .infoseries:contains("Aired:")')
      .text()
      .replace('Aired:', '')
      .trim() || '';
  const status = $('.extra .statusseries').text().trim() || '';
  const synopsis = $('div.entry-content.seriesdesc > p').text().trim();
  const studio =
    $('.bottomtitle .infoseries:contains("Studios:")')
      .text()
      .replace('Studios:', '')
      .trim() || '';

  const genres: { name: string; slug: string; anime_url: string }[] = [];
  $('.tagline a').each((_, element) => {
    const name = $(element).text().trim();
    const anime_url = $(element).attr('href') || '';
    const slug = anime_url.split('/').filter(Boolean).pop() || '';
    genres.push({ name, slug, anime_url });
  });

  const episode_lists: { episode: string; slug: string }[] = [];
  const batch: { episode: string; slug: string }[] = [];
  $('.misha_posts_wrap2 li').each((_, element) => {
    const episode = $(element).find('.t1 a').text().trim();
    const href = $(element).find('.t1 a').attr('href'); // Ambil atribut href
    let episodeSlug = '';
    if (href) {
      const segments = href.split('/');
      episodeSlug = segments.pop() || segments[segments.length - 1] || ''; // Ambil slug dari URL
    }

    if (episode.toLowerCase().includes('batch')) {
      batch.push({ episode, slug: episodeSlug });
    } else {
      episode_lists.push({ episode, slug: episodeSlug });
    }
  });
  // Add missing episodes up to episode 1
  const episodeNumbers = episode_lists.map((ep) =>
    parseInt(ep.slug.match(/\d+/)?.[0] || '0', 10)
  );
  const maxEpisode = Math.max(...episodeNumbers);

  for (let i = maxEpisode; i >= 1; i--) {
    if (!episodeNumbers.includes(i)) {
      episode_lists.push({
        episode: `${title} Episode ${i} Sub Indo`,
        slug: `${slug}-episode-${i}`,
      });
    }
  }

  // Sort episodes in descending order
  episode_lists.sort((a, b) => {
    const episodeA = parseInt(a.slug.match(/\d+/)?.[0] || '0', 10);
    const episodeB = parseInt(b.slug.match(/\d+/)?.[0] || '0', 10);
    return episodeB - episodeA;
  });

  const producers: string[] = []; // Update if producers are available in the new structure

  const recommendations: {
    title: string;
    slug: string;
    poster: string;
    status: string;
    type: string;
  }[] = [];
  // Update this part if there are recommendations in the new HTML structure

  return {
    title,
    alternative_title,
    poster,
    type,
    release_date,
    status,
    synopsis,
    studio,
    genres,
    producers,
    recommendations,
    batch, // Batch data terpisah
    episode_lists, // Episode reguler
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
    const animeData = parseAnimeData(html, slug);

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
