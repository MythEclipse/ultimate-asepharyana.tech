'use client';

import { memo } from 'react';
import { getErrorMessage } from '../../../../../utils/client-utils';
import Link from 'next/link';
import UnifiedGrid from '../../../../../components/shared/UnifiedGrid';
import { ErrorStateCenter } from '../../../../../components/error/ErrorState';
import PaginationControls from '../../../../../components/shared/PaginationControls';
import { SectionHeader } from '../../../../../components/shared/SectionHeader';
import { BookOpen, AlertTriangle, ChevronRight } from 'lucide-react';
import { useMangaPage, type KomikPaginationData } from '../../../../../utils/hooks/useKomik';

interface MangaPageClientProps {
  pageNumber: number;
  initialData: KomikPaginationData | null;
  initialError: string | null;
}

function MangaPageClient({
  pageNumber,
  initialData,
  initialError,
}: MangaPageClientProps) {
  const { data: swrData, error: swrError } = useMangaPage(
    pageNumber,
    initialData || undefined
  );

  const komikData = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  if (displayError || !komikData) {
    return (
      <ErrorStateCenter
        icon={AlertTriangle}
        title="Gagal Memuat Data"
        message={displayError || 'Silakan coba kembali beberapa saat lagi'}
        type="error"
      />
    );
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
        {/* Header Section */}
        <SectionHeader
          icon={BookOpen}
          title="Latest Manga"
          subtitle={`Halaman ${komikData.current_page} dari ${komikData.last_page}`}
          color="purple"
          action={
            <Link
              href="/komik/manga/page/1"
              className="flex items-center gap-2 text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 transition-colors px-4 py-2 rounded-lg bg-purple-50 dark:bg-purple-900/30"
            >
              Lihat Semua
              <ChevronRight className="w-5 h-5" />
            </Link>
          }
        />

        {/* Manga Grid */}
        <div className="flex flex-col items-center p-4">
          <UnifiedGrid items={komikData.data} itemType="komik" />
        </div>

        {/* Pagination */}
        <PaginationControls
          pagination={{
            current_page: komikData.current_page,
            last_visible_page: komikData.last_page,
            has_next_page: komikData.current_page < komikData.last_page,
            next_page: komikData.current_page < komikData.last_page ? komikData.current_page + 1 : null,
            has_previous_page: komikData.current_page > 1,
            previous_page: komikData.current_page > 1 ? komikData.current_page - 1 : null,
          }}
          baseUrl="/komik/manga/page"
          className="mt-8"
        />
      </div>
    </main>
  );
}

export default memo(MangaPageClient);
