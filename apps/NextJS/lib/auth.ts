import { NextRequest, NextResponse } from 'next/server';
import { verifyJwt } from './jwt';
import { prisma } from './prisma/service'; // Assuming prisma is accessible here

interface AuthorizedUser {
  id: string;
  email: string;
  name?: string;
  roles?: string[];
  permissions?: string[];
}

// Utility to verify JWT and return user ID and other claims
export async function verifyToken(
  req: NextRequest,
): Promise<AuthorizedUser | null> {
  const token = req.cookies.get('token')?.value;
  if (!token) {
    return null;
  }
  const decoded = await verifyJwt(token);
  if (!decoded || !decoded.userId) {
    return null;
  }

  // Fetch user roles and permissions from the database
  const user = await prisma.user.findUnique({
    where: { id: decoded.userId as string },
    include: {
      roles: {
        include: {
          role: {
            include: {
              permissions: {
                include: {
                  permission: true,
                },
              },
            },
          },
        },
      },
    },
  });

  if (!user) {
    return null;
  }

  const roles = user.roles.map((ur) => ur.role.name);
  const permissions = Array.from(
    new Set(
      user.roles.flatMap((ur) =>
        ur.role.permissions.map((rp) => rp.permission.name),
      ),
    ),
  );

  return {
    id: user.id,
    email: user.email,
    name: user.name || undefined,
    roles,
    permissions,
  };
}

// Utility to authorize requests based on roles/permissions
export async function authorizeRequest(
  req: NextRequest,
  requiredRoles: string[] = [],
  requiredPermissions: string[] = [],
): Promise<NextResponse | null> {
  const user = await verifyToken(req);

  if (!user) {
    return NextResponse.json(
      { message: 'Authentication required' },
      { status: 401 },
    );
  }

  // Check roles
  if (requiredRoles.length > 0) {
    const hasRequiredRole = requiredRoles.some((role) =>
      user.roles?.includes(role),
    );
    if (!hasRequiredRole) {
      return NextResponse.json(
        { message: 'Forbidden: Insufficient role' },
        { status: 403 },
      );
    }
  }

  // Check permissions
  if (requiredPermissions.length > 0) {
    const hasRequiredPermission = requiredPermissions.some((permission) =>
      user.permissions?.includes(permission),
    );
    if (!hasRequiredPermission) {
      return NextResponse.json(
        { message: 'Forbidden: Insufficient permission' },
        { status: 403 },
      );
    }
  }

  // If authorized, return null to indicate continuation
  return null;
}
