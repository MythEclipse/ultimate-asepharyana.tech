import { NextRequest } from 'next/server';
import { prisma } from './prisma/service';

interface SessionUser {
  email?: string;
}

interface SessionJson {
  user?: SessionUser;
}

export async function getAuthenticatedUser(req: NextRequest) {
  const sessionToken = req.cookies.get('next-auth.token')?.value;

  if (!sessionToken) {
    return null;
  }

  try {
    // Use relative import for the API route
    const { GET } = await import('../app/api/auth/[...nextauth]/route');
    const session = await GET(req);

    if (session && session.body && session.status === 200) {
      const reader = session.body?.getReader();
      let sessionJson: SessionJson | null = null;
      if (reader) {
        const { value } = await reader.read();
        if (value) {
          const text = new TextDecoder().decode(value);
          sessionJson = JSON.parse(text) as SessionJson;
        }
      }
      // Remove .profile from include if not defined in Prisma schema
      if (!sessionJson?.user?.email) {
        return null;
      }
      const user = await prisma.user.findUnique({
        where: { email: sessionJson.user.email },
        // Include only existing relations from Prisma schema
        include: {
          // Ensure 'profile' exists in your Prisma schema
          // profile: true,
        },
      });
      return user;
    }
  } catch (error) {
    console.error('Error getting authenticated user:', error);
  }

  return null;
}
