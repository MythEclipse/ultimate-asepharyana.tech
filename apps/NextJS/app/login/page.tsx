'use client';
import { FcGoogle } from 'react-icons/fc';
import React, { Suspense, useState } from 'react';
import { useAuth } from '@/hooks/AuthContext';

// 

function LoginButton() {
  const { login } = useAuth();
  // // const router = useRouter();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    const success = await login(email, password);
    if (!success) {
      setError('Login failed. Please check your credentials.');
    }
  };

  return (
    <form onSubmit={handleLogin} className="space-y-4">
      {error && <p className="text-red-500 text-center">{error}</p>}
      <div>
        <label htmlFor="email" className="block text-sm font-medium text-gray-700">Email</label>
        <input
          type="email"
          id="email"
          name="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm text-black"
        />
      </div>
      <div>
        <label htmlFor="password" className="block text-sm font-medium text-gray-700">Password</label>
        <input
          type="password"
          id="password"
          name="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm text-black"
        />
      </div>
      <button
        type="submit"
        className='w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500'
      >
        Sign in
      </button>
      {/* Google Sign-in button (optional, if you still want to support it or for later removal) */}
      <button
        type="button"
        onClick={() => { /* Implement Google login via your API or remove if not needed */ }}
        className='w-full flex items-center justify-center gap-3 px-6 py-3 text-xl text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors'
      >
        <FcGoogle className='text-2xl' />
        Sign in with Google (Placeholder)
      </button>
    </form>
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
