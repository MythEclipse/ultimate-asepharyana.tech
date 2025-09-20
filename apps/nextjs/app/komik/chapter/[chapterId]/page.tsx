import { APIURLSERVER } from '../../../../lib/url';
import NavigationButtons from '../NavigationButtons';
import { ImageWithFallback } from '../../../../components/shared/ImageWithFallback';
import { AlertTriangle } from 'lucide-react';

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
  const fetchData = async (url: string) => {
    const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      next: { revalidate: 60 },
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  };

  const { chapterId } = await params;

  let chapter: ChapterDetail | null = null;

  try {
    const response = await fetchData(
      `/api/komik2/chapter?chapter_url=${chapterId}`,
    );
    chapter = response.data;
  } catch (_e) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center">
        <div className="max-w-md text-center">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex flex-col items-center gap-4">
            <AlertTriangle className="w-12 h-12 text-red-600 dark:text-red-400" />
            <h1 className="text-2xl font-bold text-red-800 dark:text-red-200">
              Gagal Memuat Chapter
            </h1>
            <p className="text-red-700 dark:text-red-300">
              Silakan coba kembali beberapa saat lagi
            </p>
          </div>
        </div>
      </main>
    );
  }

  if (!chapter) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center">
        <div className="max-w-md text-center">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex flex-col items-center gap-4">
            <AlertTriangle className="w-12 h-12 text-red-600 dark:text-red-400" />
            <h1 className="text-2xl font-bold text-red-800 dark:text-red-200">
              Gagal Memuat Chapter
            </h1>
            <p className="text-red-700 dark:text-red-300">
              Chapter tidak ditemukan
            </p>
          </div>
        </div>
      </main>
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
