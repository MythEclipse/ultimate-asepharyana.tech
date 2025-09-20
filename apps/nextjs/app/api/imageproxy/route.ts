import { NextRequest, NextResponse } from 'next/server';

export async function GET(req: NextRequest) {
  const imageUrl = req.nextUrl.searchParams.get('url');

  if (!imageUrl) {
    return new NextResponse('Image URL is required', { status: 400 });
  }

  try {
    const response = await fetch(imageUrl);

    if (!response.ok) {
      throw new Error(`Failed to fetch image: ${response.statusText}`);
    }

    const contentType = response.headers.get('content-type');
    if (!contentType || !contentType.startsWith('image/')) {
      throw new Error('URL does not point to an image');
    }

    const arrayBuffer = await response.arrayBuffer();
    const headers = new Headers();
    headers.set('Content-Type', contentType);
    headers.set('Cache-Control', 'public, max-age=31536000, immutable'); // Cache images for a long time

    return new NextResponse(arrayBuffer, { headers });
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : 'Unknown error';
    console.error('Image proxy error:', error);
    return new NextResponse(`Image proxy failed: ${message}`, {
      status: 500,
    });
  }
}
