import { notFound } from 'next/navigation';
import { fetchWithFallback } from '../../../../../utils/url-utils';
import ManhuaPageClient from './ManhuaPageClient';
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
      `/api/komik2/manhua?page=${pageNumber}&order=update`,
      {
        revalidate,
        signal: AbortSignal.timeout(10000),
      },
    );

    initialData = await response.json();
  } catch (error) {
    initialError =
      error instanceof Error ? error.message : 'Failed to load manhua data';
  }

  return (
    <ManhuaPageClient
      pageNumber={pageNumber}
      initialData={initialData}
      initialError={initialError}
    />
  );
}

export default Page;
