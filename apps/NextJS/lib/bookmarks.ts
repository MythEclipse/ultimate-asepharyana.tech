// src/lib/bookmarks.ts

// Define the Bookmark interface directly here to resolve import issues
export interface Bookmark {
  id: string;
  title: string;
  url: string;
  // Add any other properties that a bookmark object might have
}

export function getBookmarks(type: 'anime' | 'komik'): Bookmark[] {
  if (typeof window === 'undefined') return [];
  const storedBookmarks = localStorage.getItem(`bookmarks-${type}`);
  return storedBookmarks ? JSON.parse(storedBookmarks) : [];
}
