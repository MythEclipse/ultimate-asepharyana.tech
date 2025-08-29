// apps/nextjs/lib/jwt.ts
import jwt, { JwtPayload } from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET || 'default_secret';

export function signJwt(payload: object, options?: jwt.SignOptions) {
  return jwt.sign(payload, SECRET, options);
}

export function verifyJwt(token: string): JwtPayload | null {
  try {
    return jwt.verify(token, SECRET) as JwtPayload;
  } catch (e) {
    return null;
  }
}
