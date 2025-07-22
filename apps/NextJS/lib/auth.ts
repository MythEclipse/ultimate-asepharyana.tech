import NextAuth from 'next-auth';
import { NextResponse } from 'next/server';
import Google from 'next-auth/providers/google';
import { PrismaAdapter } from '@auth/prisma-adapter';
import { prisma } from '@asepharyana/database';

export const { auth, handlers, signIn, signOut } = NextAuth({
  adapter: PrismaAdapter(prisma),
  providers: [Google],
  secret: process.env.NEXTAUTH_SECRET,
  
  callbacks: {
    authorized: async ({ auth, request }) => {
      

      if (auth) {
        return true;
      } else {
        request.nextUrl.searchParams.set('callbackUrl', request.nextUrl.href);
        request.nextUrl.pathname = `/login`;
        return NextResponse.redirect(request.nextUrl);
      }
    },
    session: async ({ session, user }) => {
      if (session?.user && user) {
        session.user.id = user.id;
      }
      return session;
    },
  },
});

