import { NextRequest } from 'next/server';
import { prisma } from '@/lib/prisma/service';

export async function getAuthenticatedUser(req: NextRequest) {
  const sessionToken = req.cookies.get('next-auth.token')?.value;

  if (!sessionToken) {
    return null;
  }

  try {
    // Use relative import for the API route
    const { GET } = await import('../app/api/auth/[...nextauth]/route');
    const session = await GET(req, new Request(req.url));

    if (session && session.body && session.status === 200) {
      const sessionJson = await session.body.json();
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