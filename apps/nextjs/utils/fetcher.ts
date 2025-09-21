import { HttpClient } from './httpClient';
import { APIURL } from '../lib/url';

const fetcher = async (url: string) => {
  const token =
    typeof window !== 'undefined' ? localStorage.getItem('token') : undefined;
  const fullUrl = HttpClient.buildUrl(APIURL, url);
  return HttpClient.fetchWithAuth(fullUrl, token);
};

export default fetcher;
