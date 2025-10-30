import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import UnifiedGrid from '../../../../../components/shared/UnifiedGrid';
import { ErrorStateCenter } from '../../../../../components/error/ErrorState';
import PaginationControls from '../../../../../components/shared/PaginationControls';
import { SectionHeader } from '../../../../../components/shared/SectionHeader';
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
      `/api/komik2/manga?page=${pageNumber}&order=update`,
      revalidate,
      10000
    ) as KomikData;
  } catch (err) {
    console.error('Failed to fetch manga data on server:', err);
    error = 'Failed to load manga data';
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
        <SectionHeader
          icon={BookOpen}
          title="Latest Manga"
          subtitle={`Halaman ${komikData.pagination.current_page} dari ${komikData.pagination.last_visible_page}`}
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
          pagination={komikData.pagination}
          baseUrl="/komik/manga/page"
          className="mt-8"
        />
      </div>
    </main>
  );
}

export default Page;
