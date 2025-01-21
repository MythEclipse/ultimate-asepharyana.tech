// lib/prisma.ts
import { PrismaClient } from '@prisma/client';

interface GlobalPrisma {
  prisma?: PrismaClient;
}

const globalForPrisma = global as GlobalPrisma;

export const prisma =
  globalForPrisma.prisma ||
  new PrismaClient({
    log: ['query'],
  });

if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma;
