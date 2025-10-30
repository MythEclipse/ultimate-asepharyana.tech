import { fetchWithFallback } from '../../../../utils/url-utils';
import SearchPageClient from './SearchPageClient';
import type { KomikSearchData } from '../../../../utils/hooks/useKomik';

export const revalidate = 60;

export default async function SearchPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const query = decodeURIComponent(slug);

  let initialData: KomikSearchData | null = null;
  let initialError: string | null = null;

  try {
    const response = await fetchWithFallback(
      `/api/komik2/search?query=${encodeURIComponent(query)}&page=1`,
      {
        revalidate,
        signal: AbortSignal.timeout(10000),
      },
    );

    const result = await response.json();
    initialData = result.data || { data: [] };
  } catch (error) {
    initialError =
      error instanceof Error
        ? error.message
        : 'Terjadi kesalahan saat mencari komik';
  }

  return (
    <SearchPageClient
      query={query}
      initialData={initialData}
      initialError={initialError}
    />
  );
}
