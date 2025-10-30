'use client';

import { useState, useEffect, useCallback } from 'react';
import { isBookmarked, toggleBookmark, type Bookmark } from '../../lib/bookmarks';

/**
 * Custom hook for managing bookmark state
 * Provides reactive bookmark state and toggle functionality
 *
 * @param type - Type of bookmark ('anime' or 'komik')
 * @param slug - Unique identifier for the item
 * @param bookmarkData - Data to save when bookmarking (optional, can be provided on toggle)
 * @returns Object with bookmark state and toggle function
 */
export function useBookmark<T extends Bookmark>(
  type: 'anime' | 'komik',
  slug: string,
  bookmarkData?: T
) {
  const [isMarked, setIsMarked] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  // Check bookmark status on mount and when slug changes
  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsLoading(true);
      const marked = isBookmarked(type, slug);
      setIsMarked(marked);
      setIsLoading(false);
    }
  }, [type, slug]);

  /**
   * Toggle bookmark status
   * @param data - Optional bookmark data to use instead of the one provided in hook initialization
   */
  const toggle = useCallback(
    (data?: T) => {
      const dataToUse = data || bookmarkData;
      if (!dataToUse) {
        console.warn('useBookmark: No bookmark data provided');
        return;
      }

      const wasAdded = toggleBookmark(type, slug, dataToUse);
      setIsMarked(wasAdded);
    },
    [type, slug, bookmarkData]
  );

  return {
    /** Whether the item is currently bookmarked */
    isBookmarked: isMarked,
    /** Whether the bookmark status is being loaded */
    isLoading,
    /** Toggle the bookmark status */
    toggle,
  };
}
