import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import bcrypt from 'bcrypt';
import logger from '@/lib/logger';

export async function POST(request: Request) {
  const start = Date.now();
  const ip = 'unknown';
  try {
    const { name, email, password } = await request.json();
    logger.info(`[POST /api/jwt-auth/register] Request received`, { ip, email });

    if (!name || !email || !password) {
      logger.warn(`[POST /api/jwt-auth/register] Missing fields`, { ip, email });
      return NextResponse.json({ message: 'Name, email, and password are required' }, { status: 400 });
    }

    const existingUser = await prisma.user.findUnique({
      where: { email },
    });

    if (existingUser) {
      logger.warn(`[POST /api/jwt-auth/register] Email exists`, { ip, email });
      return NextResponse.json({ message: 'User with this email already exists' }, { status: 409 });
    }

    const hashedPassword = await bcrypt.hash(password, 10);

    const newUser = await prisma.user.create({
      data: {
        name,
        email,
        password: hashedPassword,
      },
    });

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const { password: _, ...userWithoutPassword } = newUser;

    logger.info(`[POST /api/jwt-auth/register] Registration successful`, {
      ip,
      userId: newUser.id,
      durationMs: Date.now() - start,
    });

    return NextResponse.json({ message: 'Registration successful', user: userWithoutPassword }, { status: 201 });
  } catch (error) {
    logger.error(`[POST /api/jwt-auth/register] Error`, {
      ip,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}