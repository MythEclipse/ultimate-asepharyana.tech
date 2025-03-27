import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import { BaseUrl } from '@/lib/url';
import { ComicCard } from '@/components/card/ComicCard';
import { BookOpen, ChevronLeft, ChevronRight, AlertTriangle } from 'lucide-react';

export const dynamic = 'force-dynamic';

interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  next_page: number | null;
  has_previous_page: boolean;
  previous_page: number | null;
}
interface KomikData {
  data: Comic[];
  pagination: Pagination;
}

interface Comic {
  title: string;
  image: string;
  chapter: string;
  date: string;
  score: string;
  type: string;
  komik_id: string;
}

export default async function Page(props: {
  params: Promise<{ pageNumber: string }>;
}) {
  const params = await props.params;
  const pageNumber = parseInt(params.pageNumber, 10);

  if (isNaN(pageNumber)) {
    notFound();
  }

  let komikData: KomikData;
  try {
    const response = await fetch(
      `${BaseUrl}/api/komik/manhua?page=${pageNumber}&order=update`,
      { next: { revalidate: 60 } }
    );
    if (!response.ok) throw new Error('Failed to fetch');
    komikData = await response.json();
  } catch {
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
          <div className="p-3 bg-green-100 dark:bg-green-900/50 rounded-xl">
              <BookOpen className="w-8 h-8 text-green-600 dark:text-green-400" />
            </div>
            <div>
            <h1 className="text-3xl font-bold bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">
                Latest Manhua
              </h1>
              <p className="text-zinc-600 dark:text-zinc-400 mt-1">
                Halaman {pageNumber} dari {komikData.pagination.last_visible_page}
              </p>
            </div>
          </div>
          <Link
            href="/komik/manhua/page/1"
            className="flex items-center gap-2 text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 transition-colors px-4 py-2 rounded-lg bg-green-50 dark:bg-green-900/30"
          >
            Lihat Semua
            <ChevronRight className="w-5 h-5" />
          </Link>
        </div>

        {/* Comic Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {komikData.data.map((comic) => (
           
            <ComicCard
              key={comic.komik_id}
              comic={comic} />
          ))}
        </div>

        {/* Pagination */}
        <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
          <Link
            href={`/komik/manhua/page/${komikData.pagination.has_previous_page ? pageNumber - 1 : 1}`}
            className={`${!komikData.pagination.has_previous_page ? 'opacity-50 pointer-events-none' : ''
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
            Page {komikData.pagination.current_page} of {komikData.pagination.last_visible_page}
          </span>

          <Link
            href={`/komik/manhua/page/${komikData.pagination.has_next_page ? pageNumber + 1 : pageNumber}`}
            className={`${!komikData.pagination.has_next_page ? 'opacity-50 pointer-events-none' : ''
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