// src/client.ts
import { PrismaClient } from "@prisma/client";
export * from "@prisma/client";
var globalForPrisma = globalThis;
var prisma = globalForPrisma.prisma ?? new PrismaClient();
if (process.env.NODE_ENV !== "production") {
  globalForPrisma.prisma = prisma;
}
if (typeof module !== "undefined") {
  module.exports = { prisma, PrismaClient };
}
export {
  PrismaClient,
  prisma
};
