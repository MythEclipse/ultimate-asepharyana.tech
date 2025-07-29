/**
// Bun support: Run with
//   bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://example.com/"
 * Usage (from project root):
 *   bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://asepharyana.tech/"
 *
 * This will fill the input, submit the form, and print the resulting HTML.
 */
// Scrape https://www.croxyproxy.com/ using Puppeteer with CLI support

import puppeteer from 'puppeteer';
import logger from './logger';

// Generate random user agent for session isolation
function getRandomUserAgent(): string {
  const versions = [
    '114.0.0.0',
    '115.0.0.0',
    '116.0.0.0',
    '117.0.0.0',
  ];
  const os = [
    'Windows NT 10.0; Win64; x64',
    'Macintosh; Intel Mac OS X 10_15_7'
  ];
  return `Mozilla/5.0 (${os[Math.floor(Math.random() * os.length)]}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${versions[Math.floor(Math.random() * versions.length)]} Safari/537.36`;
}

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

  const page = await browser.newPage();
  logger.debug('New page created');

  // Set a random user agent for session isolation
  await page.setUserAgent(getRandomUserAgent());
  logger.debug('Random user agent set');

  // Try to bypass some basic bot detection
  await page.setExtraHTTPHeaders({
    'Accept-Language': 'en-US,en;q=0.9',
  });
  logger.debug('Extra HTTP headers set');

  // Always start from homepage to avoid session outdated error
  let retry = 0;
  let html = '';
  while (retry < 3) {
    await page.goto('https://www.croxyproxy.com/', {
      waitUntil: 'networkidle0',
      timeout: 60000,
    });
    logger.info('Navigated to CroxyProxy homepage');
    logger.debug('Waited 3 seconds before interacting with the page');

    // Wait for the main form input to load
    await page.waitForSelector('input#url', { timeout: 30000 });
    logger.debug('URL input is visible');

    // Fill the input and submit the form
    await page.type('input#url', targetUrl, { delay: 50 });
    logger.debug(`Typing URL: ${targetUrl}`);
    await page.click('#requestSubmit');
    logger.info('Submitted the form');

    // Wait for navigation or error
    await page.waitForNavigation({ waitUntil: 'domcontentloaded', timeout: 60000 }).catch(() => {});

    // Check for session outdated or error message
    const errorText = await page.evaluate(() => {
      const h1s = Array.from(document.querySelectorAll('h1, #contentBody > h1'));
      const bodyText = document.body.innerText || '';
      let error = '';
      if (h1s.some(el => el.textContent?.toLowerCase().includes('something went wrong'))) {
        error = 'Something went wrong';
      }
      if (bodyText.toLowerCase().includes('your session has outdated')) {
        error = 'Your session has outdated. Please visit the home page and start a new one';
      }
      if (bodyText.toLowerCase().includes('web proxy server')) {
        error = 'Web proxy server error';
      }
      return error;
    });

    if (errorText === 'Your session has outdated. Please visit the home page and start a new one') {
      logger.warn('Session outdated detected, retrying from homepage...');
      retry++;
      continue;
    }
    if (errorText) {
      logger.error(errorText);
      await browser.close();
      throw new Error(errorText);
    }

    // Wait for proxy to be ready using multiple checks
    await page.waitForNavigation({ waitUntil: 'networkidle0', timeout: 120000 }).catch(() => {});
    logger.info('Proxy has launched');
    await new Promise((resolve) => setTimeout(resolve, 2000));
    logger.debug('Waited 2 seconds after proxy launch');

    // Get the full HTML content after submission
    html = await page.content();
    logger.debug('Retrieved page content');
    break;
  }

  await browser.close();
  logger.info('Browser closed');
  if (!html) throw new Error('Failed to get valid session after retries');
  return html;
}

// CLI usage: bun run apps/NextJS/lib/scrapeCroxyProxy.ts "https://asepharyana.tech/"
if (require.main === module) {
  const [, , inputUrl] = process.argv;
  if (!inputUrl) {
    logger.error(
      'Usage: bun run apps/NextJS/lib/scrapeCroxyProxy.ts "<url or query>"'
    );
    process.exit(1);
  }
  logger.info(`CLI execution started for URL: ${inputUrl}`);
  scrapeCroxyProxy(inputUrl)
    .then((html) => {
      console.log(html);
      logger.info('CLI execution finished successfully.');
    })
    .catch((err) => {
      logger.error('Scraping failed:', err);
      process.exit(1);
    });
}
