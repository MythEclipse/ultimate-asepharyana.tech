import logger from '@/lib/logger';

const fetcher = async (url: string) => {
  const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
  try {
    const res = await fetch(url, {
      headers: token ? { Authorization: `Bearer ${token}` } : {},
    });

    if (!res.ok) {
      const errorData = await res.json().catch(() => ({ message: res.statusText }));
      const error = new Error(errorData.message || 'An unknown error occurred');
      logger.error(`API Error: ${url} - Status: ${res.status}, Message: ${error.message}`);
      throw error;
    }

    return res.json();
  } catch (error) {
    logger.error(`Network or unexpected error for ${url}:`, error);
    throw error; // Re-throw to propagate the error to the calling component
  }
};
export default fetcher;
