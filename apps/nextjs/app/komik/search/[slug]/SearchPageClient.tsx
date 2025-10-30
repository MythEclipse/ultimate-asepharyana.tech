'use client';

import { memo } from 'react';
import { getErrorMessage } from '../../../../utils/client-utils';
import SearchForm from '../../../../components/misc/SearchForm';
import CardA from '../../../../components/anime/MediaCard';
import { Info } from 'lucide-react';
import {
  useKomikSearch,
  type KomikSearchData,
} from '../../../../utils/hooks/useKomik';

interface SearchPageClientProps {
  query: string;
  initialData: KomikSearchData | null;
  initialError: string | null;
}

function SearchPageClient({
  query,
  initialData,
  initialError,
}: SearchPageClientProps) {
  const { data: swrData, error: swrError } = useKomikSearch(
    query,
    initialData || undefined,
  );

  const searchResults = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  const hasResults = searchResults?.data && searchResults.data.length > 0;

  return (
    <main className="p-6">
      <h1 className="text-2xl font-bold mb-4">Search Results</h1>
      <SearchForm
        classname="w-full mb-8"
        initialQuery={query}
        baseUrl="/komik"
      />

      {displayError ? (
        <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex items-center gap-4">
          <Info className="w-8 h-8 text-red-600 dark:text-red-400" />
          <div>
            <h2 className="text-xl font-medium text-red-800 dark:text-red-200">
              Terjadi Kesalahan
            </h2>
            <p className="text-red-600 dark:text-red-400 text-sm">
              {displayError}
            </p>
          </div>
        </div>
      ) : hasResults && searchResults ? (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4">
          {searchResults.data.map((komik) => (
            <CardA
              key={komik.slug}
              title={komik.title}
              description={`${komik.type || 'Unknown'} • Chapter ${komik.chapter || 'N/A'}`}
              imageUrl={komik.poster}
              linkUrl={`/komik/detail/${komik.slug}`}
            />
          ))}
        </div>
      ) : (
        <div className="p-6 bg-blue-100 dark:bg-blue-900/30 rounded-2xl flex items-center gap-4">
          <Info className="w-8 h-8 text-blue-600 dark:text-blue-400" />
          <h2 className="text-xl font-medium text-blue-800 dark:text-blue-200">
            No results found for &quot;{query}&quot;
          </h2>
        </div>
      )}
    </main>
  );
}

export default memo(SearchPageClient);
