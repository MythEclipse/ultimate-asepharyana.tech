import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import { jwtVerify } from 'jose';

export async function GET(request: Request) {
  // Try Authorization header first, fallback to cookie
  let token = request.headers.get('authorization')?.split(' ')[1];

  if (!token) {
    const cookieHeader = request.headers.get('cookie');
    if (cookieHeader) {
      const match = cookieHeader.match(/authToken=([^;]+)/);
      if (match) {
        token = match[1];
      }
    }
  }

  if (!token) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }

  try {
    // Use JWT_SECRET for consistency with login
    const { payload } = await jwtVerify(token, new TextEncoder().encode(process.env.JWT_SECRET));
    const user = await prisma.user.findUnique({
      where: { id: payload.id as string },
    });

    if (!user) {
      return NextResponse.json({ error: 'User not found' }, { status: 404 });
    }

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { password, ...userWithoutPassword } = user;
    return NextResponse.json({ user: userWithoutPassword });
  } catch (error) {
    console.error('Error verifying token:', error);
    return NextResponse.json({ error: 'Invalid token' }, { status: 401 });
  }
}