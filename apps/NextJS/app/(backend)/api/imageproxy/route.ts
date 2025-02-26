import logger from '@/lib/logger';
import { BaseUrl } from '@/lib/url';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const url = searchParams.get('url');

  // logger.info(`Received request with URL: ${url}`);

  if (!url) {
    logger.error('URL is required');
    return NextResponse.json({ error: 'URL is required' }, { status: 400 });
  }
  const cdnResponse = await cdnImage(url);
  if (cdnResponse.status !== 200) {
    const fetchRespone = await fetchManual(url);
    if (fetchRespone.status !== 200) {
      logger.error(`Failed to fetch image from URL: ${url}`);
      return await uploadImage(url);
    }
    logger.info(`Successfully fetched image from URL: ${url}`);
    return fetchRespone;
  }
  logger.info(`Successfully fetched image from CDN: ${url}`);
  return cdnResponse;
}
//hihahahahahahahahahhahahahah
//hihahahahahahahahahhahahahah
//hihahahahahahahahahhahahahah
//hihahahahahahahahahhahahahah
//hihahahahahahahahahhahahahah
//hihahahahahahahahahhahahahah
//hihahahahahahahahahhahahahah
async function cdnImage(url: string) {
  try {
    const response = await fetch(
      `https://imagecdn.app/v1/images/${encodeURIComponent(url)}`
    );
    if (!response.ok) {
      logger.error(
        `Failed to fetch image from CDN: ${url}, Status: ${response.status}`
      );
      return NextResponse.json(
        { error: 'Failed to fetch image from CDN' },
        { status: response.status }
      );
    }

    const contentType = response.headers.get('content-type');
    if (!contentType || !contentType.startsWith('image/')) {
      logger.error(`CDN URL does not point to an image: ${url}`);
      return NextResponse.json(
        { error: 'CDN URL does not point to an image' },
        { status: 400 }
      );
    }

    const arrayBuffer = await response.arrayBuffer();
    const blob = new Blob([arrayBuffer], { type: contentType });

    return new NextResponse(blob, {
      headers: {
        'Content-Type': contentType,
      },
    });
  } catch (error) {
    logger.error(`Internal server error: ${(error as Error).message}`);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

async function uploadImage(url: string) {
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

    const arrayBuffer = await response.arrayBuffer();
    const blob = new Blob([arrayBuffer], { type: contentType });
    const formData = new FormData();
    formData.append('file', blob, 'image.jpg');

    const uploadResponse = await fetch(`${BaseUrl}/api/uploader`, {
      method: 'POST',
      headers: {
        accept: 'application/json',
      },
      body: formData,
    });

    if (!uploadResponse.ok) {
      logger.error(
        `Failed to upload image to uploader service, Status: ${uploadResponse.status}`
      );
      return NextResponse.json(
        { error: 'Failed to upload image' },
        { status: uploadResponse.status }
      );
    }

    const uploadResult = await uploadResponse.json();
    const imageResponse = await fetch(uploadResult.url);
    const imageBuffer = await imageResponse.arrayBuffer();
    const imageBlob = new Blob([imageBuffer], { type: contentType });

    return new NextResponse(imageBlob, {
      headers: {
        'Content-Type': contentType,
      },
    });
    logger.info(
      `Successfully uploaded image to uploader service: ${uploadResult.url}`
    );
    return NextResponse.json({ url: uploadResult.url }, { status: 200 });
  } catch (error) {
    logger.error(`Internal server error: ${(error as Error).message}`);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

async function fetchManual(url: string) {
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

    const imageBuffer = await response.arrayBuffer();
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
