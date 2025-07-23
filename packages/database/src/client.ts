// File: database/index.ts
import { Prisma, PrismaClient } from "@prisma/client";


const globalForPrisma = globalThis as unknown as { prisma?: PrismaClient };
export const prisma = globalForPrisma.prisma ?? new PrismaClient();
if (process.env.NODE_ENV !== "production") {
  globalForPrisma.prisma = prisma;
}
export { PrismaClient, Prisma };
export type { Posts, User, Likes, Comments } from '@prisma/client';
