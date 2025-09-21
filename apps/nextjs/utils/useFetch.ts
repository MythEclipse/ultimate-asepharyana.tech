'use client';

import { UnifiedHttpClient } from './http-client';
import { getApiUrlConfig, buildUrl } from './url-utils';
import { toAppError } from './error-handler';
import { HttpMethod } from '../types/http';

export const fetchData = async <T = unknown>(
  url: string,
  method = 'GET',
  pyld?: Record<string, unknown>,
  formDataObj?: FormData,
  baseUrl?: string,
) => {
  const base = baseUrl || getApiUrlConfig().client;
  const fullUrl = buildUrl(base, url);
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

    const data = await client.request<T>(fullUrl, method as HttpMethod, pyld);
    return {
      data,
      status: 200, // HttpClient handles status internally
    };
  } catch (error: unknown) {
    const appError = toAppError(error, { url: fullUrl, method });
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

  // Use AbortController to cancel remaining requests after first success
  const controllers = endpoints.map(() => new AbortController());
  const fetchPromises = endpoints.map(({ url, baseUrl }, index) =>
    fetchData(url, method, pyld, formDataObj, baseUrl).then((result) => {
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
      url: endpoints.map(e => e.url).join(', '),
      method,
      context: { errorMessages }
    });
    throw appError;
  }
};
