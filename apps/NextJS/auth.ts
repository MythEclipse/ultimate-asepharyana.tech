import NextAuth from "next-auth";
import GoogleProvider from "next-auth/providers/google";
import { upsertGoogleUser } from "@/lib/prisma/service";

declare module "next-auth" {
  interface Session {
    user: {
      id?: string;
      name?: string | null;
      email?: string | null;
      image?: string | null;
    };
  }
  interface Token {
    id?: string | null;
    email?: string | null;
    name?: string | null;
  }
}

export const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    GoogleProvider({
      clientId: process.env.AUTH_GOOGLE_ID,
      clientSecret: process.env.AUTH_GOOGLE_SECRET,
    }),
  ],
  session: {
    strategy: "jwt",
  },
  callbacks: {
    async jwt({ token, user, account }) {
      if (user) {
        token.id = user.id ?? null;
        token.email = user.email ?? null;
        token.name = user.name ?? null;

        if (account?.provider === "google" && user.email) {
          try {
            const upsertedUser = await upsertGoogleUser(
              user.email,
              user.name || null,
              user.image || null
            );
            token.id = upsertedUser.id ?? null;
          } catch (error) {
            console.error("Error upserting Google user in JWT callback:", error);
          }
        }
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
