import { NextResponse } from 'next/server';
import { cookies } from 'next/headers';
import * as jose from 'jose';
import { verifyJwt } from '@/lib/authUtils'; // Import verifyJwt
import { prisma } from '@/lib/prisma/service'; // Added prisma import, though not directly used in refresh logic here, but might be implicitly for types

export async function POST(request: Request) {
  try {
    const cookieStore = await cookies();
    const oldToken = cookieStore.get('authToken')?.value;

    if (!oldToken) {
      return NextResponse.json({ message: 'No token provided' }, { status: 401 });
    }

    const decodedToken = await verifyJwt(oldToken);

    if (!decodedToken) {
      return NextResponse.json({ message: 'Invalid token' }, { status: 401 });
    }

    // Create a new token with a refreshed expiration time
    const secret = new TextEncoder().encode(process.env.JWT_SECRET as string); // Changed to JWT_SECRET
    const newToken = await new jose.SignJWT({ id: decodedToken.id, email: decodedToken.email })
      .setProtectedHeader({ alg: 'HS256' })
      .setExpirationTime('2h') // Refresh token expires in 2 hours
      .sign(secret);

    const response = NextResponse.json({ message: 'Token refreshed successfully' }, { status: 200 });
    response.cookies.set('authToken', newToken, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production', // Use secure in production
      maxAge: 60 * 60 * 2, // 2 hours
      path: '/',
    });

    return response;
  } catch (error) {
    console.error('Error during token refresh:', error);
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}