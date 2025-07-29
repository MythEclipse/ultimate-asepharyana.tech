// Wrapped GET handler with withLogging for centralized logging
// Fixed: Always return NextResponse for compatibility with withLogging

import { ProxyManager } from 'proxy-auto-ts';
import { NextResponse } from 'next/server';
import { withLogging } from '@/lib/api-wrapper';

// Initialize ProxyManager with custom configuration
const proxyManager = new ProxyManager({
  timeout: 12000,
  validationTimeout: 8000,
  fallbackUrls: ['https://httpbin.org/ip', 'https://api.ipify.org?format=json'],
});

async function handler(request: Request) {
  const url = new URL(request.url);
  const slug = url.searchParams.get('url');

  if (!slug) {
    return NextResponse.json(
      { error: 'Missing url parameter' },
      { status: 400 }
    );
  }

  try {
    // Use ProxyManager to fetch with automatic proxy rotation
    const result = await proxyManager.fetchWithProxy(slug, 3);

    // If the response is JSON, return it as JSON
    if (typeof result.data === 'object') {
      return NextResponse.json(result.data, {
        status: 200,
        headers: {
          'X-Proxy-Used': result.proxy,
          'X-Proxy-Latency': result.latency.toString(),
        },
      });
    }

    // For non-JSON responses, return as text in a JSON envelope
    return NextResponse.json(
      {
        data: result.data,
        proxy: result.proxy,
        latency: result.latency,
        contentType: 'text/plain',
      },
      {
        status: 200,
        headers: {
          'X-Proxy-Used': result.proxy,
          'X-Proxy-Latency': result.latency.toString(),
        },
      }
    );
  } catch (error) {
    return NextResponse.json(
      {
        error: 'Failed to fetch URL',
        details: (error as Error).message,
      },
      { status: 500 }
    );
  }
}

export const GET = withLogging(handler);
