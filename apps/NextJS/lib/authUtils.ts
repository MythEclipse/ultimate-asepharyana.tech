import { cookies, headers } from 'next/headers';
import * as jose from 'jose';

export interface DecodedToken {
  id: string;
  email: string;
  fullname?: string;
  role?: string;
  exp?: number;
  iat?: number;
  [key: string]: unknown; // Allow all JWT payload properties
}

export async function verifyJwt(token: string): Promise<DecodedToken | null> {
  try {
    const secret = new TextEncoder().encode(process.env.JWT_SECRET);
    const { payload } = await jose.jwtVerify(token, secret);
    return payload as DecodedToken;
  } catch (error) {
    console.error('JWT verification failed:', error);
    return null;
  }
}

export async function getAuthenticatedUser(): Promise<DecodedToken | null> {
  const headersList = await headers();
  const authHeader = headersList.get('authorization');
  let token: string | undefined;

  if (authHeader && authHeader.startsWith('Bearer ')) {
    token = authHeader.substring(7);
  } else {
    const cookieStore = await cookies();
    token = cookieStore.get('authToken')?.value;
  }

  if (!token) {
    return null;
  }

  const user = await verifyJwt(token);
  return user;
}