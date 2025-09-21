// Re-export all functions from the new unified client for backward compatibility
export {
  fetchWithProxy,
  fetchWithProxyOnly,
  CroxyProxyOnly,
  ProxyHttpClient,
} from '../utils/unified-http-client';

export type {
  CustomError,
} from '../utils/unified-http-client';

// Import types for backward compatibility
export type {
  FetchResult,
} from '../types/http';
