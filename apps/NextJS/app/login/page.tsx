'use client';
import Link from 'next/link';
import { FcGoogle } from 'react-icons/fc';
import React, { Suspense, useState, memo, useRef, useEffect } from 'react';
import { useAuth } from '@/hooks/AuthContext';

function LoginButton() {
  const { login } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const errorRef = useRef<HTMLParagraphElement>(null);

  useEffect(() => {
    if (error && errorRef.current) {
      errorRef.current.focus();
    }
  }, [error]);

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    const success = await login(email, password);
    if (!success) {
      setError('Login failed. Please check your credentials.');
    }
  };

  return (
    <form onSubmit={handleLogin} className="space-y-4" role="form" aria-label="Login form">
      {error && (
        <p
          className="text-red-500 text-center"
          id="login-error"
          tabIndex={-1}
          ref={errorRef}
          aria-live="polite"
        >
          {error}
        </p>
      )}
      <div>
        <label htmlFor="email" className="block text-sm font-medium text-gray-700">Email</label>
        <input
          type="email"
          id="email"
          name="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
          aria-describedby={error ? 'login-error' : undefined}
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
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
          aria-describedby={error ? 'login-error' : undefined}
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
        />
      </div>
      <button
        type="submit"
        className='w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500'
        aria-label="Sign in"
      >
        Sign in
      </button>
      {/* Google Sign-in button (optional, if you still want to support it or for later removal) */}
      <button
        type="button"
        onClick={() => { /* Implement Google login via your API or remove if not needed */ }}
        className='w-full flex items-center justify-center gap-3 px-6 py-3 text-xl text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors'
        aria-label="Sign in with Google"
      >
        <FcGoogle className='text-2xl' aria-hidden="true" />
        Sign in with Google (Placeholder)
      </button>
      <div className="text-center mt-4">
        <Link href="/register" className="text-blue-600 hover:underline">
          Don&apos;t have an account? Register here.
        </Link>
      </div>
    </form>
  );
}

const MemoizedLoginButton = memo(LoginButton);

export default function SignIn() {
  return (
    <div className='min-h-screen flex flex-col justify-center items-center'>
      <div className='p-10 rounded-lg shadow-lg'>
        <h1 className='mb-6 text-3xl font-bold text-center text-blue-600'>
          Welcome
        </h1>
        <Suspense fallback={<div className='text-blue-500'>Loading...</div>}>
          <MemoizedLoginButton />
        </Suspense>
      </div>
    </div>
  );
}
