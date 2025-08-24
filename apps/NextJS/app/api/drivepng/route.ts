import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const fileId = searchParams.get('id'); // ?id=FILE_ID

  if (!fileId) {
    return NextResponse.json({ error: 'id parameter is missing' }, { status: 400 });
  }

  try {
    const apiKey = process.env.GDRIVE_API_KEY; // taruh di .env.local
    if (!apiKey) {
      return NextResponse.json({ error: 'Missing Google Drive API key' }, { status: 500 });
    }

    // official Google Drive API endpoint
    const apiUrl = `https://www.googleapis.com/drive/v3/files/${fileId}?alt=media&key=${apiKey}`;

    const response = await fetch(apiUrl);

    if (!response.ok) {
      throw new Error(`Google API error! status: ${response.status}`);
    }

    // kalau mau langsung return binary file
    const arrayBuffer = await response.arrayBuffer();
    return new NextResponse(arrayBuffer, {
      headers: {
        'Content-Type': response.headers.get('content-type') || 'application/octet-stream',
      },
    });

    // kalau hanya butuh link JSON, bisa return:
    // return NextResponse.json({ downloadUrl: apiUrl });

  } catch (error) {
    console.error('Error fetching from Google Drive API:', error);
    return NextResponse.json({ error: 'Failed to fetch from Drive API' }, { status: 500 });
  }
}
