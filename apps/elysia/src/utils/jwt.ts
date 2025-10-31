import jwt from '@elysiajs/jwt';

export interface JWTPayload {
  user_id: string;
  email: string;
  name: string;
  exp: number;
}

export function createJWT(app: any) {
  return jwt({
    name: 'jwt',
    secret: process.env.JWT_SECRET || 'default_secret_change_this',
  });
}

export function generateToken(payload: Omit<JWTPayload, 'exp'>, expiresIn: number = 24 * 3600): string {
  const exp = Math.floor(Date.now() / 1000) + expiresIn;
  const jwtPayload: JWTPayload = {
    ...payload,
    exp,
  };

  // Note: In Elysia, we'll use the JWT plugin's sign method
  // This is just for type reference
  return JSON.stringify(jwtPayload);
}

export async function verifyToken(token: string, jwtInstance: any): Promise<JWTPayload | null> {
  try {
    const payload = await jwtInstance.verify(token);
    if (!payload) {
      return null;
    }

    // Check if token is expired
    if (payload.exp && payload.exp < Math.floor(Date.now() / 1000)) {
      return null;
    }

    return payload as JWTPayload;
  } catch (error) {
    return null;
  }
}
