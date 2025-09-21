// Centralized exports for all types
export * from './anime';
export * from './ClientUser';
export * from './http';

// Export image types with explicit naming to avoid conflicts
export type {
  ImageSource,
  ImageProxyConfig as ImageProxyServiceConfig,
  ImageSourceConfig,
  ImageValidationResult,
  ImageValidationOptions,
  ImageFallbackOptions,
  ImageProxyResult,
  ImageProcessingResult,
  ImageProcessingOptions,
  ImageCacheConfig,
  ImageProxyServiceConfig as ImageProxyServiceConfigType,
  ImageWithProxyProps,
} from './image';

export * from './logger';
export * from './sosmed';
// export * from './swagger-ui-dist'; // Skip as it's not a module
export * from './types';

// Export URL types with explicit naming to avoid conflicts
export type {
  BaseUrlConfig,
  ApiUrlConfig,
  AnimeUrlConfig,
  KomikUrlConfig,
  SocialMediaUrlConfig,
  ImageProxyConfig, // This is the URL config, different from image config
  QueryParams,
  SearchQueryParams,
  UrlBuildOptions,
  UrlValidationOptions,
  AnimeEndpoint,
  KomikEndpoint,
  ApiVersion,
  KomikBaseUrlOptions,
  KomikUrlBuilderOptions,
  UrlEnvironmentConfig,
} from './url';
