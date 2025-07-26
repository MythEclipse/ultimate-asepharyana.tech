import { cookies } from 'next/headers';
import * as jose from 'jose';

interface DecodedToken {
  id: string;
  email: string;
  // Add other properties that you expect in your JWT payload
}

export async function verifyJwt(token: string): Promise<DecodedToken | null> {
  try {
    const secret = new TextEncoder().encode(process.env.JWT_SECRET);
    const { payload } = await jose.jwtVerify(token, secret);
    // Explicitly cast to unknown first, then to DecodedToken
    return payload as unknown as DecodedToken;
  } catch (error) {
    console.error('JWT verification failed:', error);
    return null;
  }
}

export async function getAuthenticatedUser(): Promise<DecodedToken | null> {
  const cookieStore = await cookies(); // Await the cookies() call
  const token = cookieStore.get('authToken')?.value;

  if (!token) {
    return null;
  }

  const user = await verifyJwt(token);
  return user;
}