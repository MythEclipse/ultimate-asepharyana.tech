import { fetchWithFallback } from '../../../../utils/url-utils';
import KomikDetailPageClient from './KomikDetailPageClient';
import type { KomikDetail } from '../../../../utils/hooks/useKomik';

interface ApiResponse {
  status: boolean;
  data: {
    title: string;
    alternative_title?: string;
    poster: string;
    type: string;
    status: string;
    author: string;
    description: string; // API uses 'description' not 'synopsis'
    genres: string[];
    chapters: {
      chapter: string; // API uses 'chapter' not 'title'
      chapter_id: string; // API uses 'chapter_id' not 'slug'
      date: string;
    }[];
  };
}

export const revalidate = 60;

export default async function DetailMangaPage({
  params,
}: {
  params: Promise<{ komikId: string }>;
}) {
  const { komikId } = await params;

  let initialData: KomikDetail | null = null;
  let initialError: string | null = null;

  try {
    const response = await fetchWithFallback(
      `/api/komik2/detail?komik_id=${komikId}`,
      {
        revalidate,
      }
    );

    const result: ApiResponse = await response.json();

    if (result.status && result.data) {
      initialData = {
        title: result.data.title,
        alternative_title: result.data.alternative_title,
        poster: result.data.poster,
        type: result.data.type,
        status: result.data.status,
        author: result.data.author,
        synopsis: result.data.description, // Map 'description' to 'synopsis'
        genres: result.data.genres,
        chapters: result.data.chapters.map(ch => ({
          title: `Chapter ${ch.chapter}`, // Transform 'chapter' to 'title'
          slug: ch.chapter_id, // Map 'chapter_id' to 'slug'
          date: ch.date,
        })),
      };
    } else {
      initialError = 'Data tidak tersedia';
    }
  } catch (error) {
    initialError = error instanceof Error ? error.message : 'Terjadi kesalahan saat mengambil data manga';
  }

  return (
    <KomikDetailPageClient
      komikId={komikId}
      initialData={initialData}
      initialError={initialError}
    />
  );
}
