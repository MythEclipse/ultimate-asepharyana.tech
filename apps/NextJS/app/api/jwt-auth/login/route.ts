import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service'; // Changed import
import bcrypt from 'bcrypt';
import * as jose from 'jose';

export async function POST(request: Request) {
  try {
    const { email, password } = await request.json();

    console.log('JWT_SECRET:', process.env.JWT_SECRET); // Added for debugging

    if (!email || !password) {
      return NextResponse.json({ message: 'Email and password are required' }, { status: 400 });
    }

    const user = await prisma.user.findUnique({
      where: { email },
    });

    if (!user) {
      return NextResponse.json({ message: 'Invalid credentials' }, { status: 401 });
    }

    // Check if user.password exists and is not null before comparing
    if (!user.password) {
      return NextResponse.json({ message: 'User not configured for password login' }, { status: 401 });
    }

    const isPasswordValid = await bcrypt.compare(password, user.password);

    if (!isPasswordValid) {
      return NextResponse.json({ message: 'Invalid credentials' }, { status: 401 });
    }

    const secret = new TextEncoder().encode(process.env.JWT_SECRET as string);
    const token = await new jose.SignJWT({ id: user.id, email: user.email })
      .setProtectedHeader({ alg: 'HS256' })
      .setExpirationTime('2h') // Token expires in 2 hours
      .sign(secret);

    // Set the token as a httpOnly cookie
    const response = NextResponse.json({ message: 'Login successful' }, { status: 200 });
    response.cookies.set('authToken', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production', // Use secure in production
      maxAge: 60 * 60 * 2, // 2 hours
      path: '/',
    });

    return response;
  } catch (error) {
    console.error('Error during login:', error);
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  } 
}