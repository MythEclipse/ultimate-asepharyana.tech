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

  // Use default context (no incognito) for session isolation, since only newPage() is available
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

  await page.goto('https://www.croxyproxy.com/', {
    waitUntil: 'domcontentloaded',
    timeout: 60000,
  });
  logger.info('Navigated to CroxyProxy homepage');
  await new Promise((resolve) => setTimeout(resolve, 3000));
  logger.debug('Waited 3 seconds before interacting with the page');
  // Wait for the main form input to load
  await page.waitForSelector('input#url', { timeout: 30000 });
  logger.debug('URL input is visible');

  // Fill the input and submit the form
  await page.type('input#url', targetUrl, { delay: 50 });
  logger.debug(`Typing URL: ${targetUrl}`);
  await page.click('#requestSubmit');
  logger.info('Submitted the form');
  // Check for error message and retry if needed
  // Check for error message in both 'h1' and '#contentBody > h1'
  // Improved error detection: check for error message after form submission and after navigation
  let errorExists = await page.evaluate(() => {
    const h1s = Array.from(
      document.querySelectorAll('h1, #contentBody > h1')
    );
    return h1s.some(
      (el) => el.textContent?.trim().toLowerCase().includes('something went wrong')
    );
  }).catch(() => false);

  if (errorExists) {
    logger.warn(
      'Detected "Something went wrong" error, retrying from homepage...'
    );
    await page.goto('https://www.croxyproxy.com/', {
      waitUntil: 'domcontentloaded',
      timeout: 60000,
    });
    logger.info('Retried navigation to CroxyProxy homepage');
    await new Promise((resolve) => setTimeout(resolve, 3000));
    await page.waitForSelector('input#url', { timeout: 30000 });
    await page.type('input#url', targetUrl, { delay: 50 });
    await page.click('#requestSubmit');
    logger.info('Resubmitted the form after error');
    // Check again after resubmission
    errorExists = await page.evaluate(() => {
      const h1s = Array.from(
        document.querySelectorAll('h1, #contentBody > h1')
      );
      return h1s.some(
        (el) => el.textContent?.trim().toLowerCase().includes('something went wrong')
      );
    }).catch(() => false);
    if (errorExists) {
      logger.error('Still detected "Something went wrong" after retry.');
      await browser.close();
      throw new Error('CroxyProxy error: Something went wrong');
    }
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
