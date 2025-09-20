'use client';

import { APIURL, BaseUrl } from '../lib/url';

export const fetchData = async (
  url: string,
  method = 'GET',
  pyld?: Record<string, unknown>,
  formDataObj?: FormData,
  baseUrl?: string,
) => {
  const base = baseUrl || APIURL;
  const fullUrl = url.startsWith('/')
    ? `${base}${url.endsWith('/') ? url.slice(0, -1) : url}`
    : url;
  try {
    const options: RequestInit = {
      method,
      headers: {},
    };

    if (method === 'POST') {
      if (formDataObj) {
        options.body = formDataObj;
        // Content-Type header is automatically set for FormData by fetch
      } else if (pyld) {
        options.body = JSON.stringify(pyld);
        (options.headers as Record<string, string>)['Content-Type'] =
          'application/json';
      }
    }

    const response = await fetch(fullUrl, options);

    const data = await response.json();
    return {
      data,
      status: response.status,
    };
  } catch (error: unknown) {
    throw new Error(String(error));
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

  const fetchPromises = endpoints.map(({ url, baseUrl }, index) =>
    fetchData(url, method, pyld, formDataObj, baseUrl).then((result) => {
      console.log(
        `Fetch successful from endpoint ${index + 1}: ${url} (baseUrl: ${baseUrl || 'default'})`,
      );
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
    console.error(`All fetch attempts failed: ${errorMessages}`);
    throw new Error(`All fetch attempts failed: ${errorMessages}`);
  }
};
