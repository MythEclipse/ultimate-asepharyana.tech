import { notFound } from 'next/navigation';
import { fetchWithFallback } from '../../../../../utils/url-utils';
import MangaPageClient from './MangaPageClient';
import type { KomikPaginationData } from '../../../../../utils/hooks/useKomik';

export const revalidate = 60;

async function Page({ params }: { params: Promise<{ pageNumber: string }> }) {
  const { pageNumber: pageNumberStr } = await params;
  const pageNumber = parseInt(pageNumberStr, 10);

  if (isNaN(pageNumber)) {
    notFound();
  }

  let initialData: KomikPaginationData | null = null;
  let initialError: string | null = null;

  try {
    const response = await fetchWithFallback(
      `/api/komik2/manga?page=${pageNumber}&order=update`,
      {
        revalidate,
        signal: AbortSignal.timeout(10000),
      },
    );

    initialData = await response.json();
  } catch (error) {
    initialError =
      error instanceof Error ? error.message : 'Failed to load manga data';
  }

  return (
    <MangaPageClient
      pageNumber={pageNumber}
      initialData={initialData}
      initialError={initialError}
    />
  );
}

export default Page;
