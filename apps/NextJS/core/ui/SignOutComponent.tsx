'use client';
import React from 'react';
import { signOut } from "next-auth/react"
export default function SignOutComponent() {

  return (
    <button onClick={() => signOut()} type='button'>Sign Out</button>
  );
}
