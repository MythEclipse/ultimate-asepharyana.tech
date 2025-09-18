export const KOMIK: string | undefined = process.env.NEXT_PUBLIC_KOMIK;
export const PRODUCTION: string =
  process.env.NEXT_PUBLIC_PROD || 'https://asepharyana.tech';
export const APIURLSERVER: string =
  process.env.NEXT_PUBLIC_API_URL_SERVER || 'http://localhost:4091';
export const APIURLCLIENT: string =
  process.env.NEXT_PUBLIC_API_URL_CLIENT || 'https://ws.asepharyana.tech';

// Aliases for backward compatibility
export const APIURL = APIURLCLIENT;
export const BaseUrl = PRODUCTION;
