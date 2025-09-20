'use client';
import React, { useEffect, useState } from 'react';
import UnifiedGrid from '../../../../components/shared/UnifiedGrid';
import ButtonA from '../../../../components/ui/ScrollButton';
import { usePagination } from '../../../../utils/hooks/usePagination';

interface Bookmark {
  title: string;
  poster: string;
  chapter: string;
  score: string;
  slug: string;
  date: string;
  type: string;
  komik_id: string;
}

export default function BookmarkPage() {
  const [bookmarks, setBookmarks] = useState<Bookmark[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const { pagination, handlePageChange, ITEMS_PER_PAGE } = usePagination();

  useEffect(() => {
    const storedBookmarks = JSON.parse(
      localStorage.getItem('bookmarks-komik') || '[]',
    );
    setBookmarks(storedBookmarks);
    setIsLoading(false);
  }, []);

  const totalPages = Math.ceil(bookmarks.length / ITEMS_PER_PAGE);
  const current_page = pagination.currentPage;
  const last_visible_page = totalPages;
  const has_next_page = current_page < totalPages;
  const has_previous_page = current_page > 1;

  const getPaginatedBookmarks = () => {
    const start = (current_page - 1) * ITEMS_PER_PAGE;
    const end = start + ITEMS_PER_PAGE;
    return bookmarks.slice(start, end);
  };

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
          {has_previous_page && (
            <ButtonA onClick={() => handlePageChange(current_page - 1)}>
              Previous
            </ButtonA>
          )}

          {has_next_page && (
            <ButtonA onClick={() => handlePageChange(current_page + 1)}>
              Next
            </ButtonA>
          )}
        </div>

        <div className="text-gray-600 dark:text-gray-400">
          Page {current_page} of {last_visible_page}
        </div>
      </div>
    </main>
  );
}
