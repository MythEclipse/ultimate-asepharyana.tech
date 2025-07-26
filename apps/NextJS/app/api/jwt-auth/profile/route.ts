import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import { jwtVerify } from 'jose';
import logger from '@/lib/logger';

export async function GET(request: Request) {
  const start = Date.now();
  const ip = 'unknown';
  let token: string | undefined;
  try {
    // Try Authorization header first, fallback to cookie
    token = request.headers.get('authorization')?.split(' ')[1];

    if (!token) {
      const cookieHeader = request.headers.get('cookie');
      if (cookieHeader) {
        const match = cookieHeader.match(/authToken=([^;]+)/);
        if (match) {
          token = match[1];
        }
      }
    }

    logger.info(`[GET /api/jwt-auth/profile] Request received`, { ip });

    if (!token) {
      logger.warn(`[GET /api/jwt-auth/profile] Unauthorized`, { ip });
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    // Use JWT_SECRET for consistency with login
    const { payload } = await jwtVerify(token, new TextEncoder().encode(process.env.JWT_SECRET));
    const user = await prisma.user.findUnique({
      where: { id: payload.id as string },
    });

    if (!user) {
      logger.warn(`[GET /api/jwt-auth/profile] User not found`, { ip, userId: payload.id });
      return NextResponse.json({ error: 'User not found' }, { status: 404 });
    }

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { password, ...userWithoutPassword } = user;

    logger.info(`[GET /api/jwt-auth/profile] Success`, {
      ip,
      userId: user.id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ user: userWithoutPassword });
  } catch (error) {
    logger.error(`[GET /api/jwt-auth/profile] Error`, {
      ip,
      token,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json({ error: 'Invalid token' }, { status: 401 });
  }
}