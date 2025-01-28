import { fetchWithProxy } from './fetchWithProxy';
const fetcher = (url: string) => fetchWithProxy(url).then((res) => res.data);
export default fetcher;
