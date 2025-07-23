'use client';
import React, { useEffect, useState } from 'react';
import AnimeGrid from '@/components/card/AnimeGrid';
import BookmarkLoadingSkeleton from '@/components/skeleton/BookmarkLoadingSkeleton';
import EmptyBookmarkMessage from '@/components/misc/EmptyBookmarkMessage';
import { Bookmark, ChevronLeft, ChevronRight } from 'lucide-react';

const ITEMS_PER_PAGE = 24;

interface BookmarkItem {
  slug: string;
  title: string;
  poster: string;
}

interface Pagination {
  current_page: number;
  last_visible_page: number;
  has_next_page: boolean;
  has_previous_page: boolean;
}

export default function BookmarkPage() {
  const [bookmarks, setBookmarks] = useState<BookmarkItem[]>([]);
  const [pagination, setPagination] = useState<Pagination>({
    current_page: 1,
    last_visible_page: 1,
    has_next_page: false,
    has_previous_page: false,
  });
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const storedBookmarks = JSON.parse(
      localStorage.getItem('bookmarks-anime') || '[]'
    );
    setBookmarks(storedBookmarks);
    setIsLoading(false);
  }, []);

  useEffect(() => {
    const totalPages = Math.ceil(bookmarks.length / ITEMS_PER_PAGE);
    const currentPage = Math.min(pagination.current_page, totalPages || 1);

    setPagination({
      current_page: currentPage,
      last_visible_page: totalPages,
      has_next_page: currentPage < totalPages,
      has_previous_page: currentPage > 1,
    });
  }, [bookmarks, pagination.current_page]);

  const handlePageChange = (newPage: number) => {
    setPagination((prev) => ({
      ...prev,
      current_page: Math.max(1, Math.min(newPage, prev.last_visible_page)),
    }));
  };

  const getPaginatedBookmarks = () => {
    const start = (pagination.current_page - 1) * ITEMS_PER_PAGE;
    const end = start + ITEMS_PER_PAGE;
    return bookmarks.slice(start, end);
  };

  if (isLoading) {
    return <BookmarkLoadingSkeleton />;
  }

  return (
    <main className='min-h-screen p-6 bg-background dark:bg-dark'>
      <div className='max-w-7xl mx-auto space-y-8'>
        <div className='flex items-center gap-4'>
          <div className='p-3 bg-purple-100 dark:bg-purple-900/50 rounded-xl'>
            <Bookmark className='w-8 h-8 text-purple-600 dark:text-purple-400' />
          </div>
          <h1 className='text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent'>
            My Bookmarks ({bookmarks.length})
          </h1>
        </div>

        {bookmarks.length === 0 ? (
          <EmptyBookmarkMessage />
        ) : (
          <>
            <AnimeGrid animes={getPaginatedBookmarks()} />

            <div className='flex flex-wrap gap-4 justify-between items-center mt-8'>
              <div className='flex gap-4'>
                {pagination.has_previous_page && (
                  <button
                    onClick={() =>
                      handlePageChange(pagination.current_page - 1)
                    }
                    className='px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2'
                  >
                    <ChevronLeft className='w-5 h-5' />
                    Previous
                  </button>
                )}

                {pagination.has_next_page && (
                  <button
                    onClick={() =>
                      handlePageChange(pagination.current_page + 1)
                    }
                    className='px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2'
                  >
                    Next
                    <ChevronRight className='w-5 h-5' />
                  </button>
                )}
              </div>

              <span className='text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4'>
                Page {pagination.current_page} of {pagination.last_visible_page}
              </span>
            </div>
          </>
        )}
      </div>
    </main>
  );
}
