import NavigationButtons from '../NavigationButtons';
import { ImageWithFallback } from '../../../../components/shared/ImageWithFallback';
import { ErrorStateCenter } from '../../../../components/error/ErrorState';
import { AlertTriangle } from 'lucide-react';
import { fetchKomikData } from '../../../../lib/komikFetcher';

interface ChapterDetail {
  title: string;
  prev_chapter_id: string;
  next_chapter_id: string;
  list_chapter: string;
  images: string[];
}

export const revalidate = 60;

export default async function ChapterPage({
  params,
}: {
  params: Promise<{ chapterId: string }>;
}) {
  const { chapterId } = await params;

  let chapter: ChapterDetail | null = null;

  try {
    const response = await fetchKomikData(
      `/api/komik2/chapter?chapter_url=${chapterId}`,
      revalidate,
      10000
    );
    chapter = (response as { data: ChapterDetail }).data;
  } catch (_e) {
    return (
      <ErrorStateCenter
        icon={AlertTriangle}
        title="Gagal Memuat Chapter"
        message="Silakan coba kembali beberapa saat lagi"
        type="error"
      />
    );
  }

  if (!chapter) {
    return (
      <ErrorStateCenter
        icon={AlertTriangle}
        title="Gagal Memuat Chapter"
        message="Chapter tidak ditemukan"
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
          listChapter={chapter.list_chapter}
          nextChapterId={chapter.next_chapter_id}
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
          listChapter={chapter.list_chapter}
          nextChapterId={chapter.next_chapter_id}
        />
      </div>
    </main>
  );
}
