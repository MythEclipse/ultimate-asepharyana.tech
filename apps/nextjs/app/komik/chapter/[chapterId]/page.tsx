'use client';

import React, { useEffect, useState, useRef } from 'react';
import Link from 'next/link';
import {
  ChevronLeft,
  ChevronRight,
  BookOpen,
  AlertTriangle,
} from 'lucide-react';
import { ImageWithFallback } from '../../../../components/shared/ImageWithFallback';
import { Skeleton } from '../../../../components/ui/skeleton';
import { useParams } from 'next/navigation';
import { PRODUCTION } from '../../../../lib/url';

// Define the WebSocket event types
interface ChapterDataEvent {
  Chapter: {
    chapter: string;
    date: string;
    chapter_id: string;
  };
}

interface DetailDataEvent {
  Detail: {
    title: string;
    poster: string;
    description: string;
    status: string;
    type: string;
    release_date: string;
    author: string;
    total_chapter: string;
    updated_on: string;
    genres: string[];
    chapters: Array<{
      chapter: string;
      date: string;
      chapter_id: string;
    }>;
    // Assuming komikId is passed as part of the Detail event or derived otherwise
    // It's more reliable to use the komikId from the URL params directly for list_chapter
  };
}

interface ErrorEvent {
  Error: string;
}

interface ChapterImageEvent {
  ChapterImage: string;
}

interface ChapterImagesEndEvent {
  ChapterImagesEnd: null;
}

interface EndOfStreamEvent {
  EndOfStream: null;
}

type KomikDetailEvent =
  | ChapterDataEvent
  | DetailDataEvent
  | ErrorEvent
  | ChapterImageEvent
  | ChapterImagesEndEvent
  | EndOfStreamEvent;

interface ChapterDetailState {
  title: string;
  prev_chapter_id: string;
  next_chapter_id: string;
  list_chapter: string;
  images: string[];
}

export default function ChapterPage() {
  const params = useParams();
  const chapterId = params?.chapterId as string;
  const komikId = (params?.komikId as string) || ''; // Assuming komikId can be derived or passed

  const [chapterState, setChapterState] = useState<ChapterDetailState>({
    title: '',
    prev_chapter_id: '',
    next_chapter_id: '',
    list_chapter: '',
    images: [],
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (!chapterId) {
      setError('Chapter ID is missing.');
      setIsLoading(false);
      return;
    }

    const wsUrl = `${PRODUCTION.replace('http', 'ws')}/ws/komik2/detail`;
    wsRef.current = new WebSocket(wsUrl);

    wsRef.current.onopen = () => {
      console.log('WebSocket connection opened.');
      // Request chapter images
      const request = {
        komik_id: komikId, // Pass komikId if available, otherwise an empty string
        chapter_id: chapterId,
      };
      wsRef.current?.send(JSON.stringify(request));
    };

    wsRef.current.onmessage = (event) => {
      const parsedMessage: KomikDetailEvent = JSON.parse(event.data);

      if ('ChapterImage' in parsedMessage) {
        setChapterState((prevState) => ({
          ...prevState,
          images: [...prevState.images, parsedMessage.ChapterImage],
        }));
      } else if ('Detail' in parsedMessage) {
        // This might not be strictly needed if only images are streamed for chapters
        // But keeping it for completeness if detail info is sent first
        setChapterState((prevState) => ({
          ...prevState,
          title: parsedMessage.Detail.title,
          list_chapter: komikId, // Use komikId from URL params
          // prev_chapter_id and next_chapter_id would need to be fetched separately or sent via WS
        }));
      } else if ('Error' in parsedMessage) {
        setError(parsedMessage.Error);
        setIsLoading(false);
      } else if ('ChapterImagesEnd' in parsedMessage) {
        setIsLoading(false);
        console.log('Chapter images stream ended.');
      } else if ('EndOfStream' in parsedMessage) {
        // This might be for the overall detail stream, not just images
        console.log('Full stream ended.');
      }
    };

    wsRef.current.onclose = () => {
      console.log('WebSocket connection closed.');
      setIsLoading(false); // Ensure loading state is false on close
    };

    wsRef.current.onerror = (err) => {
      console.error('WebSocket error:', err);
      setError('WebSocket connection error.');
      setIsLoading(false);
    };

    return () => {
      wsRef.current?.close();
    };
  }, [chapterId, komikId]);

  if (error) {
    return (
      <main className="min-h-screen p-6 bg-background dark:bg-dark flex items-center justify-center">
        <div className="max-w-md text-center">
          <div className="p-6 bg-red-100 dark:bg-red-900/30 rounded-2xl flex flex-col items-center gap-4">
            <AlertTriangle className="w-12 h-12 text-red-600 dark:text-red-400" />
            <h1 className="text-2xl font-bold text-red-800 dark:text-red-200">
              Gagal Memuat Chapter
            </h1>
            <p className="text-red-700 dark:text-red-300">
              {error}. Silakan coba kembali nanti.
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
          {isLoading && chapterState.title === '' ? (
            <Skeleton className="h-8 w-64 mx-auto" />
          ) : (
            chapterState.title
          )}
        </h1>

        <div className="p-6">
          <div className="flex flex-col md:flex-row gap-4 items-stretch">
            {/* Previous Button */}
            <div className="flex-1 flex justify-center md:justify-start">
              {isLoading && chapterState.prev_chapter_id === '' ? (
                <Skeleton className="w-full md:w-auto h-10 px-6 py-2 rounded-lg" />
              ) : chapterState.prev_chapter_id ? (
                <Link
                  scroll
                  href={`/komik/chapter/${chapterState.prev_chapter_id}`}
                  className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors"
                >
                  <ChevronLeft className="w-5 h-5" />
                  <span className="hidden sm:inline">Chapter Sebelumnya</span>
                  <span className="sm:hidden">Sebelumnya</span>
                </Link>
              ) : (
                <div className="w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center">
                  <ChevronLeft className="w-5 h-5" />
                </div>
              )}
            </div>

            {/* Center Button */}
            <div className="flex-1 flex justify-center">
              {isLoading && chapterState.list_chapter === '' ? (
                <Skeleton className="w-full md:w-auto h-10 px-6 py-2 rounded-lg" />
              ) : (
                <Link
                  href={`/komik/detail/${chapterState.list_chapter}`}
                  className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
                >
                  <BookOpen className="w-5 h-5" />
                  <span className="hidden sm:inline">Daftar Chapter</span>
                  <span className="sm:hidden">Daftar</span>
                </Link>
              )}
            </div>

            {/* Next Button */}
            <div className="flex-1 flex justify-center md:justify-end">
              {isLoading && chapterState.next_chapter_id === '' ? (
                <Skeleton className="w-full md:w-auto h-10 px-6 py-2 rounded-lg" />
              ) : chapterState.next_chapter_id ? (
                <Link
                  scroll
                  href={`/komik/chapter/${chapterState.next_chapter_id}`}
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
      </div>

      {/* Image Content */}
      <div className="max-w-4xl mx-auto space-y-4">
        {isLoading && chapterState.images.length === 0
          ? Array.from({ length: 5 }).map((_, i) => (
              <div key={i} className="relative group w-full">
                <Skeleton className="w-full h-auto aspect-[700/1000] rounded-xl border border-zinc-200 dark:border-zinc-700" />
                <Skeleton className="absolute bottom-2 right-2 h-6 w-20 rounded-md" />
              </div>
            ))
          : chapterState.images.map((image, index) => (
              <div key={`${image}-${index}`} className="relative group">
                <ImageWithFallback imageUrl={image} index={index} />
                <div className="absolute bottom-2 right-2 bg-black/50 text-white px-3 py-1 rounded-md text-sm opacity-0 group-hover:opacity-100 transition-opacity">
                  Halaman {index + 1}
                </div>
              </div>
            ))}
      </div>

      {/* Navigation Bottom (Fixed) */}
      <div className="p-6">
        <div className="flex flex-col md:flex-row gap-4 items-stretch">
          {/* Previous Button */}
          <div className="flex-1 flex justify-center md:justify-start">
            {isLoading && chapterState.prev_chapter_id === '' ? (
              <Skeleton className="w-full md:w-auto h-10 px-6 py-2 rounded-lg" />
            ) : chapterState.prev_chapter_id ? (
              <Link
                scroll
                href={`/komik/chapter/${chapterState.prev_chapter_id}`}
                className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-purple-100 dark:bg-purple-900/50 text-purple-600 dark:text-purple-400 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors"
              >
                <ChevronLeft className="w-5 h-5" />
                <span className="hidden sm:inline">Chapter Sebelumnya</span>
                <span className="sm:hidden">Sebelumnya</span>
              </Link>
            ) : (
              <div className="w-full md:w-auto px-6 py-2 opacity-50 cursor-not-allowed flex items-center justify-center">
                <ChevronLeft className="w-5 h-5" />
              </div>
            )}
          </div>

          {/* Center Button */}
          <div className="flex-1 flex justify-center">
            {isLoading && chapterState.list_chapter === '' ? (
              <Skeleton className="w-full md:w-auto h-10 px-6 py-2 rounded-lg" />
            ) : (
              <Link
                href={`/komik/detail/${chapterState.list_chapter}`}
                className="w-full md:w-auto flex items-center justify-center gap-2 px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
              >
                <BookOpen className="w-5 h-5" />
                <span className="hidden sm:inline">Daftar Chapter</span>
                <span className="sm:hidden">Daftar</span>
              </Link>
            )}
          </div>

          {/* Next Button */}
          <div className="flex-1 flex justify-center md:justify-end">
            {isLoading && chapterState.next_chapter_id === '' ? (
              <Skeleton className="w-full md:w-auto h-10 px-6 py-2 rounded-lg" />
            ) : chapterState.next_chapter_id ? (
              <Link
                scroll
                href={`/komik/chapter/${chapterState.next_chapter_id}`}
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
    </main>
  );
}
