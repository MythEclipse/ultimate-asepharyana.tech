import { APIURLSERVER } from '../lib/url';

export const fetchKomikData = async (
  url: string,
  revalidate = 60,
  timeout = 10000
) => {
  const fullUrl = url.startsWith('/') ? `${APIURLSERVER}${url}` : url;
  const response = await fetch(fullUrl, {
    headers: {
      'Content-Type': 'application/json',
    },
    next: { revalidate: revalidate },
    signal: AbortSignal.timeout(timeout),
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return await response.json();
};
