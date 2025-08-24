import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const url = searchParams.get('url');

  if (!url) {
    return NextResponse.json({ error: 'URL parameter is missing' }, { status: 400 });
  }

  try {
    const apiUrl = `https://api.ryzumi.vip/api/downloader/gdrive?url=${encodeURIComponent(url)}`;
    const response = await fetch(apiUrl);

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();

    if (data.downloadUrl) {
      return NextResponse.json({ downloadUrl: data.downloadUrl });
    } else {
      return NextResponse.json({ error: 'downloadUrl not found in API response' }, { status: 500 });
    }
  } catch (error) {
    console.error('Error scraping drive downloader:', error);
    return NextResponse.json({ error: 'Failed to scrape drive downloader' }, { status: 500 });
  }
}