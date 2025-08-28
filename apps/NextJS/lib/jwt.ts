import { SignJWT, jwtVerify } from 'jose';

const secretKey = process.env.JWT_SECRET_KEY;
const encodedKey = new TextEncoder().encode(secretKey);

export async function signJwt(payload: Record<string, unknown>, expiresIn: string) {
  return new SignJWT(payload)
    .setProtectedHeader({ alg: 'HS256' })
    .setIssuedAt()
    .setExpirationTime(expiresIn)
    .sign(encodedKey);
}

export async function verifyJwt(token: string) {
  try {
    const { payload } = await jwtVerify(token, encodedKey);
    return payload;
  } catch (error) {
    console.error('JWT verification failed:', error);
    return null;
  }
}
