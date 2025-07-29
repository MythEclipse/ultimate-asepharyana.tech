// Minimal NextAuth config for middleware compatibility (no Prisma/server-only logic)
import NextAuth from "next-auth";
import GoogleProvider from "next-auth/providers/google";
import { prisma } from "./lib/db";
import { PrismaAdapter } from "@auth/prisma-adapter"
// Only export middleware-compatible handlers and auth
export const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    GoogleProvider({
      clientId: process.env.AUTH_GOOGLE_ID,
      clientSecret: process.env.AUTH_GOOGLE_SECRET,
    }),
  ],
  adapter: PrismaAdapter(prisma),
  session: {
    strategy: "jwt",
  },
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        token.id = user.id ?? null;
        token.email = user.email ?? null;
        token.name = user.name ?? null;
        // Do NOT call Prisma or upsertGoogleUser here (middleware-safe)
      }
      return token;
    },
    async session({ session, token }) {
      if (token && session.user) {
        session.user.id = typeof token.id === "string" ? token.id : "";
        session.user.email = token.email ?? "";
        session.user.name = token.name ?? null;
      }
      return session;
    },
  },
});

