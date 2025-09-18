import { APIURLSERVER } from '../lib/url';

export const serverFetch = async (url: string) => {
  const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;

  try {
    const response = await fetch(fullUrl, {
      headers: {
        'Content-Type': 'application/json',
      },
      // Add timeout for server-side requests
      signal: AbortSignal.timeout(10000), // 10 second timeout
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    console.error(`Server fetch error for ${url}:`, error);
    throw error;
  }
};
