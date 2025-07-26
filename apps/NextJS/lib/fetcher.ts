const fetcher = (url: string) => {
  const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
  return fetch(url, {
    headers: token ? { Authorization: `Bearer ${token}` } : {},
  }).then((res) => res.json());
};
export default fetcher;
