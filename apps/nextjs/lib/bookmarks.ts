// lib/bookmarks.ts - Centralized bookmark management

/**
 * Generic bookmark type - can be extended for specific use cases
 */
export interface Bookmark {
  id?: string;
  slug: string;
  title: string;
  poster: string;
  [key: string]: unknown; // Allow additional properties
}

/**
 * Anime-specific bookmark interface
 */
export interface AnimeBookmark extends Bookmark {
  slug: string;
  title: string;
  poster: string;
}

/**
 * Komik-specific bookmark interface
 */
export interface KomikBookmark extends Bookmark {
  title: string;
  poster: string;
  chapter: string;
  score: string;
  slug: string;
  date: string;
  type: string;
  komik_id: string;
}

/**
 * Get bookmarks from localStorage
 * @param type - Type of bookmarks ('anime' or 'komik')
 * @returns Array of bookmarks
 */
export function getBookmarks<T = Bookmark>(type: 'anime' | 'komik'): T[] {
  if (typeof window === 'undefined') return [];
  try {
    const storedBookmarks = localStorage.getItem(`bookmarks-${type}`);
    return storedBookmarks ? JSON.parse(storedBookmarks) : [];
  } catch (error) {
    console.error(`Failed to get bookmarks for ${type}:`, error);
    return [];
  }
}

/**
 * Save bookmarks to localStorage
 * @param type - Type of bookmarks ('anime' or 'komik')
 * @param bookmarks - Array of bookmarks to save
 */
export function saveBookmarks<T = Bookmark>(
  type: 'anime' | 'komik',
  bookmarks: T[],
): void {
  if (typeof window === 'undefined') return;
  try {
    localStorage.setItem(`bookmarks-${type}`, JSON.stringify(bookmarks));
  } catch (error) {
    console.error(`Failed to save bookmarks for ${type}:`, error);
  }
}

/**
 * Add a bookmark
 * @param type - Type of bookmark ('anime' or 'komik')
 * @param bookmark - Bookmark to add
 */
export function addBookmark<T = Bookmark>(
  type: 'anime' | 'komik',
  bookmark: T,
): void {
  const bookmarks = getBookmarks<T>(type);
  bookmarks.push(bookmark);
  saveBookmarks(type, bookmarks);
}

/**
 * Remove a bookmark by slug
 * @param type - Type of bookmark ('anime' or 'komik')
 * @param slug - Slug of the bookmark to remove
 */
export function removeBookmark(type: 'anime' | 'komik', slug: string): void {
  const bookmarks = getBookmarks(type);
  const filtered = bookmarks.filter((b) => b.slug !== slug);
  saveBookmarks(type, filtered);
}

/**
 * Check if a bookmark exists
 * @param type - Type of bookmark ('anime' or 'komik')
 * @param slug - Slug to check
 * @returns True if bookmark exists
 */
export function isBookmarked(type: 'anime' | 'komik', slug: string): boolean {
  const bookmarks = getBookmarks(type);
  return bookmarks.some((b) => b.slug === slug);
}

/**
 * Toggle bookmark (add if not exists, remove if exists)
 * @param type - Type of bookmark ('anime' or 'komik')
 * @param slug - Slug to toggle
 * @param bookmark - Bookmark data to add if not exists
 * @returns True if bookmark was added, false if removed
 */
export function toggleBookmark<T extends Bookmark>(
  type: 'anime' | 'komik',
  slug: string,
  bookmark: T,
): boolean {
  if (isBookmarked(type, slug)) {
    removeBookmark(type, slug);
    return false;
  } else {
    addBookmark(type, bookmark);
    return true;
  }
}
