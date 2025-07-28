'use client';
import React from 'react';

// import { useAuth } from '@/hooks/AuthContext';
import { FcGoogle } from 'react-icons/fc'; // Assuming you still want a Google button as a placeholder

function LoginButton() {
  // // // const { login } = useAuth();
  // Simplified for now, actual login logic will be in the login page
  const handleSignIn = async () => {
    // You might still keep Google sign-in logic if needed, but it should
    // redirect to your login API endpoint rather than next-auth's signIn.
    console.log('Google Sign-in placeholder clicked.');
    // Example: redirect to a generic login page or handle with your own Google OAuth
  };

  return (
    <button
      onClick={handleSignIn}
      className='flex items-center gap-3 px-6 py-3 text-xl text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors'
    >
      <FcGoogle className='text-2xl' />
      Sign in with Google (Placeholder)
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
        {/* Removed Suspense as it's not strictly necessary here and can cause issues with client components */}
        <LoginButton />
      </div>
    </div>
  );
}
