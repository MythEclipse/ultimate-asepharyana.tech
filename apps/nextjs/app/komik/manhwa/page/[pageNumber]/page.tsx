import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import ComicCard from '../../../../../components/komik/ComicGrid';
import {
  BookOpen,
  ChevronLeft,
  ChevronRight,
  AlertTriangle,
} from 'lucide-react';
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
      `/api/komik2/manhwa?page=${pageNumber}&order=update`,
      revalidate,
      10000
    );
  } catch (err) {
    console.error('Failed to fetch manhwa data on server:', err);
    error = 'Failed to load manhwa data';
  }

  if (error || !komikData) {
    return (
      <div className="min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center">
        <div className="max-w-2xl text-center">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex flex-col items-center gap-4">
            <AlertTriangle className="w-12 h-12 text-red-600 dark:text-red-400" />
            <h2 className="text-2xl font-bold text-red-800 dark:text-red-200">
              Gagal Memuat Data
            </h2>
            <p className="text-red-700 dark:text-red-300">
              Silakan coba kembali beberapa saat lagi
            </p>
          </div>
        </div>
      </div>
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
                Halaman {komikData.pagination.current_page} dari{' '}
                {komikData.pagination.last_visible_page}
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

        {/* manhwa Grid */}
        <div className="flex flex-col items-center p-4">
          <div className="grid grid-cols-3 lg:grid-cols-5 gap-4 w-full">
            {komikData.data.map((komik) => (
              <ComicCard key={komik.slug} komik={komik} />
            ))}
          </div>
        </div>

        {/* Pagination */}
        <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
          <Link
            href={`/komik/manhwa/page/${komikData.pagination.has_previous_page ? pageNumber - 1 : 1}`}
            className={`${
              !komikData.pagination.has_previous_page
                ? 'opacity-50 pointer-events-none'
                : ''
            }`}
          >
            <button
              disabled={!komikData.pagination.has_previous_page}
              className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2"
            >
              <ChevronLeft className="w-5 h-5" />
              Previous
            </button>
          </Link>
          <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400">
            Page {komikData.pagination.current_page} of{' '}
            {komikData.pagination.last_visible_page}
          </span>

          <Link
            href={`/komik/manhwa/page/${komikData.pagination.has_next_page ? pageNumber + 1 : pageNumber}`}
            className={`${
              !komikData.pagination.has_next_page
                ? 'opacity-50 pointer-events-none'
                : ''
            }`}
          >
            <button
              disabled={!komikData.pagination.has_next_page}
              className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2"
            >
              Next
              <ChevronRight className="w-5 h-5" />
            </button>
          </Link>
        </div>
      </div>
    </main>
  );
}

export default Page;
