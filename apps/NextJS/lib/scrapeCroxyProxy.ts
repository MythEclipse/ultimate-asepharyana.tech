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

puppeteer.use(StealthPlugin());

export async function scrapeCroxyProxy(targetUrl: string): Promise<string> {
  const browser = await puppeteer.launch({
    headless: true,
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--disable-blink-features=AutomationControlled',
    ],
  });
  const page = await browser.newPage();

  // Set a realistic user agent
  await page.setUserAgent(
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36'
  );

  // Try to bypass some basic bot detection
  await page.setExtraHTTPHeaders({
    'Accept-Language': 'en-US,en;q=0.9',
  });

  await page.goto('https://www.croxyproxy.com/', {
    waitUntil: 'networkidle2',
    timeout: 60000,
  });

  // Wait for the main form input to load
  await page.waitForSelector('input#url', { timeout: 30000 });

  // Fill the input and submit the form
  await page.type('input#url', targetUrl, { delay: 50 });
  await page.click('#requestSubmit');

  // Wait for navigation or page update (adjust selector as needed)
  await new Promise(res => setTimeout(res, 5000));

  // Get the full HTML content after submission
  const html = await page.content();

  await browser.close();
  return html;
}

// CLI usage: npx ts-node scrapeCroxyProxy.ts "https://asepharyana.tech/"
if (require.main === module) {
  const [, , inputUrl] = process.argv;
  if (!inputUrl) {
    console.error('Usage: npx ts-node scrapeCroxyProxy.ts "<url or query>"');
    process.exit(1);
  }
  scrapeCroxyProxy(inputUrl)
    .then(html => {
      console.log(html);
    })
    .catch(err => {
      console.error('Scraping failed:', err);
      process.exit(1);
    });
}