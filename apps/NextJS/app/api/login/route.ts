import { NextResponse } from 'next/server';
import { getDb } from '@asepharyana/services';
import bcrypt from 'bcrypt';
import { signJwt } from '../../../lib/jwt';

export async function POST(request: Request) {
  const db = getDb();
  try {
    const { email, password } = await request.json();

    if (!email || !password) {
      return NextResponse.json({ message: 'Missing email or password' }, { status: 400 });
    }

    const user = await db.selectFrom('User')
      .selectAll()
      .where('email', '=', email)
      .executeTakeFirst() as any | undefined;

    if (!user || !user.password) {
      return NextResponse.json({ message: 'Invalid credentials' }, { status: 401 });
    }

    const isPasswordValid = await bcrypt.compare(password, user.password);

    if (!isPasswordValid) {
      return NextResponse.json({ message: 'Invalid credentials' }, { status: 401 });
    }

    const token = await signJwt({ userId: user.id, email: user.email, name: user.name }, '1h');

    const response = NextResponse.json({ message: 'Login successful', user: { id: user.id, name: user.name, email: user.email } }, { status: 200 });
    response.cookies.set('token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      path: '/',
    });

    return response;
  } catch (error) {
    console.error('Login error:', error);
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}
