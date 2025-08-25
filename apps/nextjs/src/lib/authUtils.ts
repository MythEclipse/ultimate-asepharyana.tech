import { NextRequest } from 'next/server';
import { prisma } from './prisma/service';

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
      let sessionJson = null;
      if (reader) {
        const { value } = await reader.read();
        if (value) {
          const text = new TextDecoder().decode(value);
          sessionJson = JSON.parse(text);
        }
      }
      // Remove .profile from include if not defined in Prisma schema
      const user = await prisma.user.findUnique({
        where: { email: sessionJson.user?.email },
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
