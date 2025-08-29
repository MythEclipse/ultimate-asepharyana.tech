import { NextResponse } from 'next/server';
import { getDb } from '@asepharyana/services';
import bcrypt from 'bcrypt';
import { signJwt } from '../../../lib/jwt';

export async function POST(request: Request) {
  const db = getDb();
  try {
    const { name, email, password } = await request.json();

    if (!name || !email || !password) {
      return NextResponse.json(
        { message: 'Missing required fields' },
        { status: 400 },
      );
    }

    const existingUser = (await db
      .selectFrom('User')
      .selectAll()
      .where('email', '=', email)
      .executeTakeFirst()) as unknown as
      | { id: string; email: string; name: string }
      | undefined;

    if (existingUser) {
      return NextResponse.json(
        { message: 'User with this email already exists' },
        { status: 409 },
      );
    }

    const hashedPassword = await bcrypt.hash(password, 10);

    const newUser = (await db
      .insertInto('User')
      .values({
        name,
        email,
        password: hashedPassword,
        role: 'member', // Assuming a default role for new users
      })
      .returningAll()
      .executeTakeFirstOrThrow()) as unknown as {
      id: string;
      email: string;
      name: string;
    };

    const token = await signJwt(
      { userId: newUser.id, email: newUser.email, name: newUser.name },
      { expiresIn: '1h' },
    );

    const response = NextResponse.json(
      {
        message: 'User registered successfully',
        user: { id: newUser.id, name: newUser.name, email: newUser.email },
      },
      { status: 201 },
    );
    response.cookies.set('token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      path: '/',
    });

    return response;
  } catch (error) {
    console.error('Registration error:', error);
    return NextResponse.json(
      { message: 'Internal server error' },
      { status: 500 },
    );
  }
}
