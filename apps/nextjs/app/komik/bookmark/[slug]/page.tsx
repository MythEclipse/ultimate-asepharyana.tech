'use client';
import React from 'react';
import UnifiedGrid from '../../../../components/shared/UnifiedGrid';
import ButtonA from '../../../../components/ui/ScrollButton';
import { useBookmarkPagination } from '../../../../utils/hooks/useBookmarkPagination';
import type { KomikBookmark } from '../../../../lib/bookmarks';

export default function BookmarkPage() {
  const {
    bookmarks,
    isLoading,
    pagination,
    handlePageChange,
    getPaginatedBookmarks,
  } = useBookmarkPagination<KomikBookmark>('bookmarks-komik');

  if (isLoading) {
    return (
      <main className="p-6">
        <h1 className="dark:text-lighta text-2xl font-bold mt-8 mb-4">
          Bookmarked Comic ({bookmarks.length})
        </h1>
        <UnifiedGrid loading={true} items={[]} itemType="komik" />
        <div className="flex flex-wrap gap-4 justify-between items-center mt-8 animate-pulse">
          <div className="flex gap-4">
            <div className="w-24 h-10 bg-zinc-200 dark:bg-zinc-700 rounded-lg" />
            <div className="w-24 h-10 bg-zinc-200 dark:bg-zinc-700 rounded-lg" />
          </div>
          <div className="w-32 h-4 bg-zinc-200 dark:bg-zinc-700 rounded-full" />
        </div>
      </main>
    );
  }

  if (bookmarks.length === 0) {
    return (
      <main className="p-6">
        <h1 className="text-2xl font-bold mt-8 mb-4">No Bookmarked Comic</h1>
        <p>You have not bookmarked any Comic yet.</p>
      </main>
    );
  }

  return (
    <main className="p-6">
      <h1 className="dark:text-lighta text-2xl font-bold mt-8 mb-4">
        Bookmarked Comic ({bookmarks.length})
      </h1>
      <UnifiedGrid items={getPaginatedBookmarks()} itemType="komik" />
      <div className="flex justify-between mt-8">
        <div className="flex gap-4">
          {pagination.hasPreviousPage && (
            <ButtonA onClick={() => handlePageChange(pagination.currentPage - 1)}>
              Previous
            </ButtonA>
          )}

          {pagination.hasNextPage && (
            <ButtonA onClick={() => handlePageChange(pagination.currentPage + 1)}>
              Next
            </ButtonA>
          )}
        </div>

        <div className="text-gray-600 dark:text-gray-400">
          Page {pagination.currentPage} of {pagination.lastVisiblePage}
        </div>
      </div>
    </main>
  );
}
