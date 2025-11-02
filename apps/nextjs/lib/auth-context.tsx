'use client';

import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { UnifiedHttpClient } from '../utils/unified-http-client';
import type { User, LoginCredentials, RegisterData } from '../types/auth';

interface AuthContextType {
  user: User | null;
  loading: boolean;
  login: (credentials: LoginCredentials) => Promise<void>;
  register: (data: RegisterData) => Promise<void>;
  logout: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

const client = UnifiedHttpClient.createClientSide();

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  const refreshUser = async () => {
    try {
      const data = await client.fetchJson<{ user: User | null }>(
        `/api/auth/verify`,
        { credentials: 'include' }
      );
      setUser(data.user ?? null);
    } catch (error) {
      console.error('Failed to verify session:', error);
      setUser(null);
    }
  };

  useEffect(() => {
    refreshUser().finally(() => setLoading(false));
  }, []);

  const login = async (credentials: LoginCredentials) => {
    const data = await client.request<{ user: User }>(
      `/api/auth/login`,
      'POST',
      credentials,
      undefined,
      { credentials: 'include' }
    );
    setUser(data.user);
  };

  const register = async (data: RegisterData) => {
    await client.request(
      `/api/auth/register`,
      'POST',
      data,
      undefined,
      { credentials: 'include' }
    );
    // After registration, login automatically
    await login({ email: data.email, password: data.password });
  };

  const logout = async () => {
    try {
      await client.request(`/api/auth/logout`, 'POST', undefined, undefined, {
        credentials: 'include',
      });
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      setUser(null);
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        loading,
        login,
        register,
        logout,
        refreshUser,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
