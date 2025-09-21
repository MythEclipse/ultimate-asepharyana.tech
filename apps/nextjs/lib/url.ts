/**
 * URL constants and utilities
 *
 * This file now serves as a backward compatibility layer for the centralized URL utilities.
 * New code should import directly from '../utils/url-utils' instead.
 *
 * @deprecated Use '../utils/url-utils' for new code
 */

import {
  PRODUCTION,
  APIURLSERVER,
  APIURLCLIENT,
  APIURL,
  BaseUrl,
  KOMIK,
  UrlUtils,
} from '../utils/url-utils';

// Re-export all constants for backward compatibility
export { PRODUCTION, APIURLSERVER, APIURLCLIENT, APIURL, BaseUrl, KOMIK };

// Export the UrlUtils for new code that wants to use the centralized system
export { UrlUtils };

// Default export for convenience
export default {
  PRODUCTION,
  APIURLSERVER,
  APIURLCLIENT,
  APIURL,
  BaseUrl,
  KOMIK,
  UrlUtils,
};
