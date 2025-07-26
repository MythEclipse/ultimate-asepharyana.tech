import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import * as jose from 'jose';

export async function POST(request: Request) {
  try {
    const { refreshToken } = await request.json();

    if (!refreshToken) {
      return NextResponse.json({ message: 'Refresh token is required' }, { status: 400 });
    }

    const user = await prisma.user.findUnique({
      where: { refreshToken },
    });

    if (!user) {
      return NextResponse.json({ message: 'Invalid refresh token' }, { status: 401 });
    }

    // Generate a new access token
    const secret = new TextEncoder().encode(process.env.JWT_SECRET as string);
    const newToken = await new jose.SignJWT({ id: user.id, email: user.email })
      .setProtectedHeader({ alg: 'HS256' })
      .setExpirationTime('2h') // New token expires in 2 hours
      .sign(secret);

    // Optionally, rotate refresh token (generate new one and invalidate old)
    // For simplicity, we're not rotating it here, but it's a good practice for security.

    return NextResponse.json({ accessToken: newToken }, { status: 200 });

  } catch (error) {
    console.error('Error during token refresh:', error);
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}