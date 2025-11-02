'use client';

import { UnifiedHttpClient } from './unified-http-client';
import { toAppError } from './error-handler';
import { HttpMethod } from '../types/http';

export const fetchData = async <T = unknown>(
  url: string,
  method = 'GET',
  pyld?: Record<string, unknown>,
  formDataObj?: FormData,
  _baseUrl?: string, // Deprecated: UnifiedHttpClient handles fallback now
) => {
  // Don't build full URL here - let UnifiedHttpClient handle fallback
  // Only use path like '/api/sosmed/posts'
  const client = UnifiedHttpClient.createClientSide();

  try {
    const options: RequestInit = { method };

    if (method === 'POST') {
      if (formDataObj) {
        options.body = formDataObj;
      } else if (pyld) {
        options.body = JSON.stringify(pyld);
        options.headers = { 'Content-Type': 'application/json' };
      }
    }

    // Pass only the path, not full URL
    // UnifiedHttpClient will handle fallback automatically
    const data = await client.request<T>(url, method as HttpMethod, pyld);
    return {
      data,
      status: 200, // HttpClient handles status internally
    };
  } catch (error: unknown) {
    const appError = toAppError(error, { url, method });
    throw appError;
  }
};

export const fetchDataMultiple = async (
  endpoints: Array<{ url: string; baseUrl?: string }>,
  method = 'GET',
  pyld?: Record<string, unknown>,
  formDataObj?: FormData,
) => {
  if (endpoints.length === 0) {
    throw new Error('No endpoints provided');
  }

  // NOTE: UnifiedHttpClient now handles automatic fallback
  // This function is kept for backward compatibility but may be redundant

  // Use AbortController to cancel remaining requests after first success
  const controllers = endpoints.map(() => new AbortController());
  const fetchPromises = endpoints.map(({ url }, index) =>
    fetchData(url, method, pyld, formDataObj).then((result) => {
      // Abort all other requests
      controllers.forEach((ctrl, i) => {
        if (i !== index) ctrl.abort();
      });
      return result;
    }),
  );

  try {
    // Use Promise.any to get the first successful response
    const result = await Promise.any(fetchPromises);
    return result;
  } catch (errors: unknown) {
    // If all fail, throw an error with details
    const errorMessages = Array.isArray(errors)
      ? errors.map((err) => String(err)).join('; ')
      : String(errors);
    const appError = toAppError(errors, {
      url: endpoints.map((e) => e.url).join(', '),
      method,
      context: { errorMessages },
    });
    throw appError;
  }
};
