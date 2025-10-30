'use client';

import { useState, useEffect } from 'react';
import { usePagination } from './usePagination';

/**
 * Custom hook for managing bookmark pagination
 * Handles loading bookmarks from localStorage and paginating them
 *
 * @param storageKey - The localStorage key for bookmarks (e.g., 'bookmarks-anime', 'bookmarks-komik')
 * @returns Object containing bookmarks, loading state, pagination controls, and paginated items
 */
export function useBookmarkPagination<T = unknown>(storageKey: string) {
  const [bookmarks, setBookmarks] = useState<T[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const { pagination, handlePageChange, ITEMS_PER_PAGE } = usePagination();

  // Load bookmarks from localStorage
  useEffect(() => {
    try {
      const storedBookmarks = JSON.parse(
        localStorage.getItem(storageKey) || '[]'
      );
      setBookmarks(storedBookmarks);
    } catch (error) {
      console.error(`Failed to load bookmarks from ${storageKey}:`, error);
      setBookmarks([]);
    } finally {
      setIsLoading(false);
    }
  }, [storageKey]);

  // Calculate pagination data
  const totalPages = Math.ceil(bookmarks.length / ITEMS_PER_PAGE);
  const currentPage = pagination.currentPage;
  const hasNextPage = currentPage < totalPages;
  const hasPreviousPage = currentPage > 1;

  // Get current page bookmarks
  const getPaginatedBookmarks = () => {
    const start = (currentPage - 1) * ITEMS_PER_PAGE;
    const end = start + ITEMS_PER_PAGE;
    return bookmarks.slice(start, end);
  };

  return {
    bookmarks,
    isLoading,
    pagination: {
      currentPage,
      totalPages,
      hasNextPage,
      hasPreviousPage,
      lastVisiblePage: totalPages,
    },
    handlePageChange,
    getPaginatedBookmarks,
    ITEMS_PER_PAGE,
  };
}
