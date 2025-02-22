// middleware.ts
import { auth } from './auth';
import { NextResponse } from 'next/server';

import { NextRequest as OriginalNextRequest } from 'next/server';

interface NextRequest extends OriginalNextRequest {
  auth?: boolean;
}

export default auth((req: NextRequest) => {
  const isDev = process.env.NODE_ENV === 'development';
  if (!req.auth && !isDev) {
    const url = req.nextUrl.clone();
    url.pathname = '/login';
    url.searchParams.set('callbackUrl', req.nextUrl.pathname);
    return NextResponse.redirect(url);
  }
  return NextResponse.next();
});

export const config = {
  matcher: ['/((?!api|_next/static|_next/image|favicon.ico|login).*)'],
};
