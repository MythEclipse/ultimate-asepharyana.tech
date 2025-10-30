'use client';

import { memo } from 'react';
import { getErrorMessage } from '../../../../../utils/client-utils';
import Link from 'next/link';
import UnifiedGrid from '../../../../../components/shared/UnifiedGrid';
import { ErrorStateCenter } from '../../../../../components/error/ErrorState';
import PaginationControls from '../../../../../components/shared/PaginationControls';
import { BookOpen, AlertTriangle, ChevronRight } from 'lucide-react';
import { useManhuaPage, type KomikPaginationData } from '../../../../../utils/hooks/useKomik';

interface ManhuaPageClientProps {
  pageNumber: number;
  initialData: KomikPaginationData | null;
  initialError: string | null;
}

function ManhuaPageClient({
  pageNumber,
  initialData,
  initialError,
}: ManhuaPageClientProps) {
  const { data: swrData, error: swrError } = useManhuaPage(
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
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
          <div className="flex items-center gap-4">
            <div className="p-3 bg-purple-100 dark:bg-purple-900/50 rounded-xl">
              <BookOpen className="w-8 h-8 text-purple-600 dark:text-purple-400" />
            </div>
            <div>
              <h1 className="text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
                Latest Manhua
              </h1>
              <p className="text-zinc-600 dark:text-zinc-400 mt-1">
                Halaman {komikData.current_page} dari {komikData.last_page}
              </p>
            </div>
          </div>
          <Link
            href="/komik/manhua/page/1"
            className="flex items-center gap-2 text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 transition-colors px-4 py-2 rounded-lg bg-purple-50 dark:bg-purple-900/30"
          >
            Lihat Semua
            <ChevronRight className="w-5 h-5" />
          </Link>
        </div>

        {/* Manhua Grid */}
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
          baseUrl="/komik/manhua/page"
          className="mt-8"
        />
      </div>
    </main>
  );
}

export default memo(ManhuaPageClient);
