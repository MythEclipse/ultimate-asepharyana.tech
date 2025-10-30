import { fetchWithFallback } from '../../../../utils/url-utils';
import ChapterPageClient from './ChapterPageClient';
import type { ChapterDetail } from '../../../../utils/hooks/useKomik';

interface ApiResponse {
  message: string;
  data: {
    title: string;
    images: string[];
    next_chapter_id?: string | null; // API uses 'next_chapter_id'
    prev_chapter_id?: string | null; // API uses 'prev_chapter_id'
    list_chapter: string; // API provides 'list_chapter'
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
      },
    );

    const result: ApiResponse = await response.json();

    if (result.data) {
      initialData = {
        title: result.data.title,
        images: result.data.images,
        has_next: !!result.data.next_chapter_id, // Derive from next_chapter_id
        has_prev: !!result.data.prev_chapter_id, // Derive from prev_chapter_id
        next_chapter: result.data.next_chapter_id || undefined, // Map next_chapter_id to next_chapter
        prev_chapter: result.data.prev_chapter_id || undefined, // Map prev_chapter_id to prev_chapter
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
