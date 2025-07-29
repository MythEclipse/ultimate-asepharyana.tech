/**
// Bun support: Run with
//   bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://example.com/"
 * Usage (from project root):
 *   bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://asepharyana.tech/"
 *
 * This will fill the input, submit the form, and print the resulting HTML.
 */
// Scrape https://www.croxyproxy.com/ using Puppeteer with stealth and CLI support
import puppeteer from 'puppeteer-extra';
import StealthPlugin from 'puppeteer-extra-plugin-stealth';
import logger from './logger';

puppeteer.use(StealthPlugin());

export async function scrapeCroxyProxy(targetUrl: string): Promise<string> {
  logger.info(`Scraping ${targetUrl} with CroxyProxy`);
  const browser = await puppeteer.launch({
    headless: false,
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--disable-blink-features=AutomationControlled',
    ],
  });
  logger.debug('Browser launched');
  const page = await browser.newPage();
  logger.debug('New page created');

  // Set a realistic user agent
  await page.setUserAgent(
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36'
  );
  logger.debug('User agent set');

  // Try to bypass some basic bot detection
  await page.setExtraHTTPHeaders({
    'Accept-Language': 'en-US,en;q=0.9',
  });
  logger.debug('Extra HTTP headers set');

  await page.goto('https://www.croxyproxy.com/', {
    waitUntil: 'domcontentloaded',
    timeout: 60000,
  });
  logger.info('Navigated to CroxyProxy homepage');
  await new Promise(resolve => setTimeout(resolve, 3000));
  logger.debug('Waited 1 second before interacting with the page');
  // Wait for the main form input to load
  await page.waitForSelector('input#url', { timeout: 30000 });
  logger.debug('URL input is visible');
  
  // Fill the input and submit the form
  await page.type('input#url', targetUrl, { delay: 50 });
  logger.debug(`Typing URL: ${targetUrl}`);
  await page.click('#requestSubmit');
  logger.info('Submitted the form');
  // Check for error message and retry if needed
  const errorExists = await page.$eval('h1', el => el.textContent?.trim() === 'Something went wrong').catch(() => false);
  if (errorExists) {
    logger.warn('Detected "Something went wrong" error, retrying from homepage...');
    await page.goto('https://www.croxyproxy.com/', {
      waitUntil: 'domcontentloaded',
      timeout: 60000,
    });
    logger.info('Retried navigation to CroxyProxy homepage');
    await new Promise(resolve => setTimeout(resolve, 3000));
    await page.waitForSelector('input#url', { timeout: 30000 });
    await page.type('input#url', targetUrl, { delay: 50 });
    await page.click('#requestSubmit');
    logger.info('Resubmitted the form after error');
  }
  // Wait for proxy to be ready using multiple checks
  await page.waitForNavigation({ waitUntil: 'networkidle0', timeout: 120000 });
  logger.info('Proxy has launched');

  // Get the full HTML content after submission
  const html = await page.content();
  logger.debug('Retrieved page content');
  await browser.close();
  logger.info('Browser closed');
  return html;
}

// async function waitForProxyReady(page: import('puppeteer').Page) {
//   logger.debug('Waiting for proxy to be ready...');
//   try {
//     // Check for video element AND ensure page title has changed
//     await Promise.all([
//       page.waitForSelector('video', { timeout: 60000 }),
//       page.waitForFunction(() => document.title !== 'CroxyProxy Free Online Proxy - Hide IP Address')
//     ]);
//     logger.debug('Proxy ready indicators detected');
    
//     // Add a short delay to ensure stability
//     await new Promise(resolve => setTimeout(resolve, 2000));
//   } catch (error) {
//     logger.error('Proxy ready indicator not found:', error);
//     throw new Error('Proxy did not launch in time');
//   }
// }

// CLI usage: bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://asepharyana.tech/"
if (require.main === module) {
  const [, , inputUrl] = process.argv;
  if (!inputUrl) {
    logger.error('Usage: bun run apps/NextJS/lib/scrapeCroxyProxy.ts "<url or query>"');
    process.exit(1);
  }
  logger.info(`CLI execution started for URL: ${inputUrl}`);
  scrapeCroxyProxy(inputUrl)
    .then(html => {
      console.log(html);
      logger.info('CLI execution finished successfully.');
    })
    .catch(err => {
      logger.error('Scraping failed:', err);
      process.exit(1);
    });
}
