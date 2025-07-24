import { NextResponse } from 'next/server';
import { cookies } from 'next/headers';
import { prisma } from '@/lib/prisma/service'; // Added prisma import, though not used in logout

export async function POST() {
  try {
    const response = NextResponse.json({ message: 'Logout successful' }, { status: 200 });
    const cookieStore = await cookies();
    cookieStore.delete('authToken'); 
    return response;
  } catch (error) {
    console.error('Error during logout:', error);
    return NextResponse.json({ message: 'Internal server error' }, { status: 500 });
  }
}