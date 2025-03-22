import logger from '@/lib/logger';
import { NextRequest, NextResponse } from 'next/server';
import { imageProxy } from '@/lib/imageproxy';
import { corsHeaders } from '@/lib/corsHeaders';

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');
  // logger.info(`Received request with URL: ${url}`);

  if (!url) {
    logger.error('URL is required');
    return NextResponse.json(
      { error: 'URL is required' },
      { status: 400, headers: corsHeaders }
    );
  }

  const response = await imageProxy(url);
  return new NextResponse(response.body, {
    ...response,
    headers: {
      ...response.headers,
      ...corsHeaders,
    },
  });
}
