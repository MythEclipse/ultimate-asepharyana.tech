import { fetchWithFallback } from '../../../../utils/url-utils';
import ChapterPageClient from './ChapterPageClient';
import type { ChapterDetail } from '../../../../utils/hooks/useKomik';

interface ApiResponse {
  data: {
    title: string;
    images: string[];
    has_next: boolean;
    has_prev: boolean;
    next_chapter?: string;
    prev_chapter?: string;
  };
}

export const revalidate = 60;

export default async function ChapterPage({
  params,
}: {
  params: Promise<{ chapterId: string }>;
}) {
  const { chapterId } = await params;

  let initialData: ChapterDetail | null = null;
  let initialError: string | null = null;

  try {
    const response = await fetchWithFallback(
      `/api/komik2/chapter?chapter_url=${chapterId}`,
      {
        revalidate,
      }
    );

    const result: ApiResponse = await response.json();

    if (result.data) {
      initialData = {
        title: result.data.title,
        images: result.data.images,
        has_next: result.data.has_next,
        has_prev: result.data.has_prev,
        next_chapter: result.data.next_chapter,
        prev_chapter: result.data.prev_chapter,
      };
    } else {
      initialError = 'Chapter tidak ditemukan';
    }
  } catch (error) {
    initialError =
      error instanceof Error
        ? error.message
        : 'Terjadi kesalahan saat mengambil data chapter';
  }

  return (
    <ChapterPageClient
      chapterId={chapterId}
      initialData={initialData}
      initialError={initialError}
    />
  );
}
