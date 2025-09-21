import { APIURLSERVER } from '../lib/url';
import { HttpClient } from '../utils/httpClient';

export const fetchKomikData = async (
  url: string,
  revalidate = 60,
  timeout = 10000
) => {
  const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
  return HttpClient.fetchJson(fullUrl, {
    next: { revalidate },
    signal: AbortSignal.timeout(timeout),
  });
};
