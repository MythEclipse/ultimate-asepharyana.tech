/**
 * Centralized URL types for the application
 */

// Base URL configuration interfaces
export interface BaseUrlConfig {
  production: string;
  development: string;
  current: string;
}

export interface ApiUrlConfig {
  client: string;
  server: string;
  current: string;
}

// Domain-specific URL configurations
export interface AnimeUrlConfig {
  base: string;
  search: string;
  detail: string;
  complete: string;
  ongoing: string;
}

export interface KomikUrlConfig {
  base: string;
  search: string;
  detail: string;
  chapter: string;
  manga: string;
}

export interface SocialMediaUrlConfig {
  discord: string;
  facebook: string;
  instagram: string;
  linkedin: string;
}

export interface ImageProxyConfig {
  base: string;
  cdn1: string;
  cdn2: string;
  upload: string;
}

// Query parameter types
export interface QueryParams {
  [key: string]: string | number | boolean | undefined | null;
}

export interface SearchQueryParams extends QueryParams {
  q: string;
  page?: number;
  limit?: number;
}

// URL building options
export interface UrlBuildOptions {
  baseUrl?: string;
  path: string;
  queryParams?: QueryParams;
  trailingSlash?: boolean;
}

export interface UrlValidationOptions {
  requireProtocol?: boolean;
  allowedProtocols?: string[];
  validateDomain?: boolean;
}

// Domain-specific URL types
export type AnimeEndpoint = 'detail' | 'complete-anime' | 'ongoing-anime' | 'search';
export type KomikEndpoint = 'detail' | 'chapter' | 'search';
export type ApiVersion = 'v1' | 'v2';

// Komik-specific types
export interface KomikBaseUrlOptions {
  forceRefresh?: boolean;
  maxWaitTime?: number;
  cacheTtl?: number;
}

export interface KomikUrlBuilderOptions {
  slug: string;
  endpoint: KomikEndpoint;
  queryParams?: SearchQueryParams;
}

// Environment configuration
export interface UrlEnvironmentConfig {
  isDevelopment: boolean;
  isProduction: boolean;
  currentEnvironment: 'development' | 'production' | 'test';
}
