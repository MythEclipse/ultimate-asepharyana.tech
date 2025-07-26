import { v4 as uuidv4 } from 'uuid';
import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import bcrypt from 'bcrypt';
import * as jose from 'jose';
import logger from '@/lib/logger';

export async function POST(request: Request) {
  const start = Date.now();
  const ip = 'unknown';
  let email, password;
  try {
    const body = await request.json();
    email = body.email;
    password = body.password;
    logger.info(`[POST /api/jwt-auth/login] Request received`, { ip, email });
  } catch (jsonError) {
    logger.error(`[POST /api/jwt-auth/login] JSON parse error`, {
      ip,
      error: jsonError,
      durationMs: Date.now() - start,
    });
    return NextResponse.json({ message: 'Invalid request body format. Expected JSON.' }, { status: 400 });
  }

  try {
    if (!email || !password) {
      logger.warn(`[POST /api/jwt-auth/login] Email/password required`, { ip });
      return NextResponse.json({ message: 'Email and password are required' }, { status: 400 });
    }

    const user = await prisma.user.findUnique({
      where: { email },
    });

    if (!user) {
      logger.warn(`[POST /api/jwt-auth/login] Invalid credentials`, { ip, email });
      return NextResponse.json({ message: 'Invalid credentials' }, { status: 401 });
    }

    if (!user.password) {
      logger.warn(`[POST /api/jwt-auth/login] No password set`, { ip, email });
      return NextResponse.json({ message: 'User not configured for password login' }, { status: 401 });
    }

    const isPasswordValid = await bcrypt.compare(password, user.password);

    if (!isPasswordValid) {
      logger.warn(`[POST /api/jwt-auth/login] Invalid credentials`, { ip, email });
      return NextResponse.json({ message: 'Invalid credentials' }, { status: 401 });
    }

    const secret = new TextEncoder().encode(process.env.JWT_SECRET as string);
    const token = await new jose.SignJWT({ id: user.id, email: user.email })
      .setProtectedHeader({ alg: 'HS256' })
      .setExpirationTime('2h')
      .sign(secret);

    const refreshToken = uuidv4();

    await prisma.user.update({
      where: { id: user.id },
      data: { refreshToken },
    });

    const response = NextResponse.json({ message: 'Login successful', token, refreshToken }, { status: 200 });
    response.cookies.set('authToken', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      maxAge: 60 * 60 * 2,
      path: '/',
    });

    logger.info(`[POST /api/jwt-auth/login] Login successful`, {
      ip,
      userId: user.id,
      durationMs: Date.now() - start,
    });

    return response;
  } catch (error) {
    logger.error(`[POST /api/jwt-auth/login] Error`, {
      ip,
      email,
      error,
      durationMs: Date.now() - start,
    });
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}