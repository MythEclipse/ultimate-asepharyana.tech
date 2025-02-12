// src/lib/bookmarks.ts
import { Bookmark } from '@/app/(frontend)/dashboard/bookmark';

export function getBookmarks(type: 'anime' | 'komik'): Bookmark[] {
  if (typeof window === 'undefined') return [];
  const storedBookmarks = localStorage.getItem(`bookmarks-${type}`);
  return storedBookmarks ? JSON.parse(storedBookmarks) : [];
}
