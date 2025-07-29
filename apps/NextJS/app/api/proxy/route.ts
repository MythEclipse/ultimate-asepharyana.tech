import { NextResponse } from 'next/server';
import { withLogging } from '@/lib/api-wrapper';
import { fetchWithProxyOnly } from '@/lib/fetchWithProxy';

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
    // Use fetchWithProxy for all proxying
    const result = await fetchWithProxyOnly(slug);

    // If the response is JSON, return it as JSON
    if (typeof result.data === 'object') {
      return NextResponse.json(result.data, {
        status: 200,
        headers: {
          'X-Proxy-Used': 'fetchWithProxy'
        }
      });
    }

    // For non-JSON responses, return as text in a JSON envelope
    return NextResponse.json(
      {
        data: result.data,
        proxy: 'fetchWithProxy',
        contentType: result.contentType || 'text/plain',
      },
      {
        status: 200,
        headers: {
          'X-Proxy-Used': 'fetchWithProxy'
        }
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
