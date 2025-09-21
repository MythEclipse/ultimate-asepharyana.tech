import { APIURLSERVER } from '../utils/url-utils';
import { HttpClient } from '../utils/unified-http-client';
import { getApiUrlConfig, buildUrl } from '../utils/url-utils';

export const fetchKomikData = async (
  url: string,
  revalidate = 60,
  timeout = 10000
) => {
  const apiConfig = getApiUrlConfig();
  const fullUrl = buildUrl(apiConfig.server, url);
  return HttpClient.fetchJson(fullUrl, {
    next: { revalidate },
    signal: AbortSignal.timeout(timeout),
  });
};
