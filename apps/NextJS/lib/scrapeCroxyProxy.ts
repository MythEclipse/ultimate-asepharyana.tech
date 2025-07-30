/**
// Bun support: Run with
//   bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://example.com/"
 * Usage (from project root):
 *   bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://asepharyana.tech/"
 *
 * This will fill the input, submit the form, and print the resulting HTML.
 */
// Scrape https://www.croxyproxy.com/ using Puppeteer with CLI supportimport puppeteer, { Browser, Page } from 'puppeteer';
import logger from './logger';
import puppeteer, { Browser, Page } from 'puppeteer';

const CROXY_PROXY_URL = 'https://www.croxyproxy.com/';
const URL_INPUT_SELECTOR = 'input#url';
const SUBMIT_BUTTON_SELECTOR = '#requestSubmit';
const MAX_RETRIES = 3;
const BROWSER_ARGS = [
  // Argumen Anda yang sudah ada (bagus untuk stabilitas & anti-deteksi)
  '--no-sandbox',
  '--disable-setuid-sandbox',
  '--disable-blink-features=AutomationControlled',
  '--disable-dev-shm-usage',
  '--disable-infobars',
  '--window-size=1280,800',
  
  // Argumen Tambahan untuk Performa & Tanpa Media
  '--blink-settings=imagesEnabled=false', // <-- Kunci: Menonaktifkan gambar
  '--disable-gpu',                     // Nonaktifkan akselerasi hardware GPU
  '--disable-extensions',              // Nonaktifkan ekstensi browser
  '--mute-audio',                      // Matikan audio
];

function getRandomUserAgent(): string {
  const versions = ['115.0.0.0', '116.0.0.0', '117.0.0.0', '118.0.0.0'];
  const os = [
    'Windows NT 10.0; Win64; x64',
    'Macintosh; Intel Mac OS X 10_15_7',
  ];
  const randomOS = os[Math.floor(Math.random() * os.length)];
  const randomVersion = versions[Math.floor(Math.random() * versions.length)];
  return `Mozilla/5.0 (${randomOS}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${randomVersion} Safari/537.36`;
}

async function initializeBrowser(): Promise<{ browser: Browser; page: Page }> {
  const browser = await puppeteer.launch({
    headless: false,
    args: BROWSER_ARGS,
    defaultViewport: { width: 1280, height: 800 },
  });
  const page = await browser.newPage();
  await page.setUserAgent(getRandomUserAgent());
  await page.setExtraHTTPHeaders({ 'Accept-Language': 'en-US,en;q=0.9' });
  return { browser, page };
}

export async function scrapeCroxyProxy(targetUrl: string): Promise<string> {
  logger.info(`Scraping ${targetUrl} with CroxyProxy`);
  const { browser, page } = await initializeBrowser();
  let html = '';

  for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
    try {
      logger.info(`Attempt ${attempt}/${MAX_RETRIES}`);
      await page.goto(CROXY_PROXY_URL, {
        waitUntil: 'domcontentloaded',
        timeout: 60000,
      });

      await page.waitForSelector(URL_INPUT_SELECTOR, { timeout: 30000 });
      await page.type(URL_INPUT_SELECTOR, targetUrl, { delay: 50 });
      
      logger.info('Form submitted. Waiting for initial navigation...');
      await Promise.all([
        page.waitForNavigation({ waitUntil: 'domcontentloaded', timeout: 60000 }),
        page.click(SUBMIT_BUTTON_SELECTOR),
      ]);
      
      const pageContent = await page.content();
      const pageText = pageContent.toLowerCase();
      const currentUrl = page.url();

      if (currentUrl === CROXY_PROXY_URL && pageText.includes('your session has outdated')) {
        logger.warn('Session outdated detected, retrying...');
        continue;
      }

      if (pageText.includes('proxy is launching')) {
        logger.info('Proxy launching page detected. Waiting for final redirect...');
        await page.waitForNavigation({ waitUntil: 'load', timeout: 120000 });
        logger.info(`Redirected successfully to: ${page.url()}`);
      } else {
        logger.info(`Mapsd directly to: ${page.url()}`);
      }
      
      logger.info('Waiting for CroxyProxy frame to render...');
      await page.waitForSelector('#__cpsHeaderTab', { timeout: 30000 });
      logger.info('CroxyProxy frame rendered.');

      html = await page.content();
      logger.debug('Retrieved page content.');
      break;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      logger.error(`Attempt ${attempt} failed: ${errorMessage}`);
      if (attempt === MAX_RETRIES) {
        await browser.close();
        throw new Error(`Failed to scrape after ${MAX_RETRIES} attempts: ${errorMessage}`);
      }
    }
  }

  await browser.close();
  logger.info('Browser closed.');

  if (!html) {
    throw new Error('Failed to retrieve HTML content after all retries.');
  }
  return html;
}

if (require.main === module) {
  const [, , inputUrl] = process.argv;
  if (!inputUrl) {
    logger.error('Usage: bun run apps/NextJS/lib/scrapeCroxyProxy.ts "<url>"');
    process.exit(1);
  }

  logger.info(`CLI execution started for URL: ${inputUrl}`);
  scrapeCroxyProxy(inputUrl)
    .then((resultHtml) => {
      console.log(resultHtml);
      logger.info('CLI execution finished successfully.');
    })
    .catch((err) => {
      logger.error(`Scraping failed: ${err.message}`);
      process.exit(1);
    });
}