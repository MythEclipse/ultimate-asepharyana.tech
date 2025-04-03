'use client';

import React from 'react';
import useSWR from 'swr';
import { Link } from 'next-view-transitions';
import {
  ChevronLeft,
  ChevronRight,
  BookOpen,
  AlertTriangle,
} from 'lucide-react';
import { ImageWithFallback } from '@/components/ImageWithFallback';
import { Skeleton } from '@/components/ui/skeleton';
import { useParams } from 'next/navigation';

interface ChapterDetail {
  title: string;
  prev_chapter_id: string;
  next_chapter_id: string;
  list_chapter: string;
  images: string[];
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export default function ChapterPage() {
  const { chapterId } = useParams();
  const {
    data: chapter,
    error,
    isLoading,
  } = useSWR<ChapterDetail>(
    `/api/komik/chapter?chapter_url=${chapterId}`,
    fetcher,
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
    }
  );

  if (error) {
    return (
      <main className='min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center'>
        <div className='max-w-md text-center'>
          <div className='p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex flex-col items-center gap-4'>
            <AlertTriangle className='w-12 h-12 text-red-600 dark:text-red-400' />
            <h1 className='text-2xl font-bold text-red-800 dark:text-red-200'>
              Gagal Memuat Chapter
            </h1>
            <p className='text-red-700 dark:text-red-300'>
              Silakan coba kembali beberapa saat lagi
            </p>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className='min-h-screen p-4 md:p-6 bg-background dark:bg-dark'>
      {/* Navigation Top */}
      <div className='max-w-7xl mx-auto mb-6 space-y-4'>
        <h1 className='text-2xl md:text-3xl font-bold text-center bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent'>
          {isLoading ? (
        <Skeleton className='h-8 w-64 mx-auto' />
          ) : (
        chapter?.title
          )}
        </h1>

        <div className='p-6'>
          <div className='flex flex-col md:flex-row gap-4 items-stretch'>
        {/* Previous Button */}
        <div className='flex-1 flex justify-center md:justify-start'>
          {isLoading ? (
            <Skeleton className='w-full md:w-auto h-10 px-6 py-2 rounded-lg' />
          ) : chapter?.prev_chapter_id ? (
            <Link
          scroll
          href={`/komik/chapter/${chapter.prev_chapter_id}`}
          className='w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors'
            >
          <ChevronLeft className='w-5 h-5' />
          <span className='hidden sm:inline'>Chapter Sebelumnya</span>
          <span className='sm:hidden'>Sebelumnya</span>
            </Link>
          ) : (
            <div className='w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center'>
          <ChevronLeft className='w-5 h-5' />
            </div>
          )}
        </div>

        {/* Center Button */}
        <div className='flex-1 flex justify-center'>
          {isLoading ? (
            <Skeleton className='w-full md:w-auto h-10 px-6 py-2 rounded-lg' />
          ) : (
            <Link
          href={`/komik/detail/${chapter?.list_chapter}`}
          className='w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors'
            >
          <BookOpen className='w-5 h-5' />
          <span className='hidden sm:inline'>Daftar Chapter</span>
          <span className='sm:hidden'>Daftar</span>
            </Link>
          )}
        </div>

        {/* Next Button */}
        <div className='flex-1 flex justify-center md:justify-end'>
          {isLoading ? (
            <Skeleton className='w-full md:w-auto h-10 px-6 py-2 rounded-lg' />
          ) : chapter?.next_chapter_id ? (
            <Link
          scroll
          href={`/komik/chapter/${chapter.next_chapter_id}`}
          className='w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors'
            >
          <span className='hidden sm:inline'>Chapter Selanjutnya</span>
          <span className='sm:hidden'>Selanjutnya</span>
          <ChevronRight className='w-5 h-5' />
            </Link>
          ) : (
            <div className='w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center'>
          <ChevronRight className='w-5 h-5' />
            </div>
          )}
        </div>
          </div>
        </div>
      </div>

      {/* Image Content */}
      <div className='max-w-4xl mx-auto space-y-4'>
        {isLoading
          ? Array.from({ length: 5 }).map((_, i) => (
              <div key={i} className='relative group w-full'>
                <Skeleton className='w-full h-auto aspect-[700/1000] rounded-xl border border-zinc-200 dark:border-zinc-700' />
                <Skeleton className='absolute bottom-2 right-2 h-6 w-20 rounded-md' />
              </div>
            ))
          : chapter?.images?.map((image, index) => (
              <div key={`${image}-${index}`} className='relative group'>
                <ImageWithFallback
                  imageUrl={image}
                  index={index}
                  // className="w-full h-auto rounded-xl shadow-lg border border-zinc-200 dark:border-zinc-700"
                />
                <div className='absolute bottom-2 right-2 bg-black/50 text-white px-3 py-1 rounded-md text-sm opacity-0 group-hover:opacity-100 transition-opacity'>
                  Halaman {index + 1}
                </div>
              </div>
            ))}
      </div>

      {/* Navigation Bottom (Fixed) */}
      {/* <div className="fixed bottom-0 left-0 right-0 bg-background/95 dark:bg-dark/95 backdrop-blur border-t border-zinc-200 dark:border-zinc-800">
        <div className="max-w-7xl mx-auto p-4">
          <div className="flex justify-between items-center gap-4">
            <div className="flex-1 flex justify-start">
              {chapter?.prev_chapter_id && (
                <Link
                  scroll
                  href={`/komik/chapter/${chapter.prev_chapter_id}`}
                  className="flex items-center gap-2 text-sm px-4 py-2 bg-zinc-100 dark:bg-zinc-800 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
                >
                  <ChevronLeft className="w-4 h-4" />
                  Sebelumnya
                </Link>
              )}
            </div>

            <Link
              href={`/komik/detail/${chapter?.list_chapter}`}
              className="flex items-center gap-2 text-sm px-4 py-2 bg-zinc-100 dark:bg-zinc-800 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
            >
              <BookOpen className="w-4 h-4" />
            </Link>

            <div className="flex-1 flex justify-end">
              {chapter?.next_chapter_id && (
                <Link
                  scroll
                  href={`/komik/chapter/${chapter.next_chapter_id}`}
                  className="flex items-center gap-2 text-sm px-4 py-2 bg-zinc-100 dark:bg-zinc-800 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
                >
                  Selanjutnya
                  <ChevronRight className="w-4 h-4" />
                </Link>
              )}
            </div>
          </div>
        </div>
      </div> */}
      <div className='p-6'>
        <div className='flex flex-col md:flex-row gap-4 items-stretch'>
          {/* Previous Button */}
          <div className='flex-1 flex justify-center md:justify-start'>
            {chapter?.prev_chapter_id ? (
              <Link
                scroll
                href={`/komik/chapter/${chapter.prev_chapter_id}`}
                className='w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors'
              >
                <ChevronLeft className='w-5 h-5' />
                <span className='hidden sm:inline'>Chapter Sebelumnya</span>
                <span className='sm:hidden'>Sebelumnya</span>
              </Link>
            ) : (
              <div className='w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center'>
                <ChevronLeft className='w-5 h-5' />
              </div>
            )}
          </div>

          {/* Center Button */}
          <div className='flex-1 flex justify-center'>
            <Link
              href={`/komik/detail/${chapter?.list_chapter}`}
              className='w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors'
            >
              <BookOpen className='w-5 h-5' />
              <span className='hidden sm:inline'>Daftar Chapter</span>
              <span className='sm:hidden'>Daftar</span>
            </Link>
          </div>

          {/* Next Button */}
          <div className='flex-1 flex justify-center md:justify-end'>
            {chapter?.next_chapter_id ? (
              <Link
                scroll
                href={`/komik/chapter/${chapter.next_chapter_id}`}
                className='w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors'
              >
                <span className='hidden sm:inline'>Chapter Selanjutnya</span>
                <span className='sm:hidden'>Selanjutnya</span>
                <ChevronRight className='w-5 h-5' />
              </Link>
            ) : (
              <div className='w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center'>
                <ChevronRight className='w-5 h-5' />
              </div>
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
