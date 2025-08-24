import { NextResponse, NextRequest } from 'next/server';
import { withLogging } from '@/lib/api-wrapper';
import { fetchWithProxyOnly } from '@/lib/fetchWithProxy';
import logger from '@/utils/logger';

async function handler(request: NextRequest) {
  const url = new URL(request.url);
  const slug = url.searchParams.get('url');

  if (!slug) {
    return NextResponse.json(
      { error: 'Missing url parameter' },
      { status: 400 }
    );
  }

  try {
    const result = await fetchWithProxyOnly(slug);

    // If the response is JSON, return as JSON
    if (result.contentType && result.contentType.includes('application/json')) {
      // If already parsed as object, return as JSON
      if (typeof result.data === 'object') {
        return NextResponse.json(result.data, {
          status: 200,
          headers: {
            'X-Proxy-Used': 'fetchWithProxy',
          },
        });
      }
      // If string, try to parse as JSON
      try {
        const parsed = JSON.parse(result.data as string);
        return NextResponse.json(parsed, {
          status: 200,
          headers: {
            'X-Proxy-Used': 'fetchWithProxy',
          },
        });
      } catch {
        // Fallback: return as text
        return new NextResponse(result.data as string, {
          status: 200,
          headers: {
            'content-type': result.contentType,
            'X-Proxy-Used': 'fetchWithProxy',
          },
        });
      }
    }

    // For other content types, return as raw text or buffer
    return new NextResponse(
      typeof result.data === 'string'
        ? result.data
        : JSON.stringify(result.data),
      {
        status: 200,
        headers: {
          'content-type': result.contentType || 'text/plain',
          'X-Proxy-Used': 'fetchWithProxy',
        },
      }
    );
  } catch (error) {
    logger.error('Failed to fetch URL', {
      error: (error as Error).message,
      stack: (error as Error).stack,
      url: slug,
    });
    // Narrow error type to safely access custom properties
    const err = error as Error & {
      status?: number;
      response?: unknown;
      code?: string;
    };
    return NextResponse.json(
      {
        error: 'Failed to fetch URL',
        details: err.message,
        stack: err.stack,
        status: err.status || 500,
        response: err.response || undefined,
        code: err.code || undefined,
      },
      { status: err.status || 500 }
    );
  }
}

// Ensure withLogging only passes the Request object to handler
export const GET = (request: NextRequest) =>
  withLogging(handler)(request, { params: {} });
