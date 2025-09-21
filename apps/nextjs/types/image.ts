/**
 * Centralized image types for the application
 */

// Image source types
export type ImageSource = 'direct' | 'proxy' | 'cdn1' | 'cdn2' | 'fallback';

// Image proxy configuration interfaces
export interface ImageProxyConfig {
  enabled: boolean;
  fallbackEnabled: boolean;
  maxRetries: number;
  retryDelay: number;
  cacheTtl: number;
}

export interface ImageSourceConfig {
  direct: boolean;
  proxy: boolean;
  cdn1: boolean;
  cdn2: boolean;
  fallback: boolean;
}

// Image validation types
export interface ImageValidationResult {
  isValid: boolean;
  contentType?: string;
  error?: string;
}

export interface ImageValidationOptions {
  allowedTypes?: string[];
  maxSize?: number;
  requireImageContent?: boolean;
}

// Image fallback options
export interface ImageFallbackOptions {
  fallbackUrl?: string;
  useProxy?: boolean;
  useCdn?: boolean;
  maxFallbacks?: number;
}

// Image proxy result types
export interface ImageProxyResult {
  url: string;
  source: ImageSource;
  success: boolean;
  error?: string;
  contentType?: string;
}

export interface ImageProcessingResult {
  success: boolean;
  url: string;
  source: ImageSource;
  contentType?: string;
  error?: string;
  arrayBuffer?: ArrayBuffer;
}

// Image component props
export interface ImageWithProxyProps {
  src: string;
  alt: string;
  fallbackUrl?: string;
  useProxy?: boolean;
  useCdn?: boolean;
  className?: string;
  width?: number;
  height?: number;
  priority?: boolean;
  unoptimized?: boolean;
  onError?: () => void;
  onLoad?: () => void;
}

// Image proxy service configuration
export interface ImageProxyServiceConfig {
  baseUrl: string;
  proxyEndpoint: string;
  cdn1Endpoint: string;
  cdn2Endpoint: string;
  uploadEndpoint: string;
  enableCache: boolean;
  cachePrefix: string;
}

// Redis cache configuration for images
export interface ImageCacheConfig {
  enabled: boolean;
  ttl: number;
  prefix: string;
  fallbackTtl: number;
}

// Image processing options
export interface ImageProcessingOptions {
  validateContent?: boolean;
  useCache?: boolean;
  followRedirects?: boolean;
  timeout?: number;
  headers?: Record<string, string>;
}
