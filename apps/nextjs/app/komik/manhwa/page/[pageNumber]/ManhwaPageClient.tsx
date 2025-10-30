'use client';

import { memo } from 'react';
import { getErrorMessage } from '../../../../../utils/client-utils';
import Link from 'next/link';
import UnifiedGrid from '../../../../../components/shared/UnifiedGrid';
import { ErrorStateCenter } from '../../../../../components/error/ErrorState';
import PaginationControls from '../../../../../components/shared/PaginationControls';
import { BookOpen, AlertTriangle, ChevronRight } from 'lucide-react';
import { useManhwaPage, type KomikPaginationData } from '../../../../../utils/hooks/useKomik';

interface ManhwaPageClientProps {
  pageNumber: number;
  initialData: KomikPaginationData | null;
  initialError: string | null;
}

function ManhwaPageClient({
  pageNumber,
  initialData,
  initialError,
}: ManhwaPageClientProps) {
  const { data: swrData, error: swrError } = useManhwaPage(
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

  if (!komikData.data || komikData.data.length === 0) {
    return (
      <ErrorStateCenter
        icon={AlertTriangle}
        title="Tidak Ada Data"
        message="Tidak ada manhwa yang ditemukan"
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
                Latest Manhwa
              </h1>
              <p className="text-zinc-600 dark:text-zinc-400 mt-1">
                Halaman {komikData.pagination.current_page} dari {komikData.pagination.last_visible_page}
              </p>
            </div>
          </div>
          <Link
            href="/komik/manhwa/page/1"
            className="flex items-center gap-2 text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 transition-colors px-4 py-2 rounded-lg bg-purple-50 dark:bg-purple-900/30"
          >
            Lihat Semua
            <ChevronRight className="w-5 h-5" />
          </Link>
        </div>

        {/* Manhwa Grid */}
        <div className="flex flex-col items-center p-4">
          <UnifiedGrid items={komikData.data} itemType="komik" />
        </div>

        {/* Pagination */}
        <PaginationControls
          pagination={komikData.pagination}
          baseUrl="/komik/manhwa/page"
          className="mt-8"
        />
      </div>
    </main>
  );
}

export default memo(ManhwaPageClient);
