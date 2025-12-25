/**
 * Image Cache Utility for SolidJS
 *
 * Caches images via backend API with Rust-first, Elysia-fallback strategy.
 */

const RUST_API = 'https://ws.asepharyana.tech';
const ELYSIA_API = 'https://elysia.asepharyana.tech';

interface ImageCacheResponse {
  success: boolean;
  original_url: string;
  cdn_url: string;
  from_cache: boolean;
}

interface ImageCacheBatchResponse {
  success: boolean;
  results: Array<{
    original_url: string;
    cdn_url: string;
    from_cache: boolean;
  }>;
}

// In-memory cache for quick lookups (prevent duplicate requests)
const memoryCache = new Map<string, string>();

/**
 * Get cached image URL with Rust-first, Elysia-fallback strategy
 */
export async function getCachedImageUrl(originalUrl: string): Promise<string> {
  // Check memory cache first
  const cached = memoryCache.get(originalUrl);
  if (cached) {
    return cached;
  }

  // Try Rust API first
  try {
    const response = await fetch(`${RUST_API}/api/proxy/image-cache`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: originalUrl }),
    });

    if (response.ok) {
      const data: ImageCacheResponse = await response.json();
      if (data.success && data.cdn_url) {
        memoryCache.set(originalUrl, data.cdn_url);
        return data.cdn_url;
      }
    }
  } catch {
    // Rust API failed, try Elysia fallback
    console.warn('Rust API failed, trying Elysia fallback');
  }

  // Fallback to Elysia API
  try {
    const response = await fetch(`${ELYSIA_API}/api/image-cache`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: originalUrl }),
    });

    if (response.ok) {
      const data: ImageCacheResponse = await response.json();
      if (data.success && data.cdn_url) {
        memoryCache.set(originalUrl, data.cdn_url);
        return data.cdn_url;
      }
    }
  } catch {
    // Both APIs failed
    console.warn('All image cache APIs failed');
  }

  // Return original URL as final fallback
  return originalUrl;
}

/**
 * Batch cache multiple images
 */
export async function getCachedImageUrls(
  originalUrls: string[],
): Promise<string[]> {
  // Check memory cache for all URLs
  const results: string[] = [];
  const uncachedUrls: string[] = [];
  const uncachedIndices: number[] = [];

  for (let i = 0; i < originalUrls.length; i++) {
    const url = originalUrls[i];
    const cached = url ? memoryCache.get(url) : undefined;
    if (cached) {
      results[i] = cached;
    } else if (url) {
      uncachedUrls.push(url);
      uncachedIndices.push(i);
    } else {
      results[i] = '';
    }
  }

  // If all cached, return immediately
  if (uncachedUrls.length === 0) {
    return results;
  }

  // Try Rust API for batch
  try {
    const response = await fetch(`${RUST_API}/api/proxy/image-cache/batch`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ urls: uncachedUrls }),
    });

    if (response.ok) {
      const data: ImageCacheBatchResponse = await response.json();
      if (data.success) {
        data.results.forEach((result, idx) => {
          const originalIdx = uncachedIndices[idx];
          if (originalIdx !== undefined) {
            results[originalIdx] = result.cdn_url;
            memoryCache.set(result.original_url, result.cdn_url);
          }
        });
        return results;
      }
    }
  } catch {
    console.warn('Rust batch API failed, falling back to individual requests');
  }

  // Fallback: process remaining URLs individually
  for (let i = 0; i < uncachedIndices.length; i++) {
    const idx = uncachedIndices[i];
    const url = uncachedUrls[i];
    if (idx !== undefined && url) {
      results[idx] = await getCachedImageUrl(url);
    }
  }

  return results;
}

/**
 * Pre-cache images in background (fire and forget)
 */
export function preCacheImages(urls: string[]): void {
  // Filter out already cached
  const uncached = urls.filter((url) => !memoryCache.has(url));
  if (uncached.length > 0) {
    getCachedImageUrls(uncached).catch(() => {
      // Silently fail pre-caching
    });
  }
}

/**
 * Clear memory cache
 */
export function clearImageCache(): void {
  memoryCache.clear();
}
