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
    async signIn({ user, account, profile, email, credentials }) {
      // Automatic account linking for users with the same email
      if (account?.provider && user?.email) {
        const existingUser = await prisma.user.findUnique({
          where: { email: user.email },
          include: { accounts: true },
        });

        if (existingUser) {
          const alreadyLinked = existingUser.accounts.some(
            (acc) => acc.provider === account.provider && acc.providerAccountId === account.providerAccountId
          );
          if (!alreadyLinked) {
            // Link the new OAuth account to the existing user
            await prisma.account.create({
              data: {
                userId: existingUser.id,
                type: account.type,
                provider: account.provider,
                providerAccountId: account.providerAccountId,
                refresh_token: account.refresh_token,
                access_token: account.access_token,
                expires_at: account.expires_at,
                token_type: account.token_type,
                scope: account.scope,
                id_token: account.id_token,
                session_state: account.session_state ? String(account.session_state) : null,
              },
            });
          }
        }
      }
      return true;
    },
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
