import { NextResponse, NextRequest } from 'next/server';
import logger from '@/lib/logger';
import { PRODUCTION } from '@/lib/url';

declare global {
  interface RequestInit {
    duplex?: 'half' | 'full';
  }
}

const constructUrl = (base: string, path: string) =>
  new URL(path, base.endsWith('/') ? base.slice(0, -1) : base).toString();

const ALLOWED_HEADERS = ['content-type', 'content-length'];

async function compressImageFromBody(
  body: ReadableStream<Uint8Array> | null,
  headers: HeadersInit
) {
  const abortController = new AbortController();
  const timeoutId = setTimeout(() => abortController.abort(), 10000);

  try {
    const apiUrl = 'https://staging.kecilin.id/api/upload_compress';

    const response = await fetch(apiUrl, {
      method: 'POST',
      headers,
      body,
      duplex: 'half',
      signal: abortController.signal,
    });

    clearTimeout(timeoutId);

    if (!response.ok) {
      const errorBody = await response.text();
      logger.error(`API Error ${response.status}: ${errorBody}`);
      throw new Error(`API request failed: ${response.status}`);
    }

    const contentType = response.headers.get('content-type');
    if (!contentType?.includes('application/json')) {
      throw new Error('Invalid response format');
    }

    const responseData = await response.json();

    if (!responseData.data?.filename) {
      throw new Error('Invalid response structure');
    }

    return {
      status: responseData.status,
      message: responseData.message,
      data: {
        size_ori: responseData.data.size_ori,
        compress_size: responseData.data.compress_size,
        filename: responseData.data.filename,
        link: constructUrl(
          PRODUCTION,
          `/api/img-compress?url=${encodeURIComponent(responseData.data.filename)}`
        ),
      },
    };
  } catch (error) {
    clearTimeout(timeoutId);
    logger.error('Compress Image Error:', error);
    throw error;
  }
}

export async function POST(request: NextRequest) {
  try {
    const headers = new Headers();
    ALLOWED_HEADERS.forEach((header) => {
      const value = request.headers.get(header);
      if (value) headers.set(header, value);
    });

    const result = await compressImageFromBody(request.body, headers);
    return NextResponse.json(result);
  } catch (error) {
    const status =
      error instanceof Error && error.message.includes('API request failed')
        ? Number(error.message.split(': ')[1]) || 500
        : 500;

    return NextResponse.json(
      { status, message: 'Proxy error occurred' },
      { status }
    );
  }
}

export async function GET(request: NextRequest) {
    try {
        const { searchParams } = new URL(request.url);
        const fileName = searchParams.get('url');

        if (!fileName) {
            return NextResponse.json(
                { status: 400, message: "Bad Request: Missing fileName parameter" },
                { status: 400 }
            );
        }

        const apiUrl = `https://staging.kecilin.id/api/upload_compress/${fileName}`;

        const response = await fetch(apiUrl);

        if (!response.ok) {
            throw new Error(`API request failed: ${response.status} ${response.statusText}`);
        }

        const responseData = await response.blob();

        return new Response(responseData, {
            headers: {
                'Content-Type': response.headers.get('Content-Type') || 'application/octet-stream',
            },
        });
    } catch (error) {
        logger.error("Proxy error:", error);
        return NextResponse.json(
            { status: 500, message: "Internal Server Error" },
            { status: 500 }
        );
    }
}

export const config = {
  api: {
    bodyParser: false,
  },
};
