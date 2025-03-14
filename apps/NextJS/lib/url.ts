export const ANIMEAPI: string = 'https://otakudesu.asepharyana.cloud';
export const KOMIK: string | undefined = process.env.NEXT_PUBLIC_KOMIK;
export const PRODUCTION: string =
  process.env.NEXT_PUBLIC_PROD || 'https://asepharyana.cloud';
export const BaseUrl: string =
  process.env.NODE_ENV === 'development'
    ? 'http://localhost:4090'
    : 'https://asepharyana.cloud';
