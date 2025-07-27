import { PrismaClient } from '@asepharyana/database';

const globalForPrisma = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  globalForPrisma.prisma ||
  new PrismaClient({
    log:
      process.env.NODE_ENV === 'development'
        ? ['query', 'error', 'warn']
        : ['error'],
  });

if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma;

export const updateUserImage = async (id: string, image: string) => {
  try {
    const user = await prisma.user.update({
      where: {
        id,
      },
      data: {
        image,
      },
    });
    return user;
  } catch (error) {
    console.error('Error updating user image:', error);
    throw new Error('Failed to update user image');
  }
};
