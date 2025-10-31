import { SignJWT, jwtVerify } from 'jose';
import { config } from '../config';

export interface JWTPayload {
  user_id: string;
  email: string;
  name: string;
  exp?: number;
}

// Helper to sign JWT tokens
export async function signJWT(
  payload: Omit<JWTPayload, 'exp'>,
  expiresIn: number = 24 * 3600
): Promise<string> {
  const secret = new TextEncoder().encode(config.jwtSecret);

  const token = await new SignJWT(payload)
    .setProtectedHeader({ alg: 'HS256' })
    .setIssuedAt()
    .setExpirationTime(`${expiresIn}s`)
    .sign(secret);

  return token;
}

export async function verifyJWT(token: string): Promise<JWTPayload | null> {
  try {
    const secret = new TextEncoder().encode(config.jwtSecret);
    const { payload } = await jwtVerify(token, secret);

    return payload as unknown as JWTPayload;
  } catch {
    return null;
  }
}
