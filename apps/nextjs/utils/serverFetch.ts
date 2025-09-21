import { UnifiedHttpClient } from './http-client';

export const serverFetch = async (url: string) => {
  const client = UnifiedHttpClient.createServerSide();
  return client.fetchJson(url);
};
