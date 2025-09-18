export const KOMIK: string | undefined = process.env.NEXT_PUBLIC_KOMIK;
export const PRODUCTION: string =
  process.env.NEXT_PUBLIC_PROD || 'https://asepharyana.tech';
export const APIURLSERVER = 'http://localhost:4091';
export const APIURLCLIENT = 'https://ws.asepharyana.tech';

// Aliases for backward compatibility
export const APIURL = APIURLCLIENT;
export const BaseUrl = PRODUCTION;
