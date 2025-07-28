'use client';
import React from 'react';
import { useAuth } from '@/hooks/AuthContext';

export default function SignOutComponent() {
  const { logout } = useAuth();
  
  const handleSignOut = async () => {
    await logout();
  };

  return (
    <button onClick={handleSignOut} type='button'>Sign Out</button>
  );
}
