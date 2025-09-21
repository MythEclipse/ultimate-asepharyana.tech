import { clientSideFetch } from './http-client';
import { APIURL } from '../lib/url';

const fetcher = async (url: string) => {
  const token =
    typeof window !== 'undefined' ? localStorage.getItem('token') : undefined;
  return clientSideFetch(url, token);
};

export default fetcher;
