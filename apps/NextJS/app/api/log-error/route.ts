// apps/NextJS/app/api/log-error/route.ts

import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
  try {
    const { error, info } = await req.json();
    // Here you could log to a database, external service, or file
    // For now, just log to server console
    console.error('[API Error Log]', { error, info });
    return NextResponse.json({ success: true });
  } catch (e) {
    return NextResponse.json({ success: false, error: (e as Error).message }, { status: 500 });
  }
}