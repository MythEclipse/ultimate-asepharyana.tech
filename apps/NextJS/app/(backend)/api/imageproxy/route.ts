import logger from '@/lib/logger';
import { NextRequest, NextResponse } from 'next/server';
import fetch from 'node-fetch';

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');

  logger.info(`Received request with URL: ${url}`);

  if (!url) {
    logger.error('URL is required');
    return NextResponse.json({ error: 'URL is required' }, { status: 400 });
  }

  try {
    const response = await fetch(url);
    if (!response.ok) {
      logger.error(
        `Failed to fetch image from URL: ${url}, Status: ${response.status}`
      );
      return NextResponse.json(
        { error: 'Failed to fetch image' },
        { status: response.status }
      );
    }

    const contentType = response.headers.get('content-type');
    if (!contentType || !contentType.startsWith('image/')) {
      logger.error(`URL does not point to an image: ${url}`);
      return NextResponse.json(
        { error: 'URL does not point to an image' },
        { status: 400 }
      );
    }

    const imageBuffer = await response.buffer();
    logger.info(`Successfully fetched image from URL: ${url}`);
    return new NextResponse(imageBuffer, {
      headers: { 'Content-Type': contentType },
    });
  } catch (error) {
    logger.error(`Internal server error: ${(error as Error).message}`);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
