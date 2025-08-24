// File: database/index.ts
import { Prisma, PrismaClient } from "./generated/client/client.js";


const globalForPrisma = globalThis as unknown as { prisma?: PrismaClient };
export const prisma = globalForPrisma.prisma ?? new PrismaClient();
if (process.env.NODE_ENV !== "production") {
  globalForPrisma.prisma = prisma;
}
export { PrismaClient, Prisma };
export type Posts = Prisma.PostsGetPayload<object>;
export type User = Prisma.UserGetPayload<object>;
export type Likes = Prisma.LikesGetPayload<object>;
export type Comments = Prisma.CommentsGetPayload<object>;
export type ChatMessage = Prisma.ChatMessageGetPayload<object>;
export type Account = Prisma.AccountGetPayload<object>;
export type Session = Prisma.SessionGetPayload<object>;
export type Replies = Prisma.RepliesGetPayload<object>;
