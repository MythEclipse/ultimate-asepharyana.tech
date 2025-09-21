import { serverSideFetch } from './http-client';
import { APIURLSERVER } from '../lib/url';

export const serverFetch = async (url: string) => {
  return serverSideFetch(url);
};
