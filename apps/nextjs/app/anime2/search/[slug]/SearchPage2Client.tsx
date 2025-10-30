'use client';

import React, { memo } from 'react';
import { getErrorMessage } from '../../../../utils/client-utils';
import SearchForm from '../../../../components/misc/SearchForm';
import MediaCard from '../../../../components/anime/MediaCard';
import ErrorLoadingDisplay from '../../../../components/shared/ErrorLoadingDisplay';
import {
  useAnime2Search,
  type SearchDetailData2,
  type Anime2,
} from '../../../../utils/hooks/useAnime2';

interface SearchPage2ClientProps {
  initialData: SearchDetailData2 | null;
  initialError: string | null;
  query: string;
}

function SearchPage2Client({
  initialData,
  initialError,
  query,
}: SearchPage2ClientProps) {
  const { data: swrData, error: swrError } = useAnime2Search(
    query,
    initialData || undefined,
  );

  const searchResults = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  if (displayError) {
    return <ErrorLoadingDisplay type="error" message={displayError} />;
  }

  if (!searchResults) {
    return <ErrorLoadingDisplay type="loading" skeletonType="grid" />;
  }

  if (!Array.isArray(searchResults.data) || searchResults.data.length === 0) {
    return (
      <ErrorLoadingDisplay
        type="no-data"
        title={`No results found for "${query}"`}
        message="Please try a different search query."
      />
    );
  }

  return (
    <main className="p-6">
      <h1 className="text-2xl font-bold mb-4">Search Results</h1>
      <SearchForm
        classname="w-full mb-8"
        initialQuery={query}
        baseUrl="/anime2"
      />
      <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 mb-4">
        {searchResults.data.map((anime: Anime2) => (
          <MediaCard
            key={anime.slug}
            title={anime.title}
            description={`${anime.status || 'Unknown status'} • ⭐${anime.rating || 'N/A'}`}
            imageUrl={anime.poster}
            linkUrl={`/anime2/detail/${anime.slug}`}
          />
        ))}
      </div>
    </main>
  );
}

export default memo(SearchPage2Client);
