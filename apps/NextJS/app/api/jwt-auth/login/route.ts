import { v4 as uuidv4 } from 'uuid';
import { NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma/service';
import bcrypt from 'bcrypt';
import * as jose from 'jose';

export async function POST(request: Request) {
  let email, password;

  try {
    const body = await request.json();
    email = body.email;
    password = body.password;
  } catch (jsonError) {
    console.error('Error parsing JSON body for login:', jsonError);
    return NextResponse.json({ message: 'Invalid request body format. Expected JSON.' }, { status: 400 });
  }

  try {
    

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
    const refreshToken = uuidv4();

    await prisma.user.update({
      where: { id: user.id },
      data: { refreshToken },
    });

    const response = NextResponse.json({ message: 'Login successful', token, refreshToken }, { status: 200 });
    response.cookies.set('authToken', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production', // Use secure in production
      maxAge: 60 * 60 * 2, // 2 hours
      path: '/',
    });

    return response;
  } catch (error) {
console.error('Error in login POST handler:', error);
    console.error('Error during login:', error);
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}