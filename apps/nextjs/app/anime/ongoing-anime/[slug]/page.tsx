import { notFound } from 'next/navigation';
import { APIURLSERVER } from '../../../../lib/url';
import { CompleteAnimeData } from '../../../../types/anime';
import OngoingAnimePageClient from './OngoingAnimePageClient';

export const revalidate = 60;

async function AnimePage({ params }: { params: Promise<{ slug: string }>; }) {
  const { slug } = await params;

  if (!slug) {
    notFound();
  }

  let initialData: CompleteAnimeData | null = null;
  let initialError: string | null = null;

  try {
    const url = `/api/anime/ongoing-anime/${slug}`;
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    initialData = await response.json();
  } catch (err) {
    console.error('Failed to fetch ongoing anime data on server:', err);
    initialError = 'Failed to load ongoing anime data';
  }

  return (
    <OngoingAnimePageClient
      initialData={initialData}
      initialError={initialError}
      slug={slug}
    />
  );
}

export default AnimePage;
