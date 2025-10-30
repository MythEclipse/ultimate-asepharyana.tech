'use client';
import React from 'react';
import UnifiedGrid from '../../../../components/shared/UnifiedGrid';
import BookmarkLoadingSkeleton from '../../../../components/skeleton/BookmarkLoadingSkeleton';
import EmptyBookmarkMessage from '../../../../components/misc/EmptyBookmarkMessage';
import { Bookmark, ChevronLeft, ChevronRight } from 'lucide-react';
import { useBookmarkPagination } from '../../../../utils/hooks/useBookmarkPagination';
import type { AnimeBookmark } from '../../../../lib/bookmarks';

export default function BookmarkPage() {
  const {
    bookmarks,
    isLoading,
    pagination,
    handlePageChange,
    getPaginatedBookmarks,
  } = useBookmarkPagination<AnimeBookmark>('bookmarks-anime');

  if (isLoading) {
    return <BookmarkLoadingSkeleton />;
  }

  return (
    <main className="min-h-screen p-6 bg-background dark:bg-dark">
      <div className="max-w-7xl mx-auto space-y-8">
        <div className="flex items-center gap-4">
          <div className="p-3 bg-purple-100 dark:bg-purple-900/50 rounded-xl">
            <Bookmark className="w-8 h-8 text-purple-600 dark:text-purple-400" />
          </div>
          <h1 className="text-3xl font-bold bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
            My Bookmarks ({bookmarks.length})
          </h1>
        </div>

        {bookmarks.length === 0 ? (
          <EmptyBookmarkMessage />
        ) : (
          <>
            <UnifiedGrid items={getPaginatedBookmarks()} itemType="anime" />

            <div className="flex flex-wrap gap-4 justify-between items-center mt-8">
              <div className="flex gap-4">
                {pagination.hasPreviousPage && (
                  <button
                    onClick={() => handlePageChange(pagination.currentPage - 1)}
                    className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2"
                  >
                    <ChevronLeft className="w-5 h-5" />
                    Previous
                  </button>
                )}

                {pagination.hasNextPage && (
                  <button
                    onClick={() => handlePageChange(pagination.currentPage + 1)}
                    className="px-6 py-2 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 rounded-lg hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors flex items-center gap-2"
                  >
                    Next
                    <ChevronRight className="w-5 h-5" />
                  </button>
                )}
              </div>

              <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400 mx-4">
                Page {pagination.currentPage} of {pagination.lastVisiblePage}
              </span>
            </div>
          </>
        )}
      </div>
    </main>
  );
}
