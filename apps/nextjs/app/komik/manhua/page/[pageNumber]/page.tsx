import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import UnifiedGrid from '../../../../../components/shared/UnifiedGrid';
import { ErrorStateCenter } from '../../../../../components/error/ErrorState';
import PaginationControls from '../../../../../components/shared/PaginationControls';
import { BookOpen, AlertTriangle, ChevronRight } from 'lucide-react';
import { fetchKomikData } from '../../../../../lib/komikFetcher';

export const revalidate = 60;

interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}

interface KomikData {
  data: Komik[];
  pagination: Pagination;
}

export interface Komik {
  title: string;
  poster: string;
  chapter: string;
  date: string;
  reader_count: string;
  type: string;
  slug: string;
}

async function Page({ params }: { params: Promise<{ pageNumber: string }> }) {
  const { pageNumber: pageNumberStr } = await params;
  const pageNumber = parseInt(pageNumberStr, 10);

  if (isNaN(pageNumber)) {
    notFound();
  }

  let komikData: KomikData | null = null;
  let error: string | null = null;

  try {
    komikData = await fetchKomikData(
      `/api/komik2/manhua?page=${pageNumber}&order=update`,
      revalidate,
      10000
    ) as KomikData;
  } catch (err) {
    console.error('Failed to fetch manhua data on server:', err);
    error = 'Failed to load manhua data';
  }

  if (error || !komikData) {
    return (
      <ErrorStateCenter
        icon={AlertTriangle}
        title="Gagal Memuat Data"
        message="Silakan coba kembali beberapa saat lagi"
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
                Halaman {komikData.pagination.current_page} dari{' '}
                {komikData.pagination.last_visible_page}
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

        {/* manhua Grid */}
        <div className="flex flex-col items-center p-4">
          <UnifiedGrid items={komikData.data} itemType="komik" />
        </div>

        {/* Pagination */}
        <PaginationControls
          pagination={komikData.pagination}
          baseUrl="/komik/manhua/page"
          className="mt-8"
        />
      </div>
    </main>
  );
}

export default Page;
