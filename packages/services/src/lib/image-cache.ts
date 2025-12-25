/**
 * Image Cache Service
 *
 * Caches images to jsDelivr CDN via picser.pages.dev
 * Uses database storage for URL mappings.
 */

import { eq } from 'drizzle-orm';
import { getDb } from './database';
import { imageCache } from './schema';

import pLimit from 'p-limit';

// Use /api/upload endpoint (no token required, same as Rust)
const PICSER_API_URL = 'https://picser.pages.dev/api/upload';
const IMAGE_CACHE_REDIS_PREFIX = 'img_cache:';
const IMAGE_CACHE_TTL = 86400; // 24 hours

// Limit concurrency to 5 requests to Picser API
const limiter = pLimit(5);

interface PicserResponse {
  success: boolean;
  filename?: string;
  url?: string;
  urls?: {
    github?: string;
    raw?: string;
    jsdelivr?: string;
    jsdelivr_commit?: string;
  };
  size?: number;
  type?: string;
  commit_sha?: string;
  error?: string;
}

interface ImageCacheConfig {
  githubToken?: string;
  githubOwner?: string;
  githubRepo?: string;
  githubBranch?: string;
  folder?: string;
}

const defaultConfig: Required<ImageCacheConfig> = {
  githubToken: '',
  githubOwner: 'sh20raj',
  githubRepo: 'picser',
  githubBranch: 'main',
  folder: 'uploads',
};

/**
 * Create hash of URL for cache key
 */
function hashUrl(url: string): string {
  let hash = 0;
  for (let i = 0; i < url.length; i++) {
    const char = url.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }
  return Math.abs(hash).toString(16);
}

/**
 * Extract filename from URL
 */
function extractFilename(url: string): string {
  const parts = url.split('/');
  const last = parts[parts.length - 1]?.split('?')[0];
  if (last && last.includes('.')) {
    return last;
  }
  return `${hashUrl(url)}.jpg`;
}

/**
 * Get or cache an image URL
 * Returns CDN URL if cached, otherwise uploads to Picser and caches
 *
 * @param lazy - If true, returns original URL immediately and uploads in background
 */
export async function getOrCacheImage(
  originalUrl: string,
  redis?: {
    get: (key: string) => Promise<string | null>;
    set: (key: string, value: string, ttl?: number) => Promise<void>;
  },
  config?: ImageCacheConfig,
  options?: { lazy?: boolean },
): Promise<string> {
  const mergedConfig = { ...defaultConfig, ...config };
  const cacheKey = `${IMAGE_CACHE_REDIS_PREFIX}${hashUrl(originalUrl)}`;
  const isLazy = options?.lazy ?? false;

  // 1. Check Redis if available
  if (redis) {
    try {
      const cached = await redis.get(cacheKey);
      if (cached) {
        return cached;
      }
    } catch {
      // Ignore Redis errors, continue with DB
    }
  }

  // 2. Check database
  const db = getDb();
  const existing = await db
    .select()
    .from(imageCache)
    .where(eq(imageCache.originalUrl, originalUrl))
    .limit(1);

  if (existing.length > 0 && existing[0]) {
    const cdnUrl = existing[0].cdnUrl;
    // Store in Redis for faster access
    if (redis) {
      try {
        await redis.set(cacheKey, cdnUrl, IMAGE_CACHE_TTL);
      } catch {
        // Ignore Redis errors
      }
    }
    return cdnUrl;
  }

  // 3. If lazy mode, return original URL and upload in background
  if (isLazy) {
    // Fire-and-forget background upload
    setImmediate(async () => {
      try {
        const cdnUrl = await limiter(() =>
          uploadToPicser(originalUrl, mergedConfig),
        );
        // Save to database
        const id = crypto.randomUUID();
        await db.insert(imageCache).values({
          id,
          originalUrl,
          cdnUrl,
          createdAt: new Date(),
        });
        // Cache in Redis
        if (redis) {
          try {
            await redis.set(cacheKey, cdnUrl, IMAGE_CACHE_TTL);
          } catch {
            // Ignore Redis errors
          }
        }
        console.log(`[LazyCache] Successfully cached: ${originalUrl}`);
      } catch (error) {
        console.warn(
          `[LazyCache] Background upload failed for ${originalUrl}:`,
          error,
        );
      }
    });
    return originalUrl; // Return original immediately
  }

  // 4. Blocking mode: Upload to Picser (with concurrency limit)
  try {
    const cdnUrl = await limiter(() =>
      uploadToPicser(originalUrl, mergedConfig),
    );

    // 5. Save to database
    const id = crypto.randomUUID();
    await db.insert(imageCache).values({
      id,
      originalUrl,
      cdnUrl,
      createdAt: new Date(),
    });

    // 6. Cache in Redis
    if (redis) {
      try {
        await redis.set(cacheKey, cdnUrl, IMAGE_CACHE_TTL);
      } catch {
        // Ignore Redis errors
      }
    }

    return cdnUrl;
  } catch (error) {
    console.warn(`Failed to cache image ${originalUrl}:`, error);
    return originalUrl; // Graceful fallback
  }
}

/**
 * Get CDN URL without uploading (read-only lookup)
 */
export async function getCdnUrl(originalUrl: string): Promise<string | null> {
  const db = getDb();
  const existing = await db
    .select()
    .from(imageCache)
    .where(eq(imageCache.originalUrl, originalUrl))
    .limit(1);

  return existing.length > 0 && existing[0] ? existing[0].cdnUrl : null;
}

/**
 * Invalidate cached image
 */
export async function invalidateImageCache(
  originalUrl: string,
  redis?: { del: (key: string) => Promise<void> },
): Promise<void> {
  const db = getDb();
  const cacheKey = `${IMAGE_CACHE_REDIS_PREFIX}${hashUrl(originalUrl)}`;

  // Remove from Redis
  if (redis) {
    try {
      await redis.del(cacheKey);
    } catch {
      // Ignore Redis errors
    }
  }

  // Remove from database
  await db.delete(imageCache).where(eq(imageCache.originalUrl, originalUrl));
}

/**
 * Upload image to Picser CDN
 */
async function uploadToPicser(
  originalUrl: string,
  config: Required<ImageCacheConfig>,
): Promise<string> {
  // Download the image
  const imageResponse = await fetch(originalUrl, {
    headers: {
      'User-Agent':
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    },
  });
  if (!imageResponse.ok) {
    throw new Error(`Failed to download image: ${imageResponse.status}`);
  }
  const imageBlob = await imageResponse.blob();

  // Create form data - /api/upload only needs the file (no github credentials)
  const formData = new FormData();
  formData.append('file', imageBlob, extractFilename(originalUrl));

  // Upload to Picser
  const response = await fetch(PICSER_API_URL, {
    method: 'POST',
    body: formData,
  });

  const result: PicserResponse = await response.json();

  if (!result.success) {
    throw new Error(result.error || 'Picser upload failed');
  }

  // Prefer jsdelivr_commit URL for permanence
  const cdnUrl =
    result.urls?.jsdelivr_commit || result.urls?.jsdelivr || result.url;

  if (!cdnUrl) {
    throw new Error('No CDN URL in Picser response');
  }

  return cdnUrl;
}

/**
 * Batch cache multiple images
 */
export async function cacheImageUrls(
  urls: string[],
  redis?: {
    get: (key: string) => Promise<string | null>;
    set: (key: string, value: string, ttl?: number) => Promise<void>;
  },
  config?: ImageCacheConfig,
): Promise<string[]> {
  const results: string[] = [];

  for (const url of urls) {
    const cdnUrl = await getOrCacheImage(url, redis, config);
    results.push(cdnUrl);
  }

  return results;
}
