'use client';

import { memo } from 'react';
import { getErrorMessage } from '../../../../utils/client-utils';
import NavigationButtons from '../NavigationButtons';
import { ImageWithFallback } from '../../../../components/shared/ImageWithFallback';
import { ErrorStateCenter } from '../../../../components/error/ErrorState';
import { AlertTriangle } from 'lucide-react';
import { useChapterDetail, type ChapterDetail } from '../../../../utils/hooks/useKomik';

interface ChapterPageClientProps {
  chapterId: string;
  initialData: ChapterDetail | null;
  initialError: string | null;
}

function ChapterPageClient({
  chapterId,
  initialData,
  initialError,
}: ChapterPageClientProps) {
  const { data: swrData, error: swrError } = useChapterDetail(
    chapterId,
    initialData || undefined
  );

  const chapter = swrData || initialData;
  const displayError = getErrorMessage(swrError) || initialError;

  if (displayError || !chapter) {
    return (
      <ErrorStateCenter
        icon={AlertTriangle}
        title="Gagal Memuat Chapter"
        message={displayError || 'Chapter tidak ditemukan'}
        type="error"
      />
    );
  }

  return (
    <main className="min-h-screen p-4 md:p-6 bg-background dark:bg-dark">
      {/* Navigation Top */}
      <div className="max-w-7xl mx-auto mb-6 space-y-4">
        <h1 className="text-2xl md:text-3xl font-bold text-center bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
          {chapter.title}
        </h1>

        <NavigationButtons
          listChapter={chapterId}
          nextChapterId={chapter.next_chapter}
        />
      </div>

      {/* Image Content */}
      <div className="max-w-4xl mx-auto space-y-4">
        {chapter.images?.map((image, index) => (
          <div key={`${image}-${index}`} className="relative group">
            <ImageWithFallback imageUrl={image} index={index} />
            <div className="absolute bottom-2 right-2 bg-black/50 text-white px-3 py-1 rounded-md text-sm opacity-0 group-hover:opacity-100 transition-opacity">
              Halaman {index + 1}
            </div>
          </div>
        ))}
      </div>

      {/* Navigation Bottom */}
      <div className="p-6">
        <NavigationButtons
          listChapter={chapterId}
          nextChapterId={chapter.next_chapter}
        />
      </div>
    </main>
  );
}

export default memo(ChapterPageClient);
