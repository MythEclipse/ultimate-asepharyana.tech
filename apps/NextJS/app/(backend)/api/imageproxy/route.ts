import logger from '@/lib/logger';
import { NextRequest, NextResponse } from 'next/server';
import { imageProxy } from '@/lib/imageproxy';

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');
  // logger.info(`Received request with URL: ${url}`);

  if (!url) {
    logger.error('URL is required');
    return NextResponse.json({ error: 'URL is required' }, { status: 400 });
  }
  return await imageProxy(url);
}
