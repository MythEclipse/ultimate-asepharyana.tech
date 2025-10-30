'use client';

import React, { memo } from 'react';
import { getErrorMessage } from '../../../../utils/client-utils';
import Link from 'next/link';
import {
  ChevronLeft,
  ChevronRight,
  CheckCircle,
} from 'lucide-react';
import UnifiedGrid from '../../../../components/shared/UnifiedGrid';
import ErrorLoadingDisplay from '../../../../components/shared/ErrorLoadingDisplay';
import { useCompleteAnime2, type CompleteAnimeData2, type Pagination2 } from '../../../../utils/hooks/useAnime2';

interface CompleteAnime2PageClientProps {
  initialData: CompleteAnimeData2 | null;
  initialError: string | null;
  slug: string;
}

function CompleteAnime2PageClient({
  initialData,
  initialError,
  slug,
}: CompleteAnime2PageClientProps) {
  const { data: swrData, error: swrError } = useCompleteAnime2(slug, initialData || undefined);

  const data = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  if (displayError) {
    return <ErrorLoadingDisplay type="error" message={displayError} />;
  }

  if (!data) {
    return <ErrorLoadingDisplay type="loading" skeletonType="grid" />;
  }

  if (!Array.isArray(data.data) || data.data.length === 0) {
    return (
      <ErrorLoadingDisplay
        type="no-data"
        title="No Complete Anime Available"
        message="There are no complete anime to display at this time."
      />
    );
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
              <CheckCircle className="w-6 h-6 text-green-600 dark:text-green-400" />
            </div>
            <h2 className="text-2xl font-bold bg-gradient-to-r from-green-600 to-purple-600 bg-clip-text text-transparent">
              Complete Anime
            </h2>
          </div>
        </div>

        <UnifiedGrid items={data.data} itemType="anime" isAnime2={true} />

        <PaginationComponent pagination={data.pagination} />
      </div>
    </main>
  );
}

const PaginationComponent = ({ pagination }: { pagination?: Pagination2 }) => {
  if (!pagination) return null;

  return (
    <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
      {pagination?.has_previous_page && pagination?.previous_page !== null && (
        <Link href={`/anime2/complete-anime/${pagination.previous_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            <ChevronLeft className="w-5 h-5" />
            Sebelumnya
          </button>
        </Link>
      )}

      <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4">
        Halaman {pagination?.current_page ?? 1} dari {pagination?.last_visible_page ?? 1}
      </span>

      {pagination?.has_next_page && pagination?.next_page !== null && (
        <Link href={`/anime2/complete-anime/${pagination.next_page}`}>
          <button className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2">
            Selanjutnya
            <ChevronRight className="w-5 h-5" />
          </button>
        </Link>
      )}
    </div>
  );
};

export default memo(CompleteAnime2PageClient);
