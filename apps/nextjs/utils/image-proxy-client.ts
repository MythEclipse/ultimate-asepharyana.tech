/**
 * Client-side image proxy utilities (no Redis dependencies)
 * This file provides client-safe versions of image proxy functions
 */

import logger from './logger';
import {
  ImageSource,
  ImageProxyConfig,
  ImageSourceConfig,
  ImageValidationResult,
  ImageValidationOptions,
  ImageFallbackOptions,
  ImageProxyResult,
  ImageProcessingResult,
  ImageProcessingOptions,
  ImageCacheConfig,
  ImageProxyServiceConfig
} from '../types/image';
import { getImageProxyUrlConfig, sanitizeUrl, isValidUrl } from './url-utils';
import { NextResponse } from 'next/server';

// Default configuration
const DEFAULT_CONFIG: ImageProxyConfig = {
  enabled: true,
  fallbackEnabled: true,
  maxRetries: 3,
  retryDelay: 1000,
  cacheTtl: 86400, // 24 hours
};

const DEFAULT_SERVICE_CONFIG: ImageProxyServiceConfig = {
  baseUrl: process.env.NEXT_PUBLIC_BASE_URL || '',
  proxyEndpoint: '/api/imageproxy',
  cdn1Endpoint: 'https://imagecdn.app/v1/images',
  cdn2Endpoint: 'https://imagecdn.app/v2/images',
  uploadEndpoint: '/api/uploader',
  enableCache: false, // Disabled for client-side
  cachePrefix: 'image:proxy:',
};

/**
 * Image validation utility
 */
export async function validateImage(
  url: string,
  options: ImageValidationOptions = {}
): Promise<ImageValidationResult> {
  try {
    const {
      allowedTypes = ['image/jpeg', 'image/jpg', 'image/png', 'image/gif', 'image/webp'],
      requireImageContent = true
    } = options;

    // Validate URL format
    if (!isValidUrl(url)) {
      return { isValid: false, error: 'Invalid URL format' };
    }

    // Sanitize URL
    const sanitizedUrl = sanitizeUrl(url);

    // Check if URL points to an image (basic check)
    if (requireImageContent) {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 5000);

      try {
        const response = await fetch(sanitizedUrl, {
          method: 'HEAD',
          signal: controller.signal
        });

        clearTimeout(timeoutId);

        if (!response.ok) {
          return { isValid: false, error: `HTTP ${response.status}` };
        }

        const contentType = response.headers.get('content-type');
        if (!contentType || !contentType.startsWith('image/')) {
          return { isValid: false, error: 'URL does not point to an image' };
        }

        if (!allowedTypes.some(type => contentType.includes(type))) {
          return { isValid: false, error: `Unsupported image type: ${contentType}` };
        }

        return { isValid: true, contentType };
      } catch (error) {
        clearTimeout(timeoutId);
        if (error instanceof Error && error.name === 'AbortError') {
          return { isValid: false, error: 'Request timeout' };
        }
        throw error;
      }
    }

    return { isValid: true };
  } catch (error) {
    logger.error(`Image validation failed for ${url}:`, error);
    return { isValid: false, error: (error as Error).message };
  }
}

/**
 * Normalize image URL
 */
export function normalizeImageUrl(url: string, baseUrl?: string): string {
  if (!url || typeof url !== 'string') {
    return '';
  }

  // Already absolute URL
  if (url.startsWith('http://') || url.startsWith('https://')) {
    return url;
  }

  // Handle relative paths
  if (url.startsWith('/')) {
    const origin = typeof window !== 'undefined' ? window.location.origin : (baseUrl || '');
    return `${origin}${url}`;
  }

  // Handle relative paths without leading slash
  if (baseUrl) {
    return `${baseUrl.replace(/\/$/, '')}/${url}`;
  }

  // Fallback to current origin
  if (typeof window !== 'undefined') {
    return `${window.location.origin}/${url}`;
  }

  return url;
}

/**
 * Generate image proxy URL
 */
export function generateProxyUrl(url: string, config: Partial<ImageProxyServiceConfig> = {}): string {
  const serviceConfig = { ...DEFAULT_SERVICE_CONFIG, ...config };
  const normalizedUrl = normalizeImageUrl(url, serviceConfig.baseUrl);

  if (!normalizedUrl) {
    return '';
  }

  return `${serviceConfig.baseUrl}${serviceConfig.proxyEndpoint}?url=${encodeURIComponent(normalizedUrl)}`;
}

/**
 * Generate CDN image URL
 */
export function generateCdnUrl(url: string, cdnVersion: 1 | 2 = 1): string {
  const config = getImageProxyUrlConfig();
  const endpoint = cdnVersion === 1 ? config.cdn1 : config.cdn2;
  return `${endpoint}/${encodeURIComponent(url)}`;
}

/**
 * Process image from direct URL (client-side version)
 */
async function processDirectImage(
  url: string,
  options: ImageProcessingOptions = {}
): Promise<ImageProcessingResult> {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), options.timeout || 10000);

  try {
    const response = await fetch(url, {
      method: 'GET',
      signal: controller.signal,
      headers: options.headers || {},
    });

    clearTimeout(timeoutId);

    if (!response.ok) {
      return {
        success: false,
        url,
        source: 'direct',
        error: `HTTP ${response.status}`,
      };
    }

    const contentType = response.headers.get('content-type');
    if (!contentType || !contentType.startsWith('image/')) {
      return {
        success: false,
        url,
        source: 'direct',
        error: 'Not an image',
      };
    }

    const arrayBuffer = await response.arrayBuffer();

    return {
      success: true,
      url,
      source: 'direct',
      contentType,
      arrayBuffer,
    };
  } catch (error) {
    clearTimeout(timeoutId);
    logger.error(`[ImageProxy] Direct image processing failed:`, error);
    return {
      success: false,
      url,
      source: 'direct',
      error: (error as Error).message,
    };
  }
}

/**
 * Process image from CDN (client-side version)
 */
async function processCdnImage(
  url: string,
  cdnVersion: 1 | 2,
  options: ImageProcessingOptions = {}
): Promise<ImageProcessingResult> {
  const cdnUrl = generateCdnUrl(url, cdnVersion);
  const source = cdnVersion === 1 ? 'cdn1' : 'cdn2';
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), options.timeout || 10000);

  try {
    const response = await fetch(cdnUrl, {
      method: 'GET',
      signal: controller.signal,
    });

    clearTimeout(timeoutId);

    if (!response.ok) {
      return {
        success: false,
        url: cdnUrl,
        source,
        error: `HTTP ${response.status}`,
      };
    }

    const contentType = response.headers.get('content-type');
    if (!contentType || !contentType.startsWith('image/')) {
      return {
        success: false,
        url: cdnUrl,
        source,
        error: 'Not an image',
      };
    }

    const arrayBuffer = await response.arrayBuffer();

    return {
      success: true,
      url: cdnUrl,
      source,
      contentType,
      arrayBuffer,
    };
  } catch (error) {
    clearTimeout(timeoutId);
    logger.error(`[ImageProxy] CDN image processing failed:`, error);
    return {
      success: false,
      url: cdnUrl,
      source,
      error: (error as Error).message,
    };
  }
}

/**
 * Process image with fallback chain (client-side version - no Redis)
 */
export async function processImageWithFallback(
  url: string,
  options: ImageFallbackOptions = {},
  processingOptions: ImageProcessingOptions = {}
): Promise<ImageProcessingResult> {
  const {
    fallbackUrl = '/default.png',
    useProxy = true,
    useCdn = true,
    maxFallbacks = 3,
  } = options;

  if (!url || url.trim() === '') {
    return {
      success: true,
      url: fallbackUrl,
      source: 'fallback',
    };
  }

  const normalizedUrl = normalizeImageUrl(url);
  const sources: Array<{ type: ImageSource; processor: () => Promise<ImageProcessingResult> }> = [];

  // Build fallback chain
  if (useProxy) {
    sources.push({
      type: 'proxy',
      processor: () => processDirectImage(generateProxyUrl(url), processingOptions),
    });
  }

  if (useCdn) {
    sources.push(
      {
        type: 'cdn1',
        processor: () => processCdnImage(normalizedUrl, 1, processingOptions),
      },
      {
        type: 'cdn2',
        processor: () => processCdnImage(normalizedUrl, 2, processingOptions),
      }
    );
  }

  sources.push({
    type: 'direct',
    processor: () => processDirectImage(normalizedUrl, processingOptions),
  });

  // Try each source in order (no cache on client-side)
  for (const { type, processor } of sources.slice(0, maxFallbacks)) {
    const result = await processor();

    if (result.success) {
      return result;
    }

    logger.warn(`[ImageProxy] ${type} failed for ${url}: ${result.error}`);
  }

  // Final fallback
  return {
    success: true,
    url: fallbackUrl,
    source: 'fallback',
  };
}

/**
 * Generate image sources for client-side fallback
 */
export function generateImageSources(
  url: string,
  options: ImageFallbackOptions = {}
): string[] {
  const {
    fallbackUrl = '/default.png',
    useProxy = true,
    useCdn = true,
  } = options;

  if (!url || url.trim() === '') {
    return [fallbackUrl];
  }

  const normalizedUrl = normalizeImageUrl(url);
  const sources: string[] = [];

  // Direct URL first
  sources.push(normalizedUrl);

  // Proxy URL
  if (useProxy) {
    sources.push(generateProxyUrl(url));
  }

  // CDN URLs
  if (useCdn) {
    sources.push(generateCdnUrl(normalizedUrl, 1));
    sources.push(generateCdnUrl(normalizedUrl, 2));
  }

  // Final fallback
  sources.push(fallbackUrl);

  return sources;
}

/**
 * Create NextResponse for image data
 */
export function createImageResponse(
  result: ImageProcessingResult,
  cacheHeaders: Record<string, string> = {}
): NextResponse {
  if (!result.success || !result.arrayBuffer) {
    return NextResponse.json(
      { error: result.error || 'Image processing failed' },
      { status: 400 }
    );
  }

  const blob = new Blob([result.arrayBuffer], { type: result.contentType || 'image/jpeg' });

  return new NextResponse(blob, {
    headers: {
      'Content-Type': result.contentType || 'image/jpeg',
      'Cache-Control': 'public, max-age=86400, stale-while-revalidate=3600, s-maxage=0',
      ...cacheHeaders,
    },
  });
}

/**
 * Convenience function to get image with fallback (for backward compatibility)
 */
export async function getImageWithFallback(
  url: string,
  fallbackUrl?: string
): Promise<string> {
  const result = await processImageWithFallback(url, { fallbackUrl });
  return result.url;
}

// Export utilities
export const ImageProxyUtils = {
  validateImage,
  normalizeImageUrl,
  generateProxyUrl,
  generateCdnUrl,
  processImageWithFallback,
  generateImageSources,
  createImageResponse,
  getImageWithFallback,
} as const;
