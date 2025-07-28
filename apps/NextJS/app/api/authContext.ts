// apps/NextJS/app/api/authContext.ts

import { jwtVerify, JWTPayload } from 'jose';

const JWT_SECRET = process.env.JWT_SECRET || 'default_secret';

export interface AuthResult {
  valid: boolean;
  payload?: JWTPayload;
  status: 'loading' | 'authenticated' | 'unauthenticated';
  error?: string;
}

// Fungsi mirip useSession untuk API/SSR
export async function getSessionFromRequest(req: Request): Promise<AuthResult> {
  const authHeader = req.headers.get('authorization');
  const token = authHeader?.replace('Bearer ', '');
  if (!token) {
    return { valid: false, status: 'unauthenticated', error: 'Unauthorized' };
  }
  try {
    const secret = new TextEncoder().encode(JWT_SECRET);
    const { payload } = await jwtVerify(token, secret);
    if (payload.exp && Date.now() >= payload.exp * 1000) {
      return { valid: false, status: 'unauthenticated', error: 'Token expired' };
    }
    return { valid: true, payload, status: 'authenticated' };
  } catch (err) {
    return { valid: false, status: 'unauthenticated', error: (err as Error).message };
  }
}

export async function verifyJWT(token: string): Promise<AuthResult> {
  try {
    const secret = new TextEncoder().encode(JWT_SECRET);
    const { payload } = await jwtVerify(token, secret);
    if (payload.exp && Date.now() >= payload.exp * 1000) {
      return { valid: false, status: 'unauthenticated', error: 'Token expired' };
    }
    return { valid: true, payload, status: 'authenticated' };
  } catch (err) {
    return { valid: false, status: 'unauthenticated', error: (err as Error).message };
  }
}