'use client';

import React from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import {
  ChevronLeft,
  ChevronRight,
  BookOpen,
} from 'lucide-react';

interface NavigationButtonsProps {
  listChapter: string;
  nextChapterId?: string;
}

export default function NavigationButtons({ listChapter, nextChapterId }: NavigationButtonsProps) {
  const router = useRouter();

  return (
    <div className="p-6">
      <div className="flex flex-col md:flex-row gap-4 items-stretch">
        {/* Previous Button */}
        <div className="flex-1 flex justify-center md:justify-start">
          <button
            onClick={() => router.back()}
            className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors"
          >
            <ChevronLeft className="w-5 h-5" />
            <span className="hidden sm:inline">Kembali</span>
            <span className="sm:hidden">Kembali</span>
          </button>
        </div>

        {/* Center Button */}
        <div className="flex-1 flex justify-center">
          <Link
            href={`/komik/detail/${listChapter}`}
            className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
          >
            <BookOpen className="w-5 h-5" />
            <span className="hidden sm:inline">Daftar Chapter</span>
            <span className="sm:hidden">Daftar</span>
          </Link>
        </div>

        {/* Next Button */}
        <div className="flex-1 flex justify-center md:justify-end">
          {nextChapterId ? (
            <Link
              scroll
              href={`/komik/chapter/${nextChapterId}`}
              className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors"
            >
              <span className="hidden sm:inline">Chapter Selanjutnya</span>
              <span className="sm:hidden">Selanjutnya</span>
              <ChevronRight className="w-5 h-5" />
            </Link>
          ) : (
            <div className="w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center">
              <ChevronRight className="w-5 h-5" />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
