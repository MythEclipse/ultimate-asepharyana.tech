'use client';

import { APIURL } from '../lib/url';

export const fetchData = async (
  url: string,
  method = 'GET',
  pyld?: Record<string, unknown>,
  formDataObj?: FormData,
) => {
  const fullUrl = url.startsWith('/')
    ? `${APIURL}${url.endsWith('/') ? url.slice(0, -1) : url}`
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
