// Re-export all functions from the new proxy client for backward compatibility
export {
  fetchWithProxy,
  fetchWithProxyOnly,
  CroxyProxyOnly,
  ProxyHttpClient,
} from '../utils/proxy-client';

export type {
  CustomError,
} from '../utils/proxy-client';

// Import types for backward compatibility
export type {
  FetchResult,
} from '../types/http';
