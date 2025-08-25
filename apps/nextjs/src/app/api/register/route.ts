// Wrapped POST handler with withLogging for centralized logging
// Fixed: Always return NextResponse for compatibility with withLogging

import { NextResponse, NextRequest } from 'next/server';
import { hash } from 'bcryptjs';
import { prisma } from '../../../lib/db';
import { ratelimit } from '../../../lib/ratelimit';
import { withLogging } from '../../../lib/api-wrapper';

async function handler(request: NextRequest) {
  try {
    const ip = request.headers.get('x-forwarded-for') ?? '127.0.0.1';
    const { success, limit, reset, remaining } = await ratelimit.limit(ip);

    if (!success) {
      return NextResponse.json(
        { message: 'Too many requests' },
        {
          status: 429,
          headers: {
            'X-RateLimit-Limit': limit.toString(),
            'X-RateLimit-Remaining': remaining.toString(),
            'X-RateLimit-Reset': reset.toString(),
          },
        }
      );
    }

    const { name, email, password } = await request.json();

    // Validasi input dasar
    if (!name || !email || !password) {
      return NextResponse.json({ message: 'Nama, email, dan kata sandi harus diisi.' }, { status: 400 });
    }

    if (password.length < 6) {
      return NextResponse.json({ message: 'Kata sandi minimal harus 6 karakter.' }, { status: 400 });
    }

    // Cek apakah pengguna sudah ada
    const existingUser = await prisma.user.findUnique({
      where: { email: email },
    });

    if (existingUser) {
      return NextResponse.json({ message: 'Email sudah terdaftar.' }, { status: 409 }); // 409 Conflict
    }

    // Hash kata sandi sebelum disimpan
    const hashedPassword = await hash(password, 10);

    // Buat pengguna baru di database
    await prisma.user.create({
      data: {
        name,
        email,
        password: hashedPassword,
      },
    });

    return NextResponse.json({ message: 'Registrasi berhasil!' }, { status: 201 });
  } catch (error) {
    console.error('REGISTRATION_ERROR', error);
    return NextResponse.json({ message: 'Terjadi kesalahan pada server.' }, { status: 500 });
  }
}

export const POST = (request: NextRequest) => withLogging(handler)(request, { params: {} });
