import NextAuth from "next-auth";
import GoogleProvider from "next-auth/providers/google";
import CredentialsProvider from "next-auth/providers/credentials";
import { prisma } from "./lib/db";
import { PrismaAdapter } from "@auth/prisma-adapter";
import { compare } from "bcryptjs";

// Define a local Account interface to ensure type compatibility
interface LocalAccount {
  provider: string;
  type: string; // Use string as it's what's coming from the account object
  providerAccountId: string;
  access_token?: string | null;
  expires_at?: number | null;
  refresh_token?: string | null;
  id_token?: string | null;
  scope?: string | null;
  session_state?: string | null;
  token_type?: string | null;
}

import type { NextAuthConfig } from "next-auth";
import type { signIn as nextAuthSignIn, signOut as nextAuthSignOut } from "next-auth/react";

const nextAuthInstance = NextAuth({
  providers: [
    GoogleProvider({
      clientId: process.env.AUTH_GOOGLE_ID,
      clientSecret: process.env.AUTH_GOOGLE_SECRET,
    }),
    CredentialsProvider({
      name: "Credentials",
      credentials: {
        email: { label: "Email", type: "email", placeholder: "jsmith@example.com" },
        password: { label: "Password", type: "password" },
      },
      async authorize(credentials) {
        if (!credentials?.email || !credentials.password) {
          return null;
        }

        const user = await prisma.user.findUnique({
          where: {
            email: credentials.email as string,
          },
        });

        if (!user || !user.password) {
          return null;
        }

        const isPasswordValid = await compare(
          credentials.password as string,
          user.password
        );

        if (!isPasswordValid) {
          return null;
        }

        return {
          id: user.id,
          email: user.email,
          name: user.name,
        };
      },
    }),
  ],
  adapter: PrismaAdapter(prisma),
  session: {
    strategy: "jwt",
  },
  callbacks: {

    async signIn({ user, account, profile, email, credentials }) {
      if (account?.provider && user?.email) {
        const existingUser = await prisma.user.findUnique({
          where: { email: user.email },
          include: { accounts: true },
        });

        if (existingUser) {
          const alreadyLinked = existingUser.accounts.some(
            (acc: LocalAccount) => // Use LocalAccount here
              acc.provider === account.provider &&
              acc.providerAccountId === account.providerAccountId
          );
          if (!alreadyLinked) {
            await prisma.account.create({
              data: {
                userId: existingUser.id,
                type: account.type as string, // Explicitly cast to string
                provider: account.provider,
                providerAccountId: account.providerAccountId,
                refresh_token: account.refresh_token,
                access_token: account.access_token,
                expires_at: account.expires_at,
                token_type: account.token_type,
                scope: account.scope,
                id_token: account.id_token,
                session_state: account.session_state
                  ? String(account.session_state)
                  : null,
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

// Explicitly export destructured NextAuth instance members with type annotations
export const handlers = nextAuthInstance.handlers;
export const signIn: typeof nextAuthSignIn = nextAuthInstance.signIn;
export const signOut: typeof nextAuthSignOut = nextAuthInstance.signOut;
export const auth = nextAuthInstance.auth;
