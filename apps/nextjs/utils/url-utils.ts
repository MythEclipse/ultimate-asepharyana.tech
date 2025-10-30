/**
 * Centralized URL utilities for the application
 * Provides consistent URL building, validation, and management across the codebase
 */

import {
  BaseUrlConfig,
  ApiUrlConfig,
  AnimeUrlConfig,
  KomikUrlConfig,
  SocialMediaUrlConfig,
  ImageProxyConfig,
  SearchQueryParams,
  UrlBuildOptions,
  UrlValidationOptions,
  AnimeEndpoint,
  KomikEndpoint,
  UrlEnvironmentConfig,
} from '../types/url';

// =============================================================================
// CENTRALIZED CONSTANTS
// =============================================================================

// Environment-based URL configurations
export const URL_CONFIG = {
  production: {
    base: process.env.NEXT_PUBLIC_PRODUCTION_URL || 'https://asepharyana.tech',
    api: {
      client: process.env.NEXT_PUBLIC_API_URL || 'https://ws.asepharyana.tech',
      server: process.env.API_URL_SERVER || 'http://localhost:4091',
    },
  },
  development: {
    base: process.env.NEXT_PUBLIC_PRODUCTION_URL || 'https://asepharyana.tech',
    api: {
      client: process.env.NEXT_PUBLIC_API_URL || 'https://ws.asepharyana.tech',
      server: process.env.API_URL_SERVER || 'http://localhost:4091',
    },
  },
} as const;

// Domain-specific base URLs
export const KOMIK_BASE_URL = process.env.NEXT_PUBLIC_KOMIK;

// Social media URLs
export const SOCIAL_MEDIA_URLS = {
  discord: 'https://discord.gg/asepharyana',
  facebook: 'https://facebook.com/asepharyana',
  instagram: 'https://instagram.com/asepharyana',
  linkedin: 'https://linkedin.com/in/asepharyana',
} as const;

// Image proxy URLs
export const IMAGE_PROXY_URLS = {
  base: '/api/imageproxy',
  cdn1: 'https://imagecdn.app/v1/images',
  cdn2: 'https://imagecdn.app/v2/images',
  upload: 'https://tmpfiles.org/api/v1/upload',
} as const;

// =============================================================================
// ENVIRONMENT UTILITIES
// =============================================================================

/**
 * Get current environment configuration
 */
export const getEnvironmentConfig = (): UrlEnvironmentConfig => {
  const isDevelopment = process.env.NODE_ENV === 'development';
  const isProduction = process.env.NODE_ENV === 'production';

  return {
    isDevelopment,
    isProduction,
    currentEnvironment: isDevelopment
      ? 'development'
      : isProduction
        ? 'production'
        : 'test',
  };
};

/**
 * Get base URL configuration based on current environment
 */
export const getBaseUrlConfig = (): BaseUrlConfig => {
  const env = getEnvironmentConfig();
  const config = env.isDevelopment
    ? URL_CONFIG.development
    : URL_CONFIG.production;

  return {
    production: URL_CONFIG.production.base,
    development: URL_CONFIG.development.base,
    current: config.base,
  };
};

/**
 * Get API URL configuration based on current environment
 */
export const getApiUrlConfig = (): ApiUrlConfig => {
  const env = getEnvironmentConfig();
  const config = env.isDevelopment
    ? URL_CONFIG.development
    : URL_CONFIG.production;

  return {
    client: config.api.client,
    server: config.api.server,
    current: config.api.client,
  };
};

// =============================================================================
// GENERIC URL BUILDING UTILITIES
// =============================================================================

/**
 * Build a complete URL from base and path
 */
export const buildUrl = (base: string, path: string): string => {
  if (path.startsWith('http://') || path.startsWith('https://')) {
    return path;
  }

  const cleanBase = base.replace(/\/$/, '');
  const cleanPath = path.startsWith('/')
    ? path.replace(/\/$/, '')
    : `/${path.replace(/\/$/, '')}`;

  return `${cleanBase}${cleanPath}`;
};

/**
 * Build URL with query parameters
 */
export const buildUrlWithParams = (options: UrlBuildOptions): string => {
  const { baseUrl, path, queryParams, trailingSlash = false } = options;
  const base = baseUrl || getBaseUrlConfig().current;

  let url = buildUrl(base, path);

  if (queryParams && Object.keys(queryParams).length > 0) {
    const params = new URLSearchParams();

    Object.entries(queryParams).forEach(([key, value]) => {
      if (value !== undefined && value !== null) {
        params.append(key, String(value));
      }
    });

    const queryString = params.toString();
    if (queryString) {
      url += `?${queryString}`;
    }
  }

  if (trailingSlash && !url.endsWith('/')) {
    url += '/';
  }

  return url;
};

/**
 * Build search URL with query parameters
 */
export const buildSearchUrl = (
  baseUrl: string,
  query: string,
  additionalParams?: SearchQueryParams,
): string => {
  const params: SearchQueryParams = { q: query, ...additionalParams };
  return buildUrlWithParams({
    baseUrl,
    path: '/search',
    queryParams: params,
  });
};

// =============================================================================
// URL VALIDATION UTILITIES
// =============================================================================

/**
 * Validate URL format
 */
export const isValidUrl = (
  url: string,
  options: UrlValidationOptions = {},
): boolean => {
  const {
    requireProtocol = true,
    allowedProtocols = ['http:', 'https:'],
    validateDomain = false,
  } = options;

  try {
    const urlObj = new URL(url);

    if (requireProtocol && !allowedProtocols.includes(urlObj.protocol)) {
      return false;
    }

    if (validateDomain) {
      // Basic domain validation - can be extended
      const domainPattern =
        /^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]?\.[a-zA-Z]{2,}$/;
      if (!domainPattern.test(urlObj.hostname)) {
        return false;
      }
    }

    return true;
  } catch {
    return false;
  }
};

/**
 * Sanitize URL by removing dangerous characters
 */
export const sanitizeUrl = (url: string): string => {
  // Remove potential XSS vectors
  return url
    .replace(/javascript:/gi, '')
    .replace(/data:/gi, '')
    .replace(/vbscript:/gi, '')
    .replace(/onload=/gi, '')
    .replace(/onerror=/gi, '')
    .trim();
};

// =============================================================================
// DOMAIN-SPECIFIC URL BUILDERS
// =============================================================================

/**
 * Get anime URL configuration
 */
export const getAnimeUrlConfig = (): AnimeUrlConfig => {
  const baseUrl = getApiUrlConfig().server;

  return {
    base: `${baseUrl}/api/anime`,
    search: `${baseUrl}/api/anime/search`,
    detail: `${baseUrl}/api/anime/detail`,
    complete: `${baseUrl}/api/anime/complete-anime`,
    ongoing: `${baseUrl}/api/anime/ongoing-anime`,
  };
};

/**
 * Get komik URL configuration
 */
export const getKomikUrlConfig = (): KomikUrlConfig => {
  const baseUrl = getApiUrlConfig().server;

  return {
    base: `${baseUrl}/api/komik`,
    search: `${baseUrl}/api/komik/search`,
    detail: `${baseUrl}/api/komik/detail`,
    chapter: `${baseUrl}/api/komik/chapter`,
    manga: `${baseUrl}/api/komik2/manga`,
  };
};

/**
 * Get social media URL configuration
 */
export const getSocialMediaUrlConfig = (): SocialMediaUrlConfig => {
  return { ...SOCIAL_MEDIA_URLS };
};

/**
 * Get image proxy URL configuration
 */
export const getImageProxyUrlConfig = (): ImageProxyConfig => {
  return { ...IMAGE_PROXY_URLS };
};

// =============================================================================
// API-SPECIFIC URL BUILDERS
// =============================================================================

/**
 * Build anime API URL
 */
export const buildAnimeUrl = (
  endpoint: AnimeEndpoint,
  slug?: string,
  query?: string,
): string => {
  const config = getAnimeUrlConfig();

  switch (endpoint) {
    case 'search':
      return query ? buildSearchUrl(config.base, query) : config.search;
    case 'detail':
      return slug ? buildUrl(config.detail, slug) : config.detail;
    case 'complete-anime':
      return slug ? buildUrl(config.complete, slug) : config.complete;
    case 'ongoing-anime':
      return config.ongoing;
    default:
      return config.base;
  }
};

/**
 * Build komik API URL
 */
export const buildKomikUrl = (
  endpoint: KomikEndpoint,
  slug?: string,
  query?: string,
): string => {
  const config = getKomikUrlConfig();

  switch (endpoint) {
    case 'search':
      return query ? buildSearchUrl(config.base, query) : config.search;
    case 'detail':
      return slug ? buildUrl(config.detail, slug) : config.detail;
    case 'chapter':
      return slug ? buildUrl(config.chapter, slug) : config.chapter;
    default:
      return config.base;
  }
};

/**
 * Build komik2 manga URL
 */
export const buildKomik2MangaUrl = (slug: string): string => {
  const config = getKomikUrlConfig();
  return buildUrl(config.manga, slug);
};

// =============================================================================
// BACKWARD COMPATIBILITY EXPORTS
// =============================================================================

// Maintain backward compatibility with existing code
export const PRODUCTION = getBaseUrlConfig().current;
export const APIURLSERVER = getApiUrlConfig().server;
export const APIURLCLIENT = getApiUrlConfig().client;
export const APIURL = APIURLCLIENT;
export const BaseUrl = PRODUCTION;

// Export komik base URL (will be undefined if not set)
export const KOMIK = KOMIK_BASE_URL;

// =============================================================================
// UTILITY EXPORTS
// =============================================================================

export const UrlUtils = {
  // Environment
  getEnvironmentConfig,
  getBaseUrlConfig,
  getApiUrlConfig,

  // Generic building
  buildUrl,
  buildUrlWithParams,
  buildSearchUrl,

  // Validation
  isValidUrl,
  sanitizeUrl,

  // Domain-specific configs
  getAnimeUrlConfig,
  getKomikUrlConfig,
  getSocialMediaUrlConfig,
  getImageProxyUrlConfig,

  // API builders
  buildAnimeUrl,
  buildKomikUrl,
  buildKomik2MangaUrl,

  // Constants
  URL_CONFIG,
  SOCIAL_MEDIA_URLS,
  IMAGE_PROXY_URLS,
} as const;

// =============================================================================
// SERVER-SIDE FETCH WITH FALLBACK
// =============================================================================

/**
 * Fetch with automatic fallback from server URL to client URL
 * This ensures the app works even when the server API is unavailable
 */
export async function fetchWithFallback(
  path: string,
  options?: RequestInit & { revalidate?: number },
): Promise<Response> {
  const { revalidate, ...fetchOptions } = options || {};

  // Prepare fetch options
  const baseOptions: RequestInit = {
    ...fetchOptions,
    headers: {
      'Content-Type': 'application/json',
      ...fetchOptions?.headers,
    },
    ...(revalidate !== undefined && { next: { revalidate } }),
  };

  // Try server URL first
  try {
    const serverUrl = path.startsWith('/') ? `${APIURLSERVER}${path}` : path;
    const response = await fetch(serverUrl, baseOptions);

    if (response.ok) {
      return response;
    }

    // If server responds but with error status, try client URL
    throw new Error(`Server responded with status: ${response.status}`);
  } catch (serverError) {
    // Fallback to client URL
    try {
      const clientUrl = path.startsWith('/') ? `${APIURL}${path}` : path;
      const response = await fetch(clientUrl, baseOptions);

      if (!response.ok) {
        throw new Error(`Client responded with status: ${response.status}`);
      }

      return response;
    } catch (clientError) {
      // Both failed, throw combined error
      throw new Error(
        `Both server and client fetch failed. Server: ${serverError instanceof Error ? serverError.message : 'Unknown error'}. Client: ${clientError instanceof Error ? clientError.message : 'Unknown error'}`,
      );
    }
  }
}
