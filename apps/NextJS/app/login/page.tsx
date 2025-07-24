'use client';
import { signIn } from 'next-auth/react';
import { FcGoogle } from 'react-icons/fc';
import React, { Suspense } from 'react';

function LoginButton() {
  return (
    <button
      onClick={() => signIn('google', { redirectTo: '/' })}
      className='flex items-center gap-3 px-6 py-3 text-xl text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors'
    >
      <FcGoogle className='text-2xl' />
      Sign in with Google
    </button>
  );
}

export default function SignIn() {
  return (
    <div className='min-h-screen flex flex-col justify-center items-center'>
      <div className='p-10 rounded-lg shadow-lg'>
        <h1 className='mb-6 text-3xl font-bold text-center text-blue-600'>
          Welcome
        </h1>
        <Suspense fallback={<div className='text-blue-500'>Loading...</div>}>
          <LoginButton />
        </Suspense>
      </div>
    </div>
  );
}
