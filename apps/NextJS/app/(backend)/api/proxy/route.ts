import { ProxyManager } from 'proxy-auto-ts';
import { NextResponse } from 'next/server';

// Initialize ProxyManager with custom configuration
const proxyManager = new ProxyManager({
  timeout: 12000,
  validationTimeout: 8000,
  fallbackUrls: [
    'https://httpbin.org/ip',
    'https://api.ipify.org?format=json'
  ],
  userAgents: [
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
    'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
    'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
  ]
});

export async function GET(request: Request) {
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
      return NextResponse.json(result.data);
    }
    
    // For non-JSON responses, return as text
    return new Response(result.data, {
      headers: { 
        'Content-Type': 'text/plain',
        'X-Proxy-Used': result.proxy,
        'X-Proxy-Latency': result.latency.toString()
      }
    });
    
  } catch (error) {
    return NextResponse.json(
      { 
        error: 'Failed to fetch URL', 
        details: (error as Error).message 
      },
      { status: 500 }
    );
  }
}
