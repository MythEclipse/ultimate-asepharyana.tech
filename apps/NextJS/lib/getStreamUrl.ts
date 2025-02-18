// import fetch from 'node-fetch';
// import { BaseUrl } from './url';
// import { error } from 'console';
// import puppeteer from 'puppeteer';
// interface AnimeResponse {
//   status: string;
//   data: AnimeData;
// }

// interface AnimeData {
//   episode: string;
//   episode_number: string;
//   anime: AnimeInfo;
//   has_next_episode: boolean;
//   next_episode: EpisodeInfo | null;
//   has_previous_episode: boolean;
//   previous_episode: EpisodeInfo | null;
//   stream_url: string;
//   download_urls: Record<string, { server: string; url: string }[]>;
//   image_url: string;
// }

// interface AnimeInfo {
//   slug: string;
// }

// interface EpisodeInfo {
//   slug: string;
// }

// async function getmegaUrl(slug: string): Promise<string | null> {
//   const response = await fetch(`${BaseUrl}/api/anime/full/${slug}`);
//   const anime: AnimeResponse = (await response.json()) as AnimeResponse;

//   if (anime.status !== 'Ok') {
//     return null;
//   }

//   const megaUrls = anime.data.download_urls['Mp4 720p'].filter(
//     (url) => url.server === 'Mega'
//   );
//   return megaUrls.length > 0 ? megaUrls[0].url : null;
// }
// async function getpdrainUrl(slug: string): Promise<string | null> {
//   const response = await fetch(`${BaseUrl}/api/anime/full/${slug}`);
//   const anime: AnimeResponse = (await response.json()) as AnimeResponse;

//   if (anime.status !== 'Ok') {
//     return null;
//   }

//   const megaUrls = anime.data.download_urls['Mp4 720p'].filter(
//     (url) => url.server === 'Pdrain'
//   );
//   return megaUrls.length > 0 ? megaUrls[0].url : null;
// }

// export async function getStreamUrl(slug: string) {
//   const pdrainUrl = await getpdrainUrl(slug);
//   console.log(pdrainUrl);
//   if (pdrainUrl) {
//     try {
//       const browser = await puppeteer.launch({
//         headless: true,
//         args: ['--no-sandbox', '--disable-setuid-sandbox'],
//       });
//       const page = await browser.newPage();
//       await page.goto(pdrainUrl, { waitUntil: 'networkidle2' });
//       await page.waitForSelector('video');
//       await page.waitForNetworkIdle();
//       const streamUrl = await page.evaluate(() => {
//         const video = document.querySelector('video') as HTMLVideoElement;
//         console.log(video);
//         console.log(video.src);
//         console.log(window.location.href);
//         return video ? video.src : window.location.href;
//       });
//       await browser.close();
//       console.log(streamUrl);
//       if (streamUrl) {
//         return streamUrl;
//       }
//     } catch (err) {
//       return error('Failed to fetch stream url');
//     }
//   }
// }
