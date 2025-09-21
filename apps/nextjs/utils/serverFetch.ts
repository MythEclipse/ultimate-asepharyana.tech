import { HttpClient } from './httpClient';
import { APIURLSERVER } from '../lib/url';

export const serverFetch = async (url: string) => {
  const fullUrl = HttpClient.buildUrl(APIURLSERVER, url);
  return HttpClient.fetchJson(fullUrl);
};
