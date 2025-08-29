import { NextRequest, NextResponse } from 'next/server';
import { verifyJwt } from './jwt';
import { getDb, User } from '@asepharyana/services';

interface AuthorizedUser {
  id: string;
  email: string | null;
  name?: string | null;
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

  const db = getDb();

  // Fetch user roles and permissions from the database
  const user = await db
    .selectFrom('User')
    .selectAll()
    .where('User.id', '=', decoded.userId as string)
    .leftJoin('UserRole', 'UserRole.userId', 'User.id')
    .leftJoin('Role', 'Role.id', 'UserRole.roleId')
    .leftJoin('RolePermission', 'RolePermission.roleId', 'Role.id')
    .leftJoin('Permission', 'Permission.id', 'RolePermission.permissionId')
    .select([
      'User.id',
      'User.email',
      'User.name',
      db.fn.agg('GROUP_CONCAT', ['Role.name']).as('roles'),
      db.fn.agg('GROUP_CONCAT', ['Permission.name']).as('permissions'),
    ])
    .groupBy('User.id')
    .executeTakeFirst();

  if (!user) {
    return null;
  }

  const roles = user.roles ? (user.roles as string).split(',') : [];
  const permissions = user.permissions
    ? (user.permissions as string).split(',')
    : [];

  return {
    id: user.id ?? "",
    email: user.email,
    name: user.name,
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
