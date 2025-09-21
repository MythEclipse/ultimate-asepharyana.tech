import { UnifiedHttpClient } from './http-client';

const fetcher = async (url: string) => {
  const token =
    typeof window !== 'undefined' ? localStorage.getItem('token') : undefined;
  const client = UnifiedHttpClient.createClientSide();
  return client.fetchWithAuth(url, token);
};

export default fetcher;
