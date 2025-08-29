import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { verifyJwt } from './lib/jwt';

export async function middleware(request: NextRequest) {
  const token = request.cookies.get('token')?.value;

  // Allow access to login and register routes without authentication
  if (
    request.nextUrl.pathname.startsWith('/api/login') ||
    request.nextUrl.pathname.startsWith('/api/register')
  ) {
    return NextResponse.next();
  }

  if (!token) {
    return NextResponse.json(
      { message: 'Authentication required' },
      { status: 401 },
    );
  }

  try {
    const decoded = await verifyJwt(token);
    if (!decoded) {
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }
    // You can attach the decoded user info to the request headers if needed
    // request.headers.set('x-user-id', decoded.userId as string);
    // request.headers.set('x-user-email', decoded.email as string);
    return NextResponse.next();
  } catch (error) {
    console.error('JWT verification failed in middleware:', error);
    return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
  }
}

export const config = {
  matcher: '/api/sosmed/:path*', // Protect only /api/sosmed routes
};
