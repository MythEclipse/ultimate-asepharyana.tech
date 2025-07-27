/**
 * AuthContext provides authentication state and actions for the app.
 *
 * ## State Managed:
 * - user: The current authenticated user (ClientUser or null)
 * - isLoading: Whether authentication state is being resolved
 *
 * ## Actions:
 * - login(email, password): Authenticates user, stores tokens, fetches user info, updates state, redirects to home
 * - logout(): Clears tokens, resets user, redirects to login
 * - register(name, email, password): Registers a new user
 * - refreshAccessToken(): Refreshes the access token, handles session expiry
 *
 * ## Effects:
 * - On mount, verifies token from localStorage and fetches user profile
 *
 * ## Usage:
 * Wrap your app with <AuthProvider>. Use the useAuth() hook to access state and actions.
 *
 * ## Example:
 * ```tsx
 * import { AuthProvider, useAuth } from '@/hooks/AuthContext';
 * 
 * function MyComponent() {
 *   const { user, login, logout, isLoading } = useAuth();
 *   // ...
 * }
 * ```
 */
'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { useRouter } from 'next/navigation';
import { ClientUser } from '../types/index'; // Import ClientUser from '@/types'

interface AuthContextType {
  user: ClientUser | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  refreshAccessToken: () => Promise<boolean>;
  register: (name: string, email: string, password: string) => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<ClientUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    // Only run on client
    const verifyToken = async () => {
      setIsLoading(true);
      let token: string | null = null;
      if (typeof window !== 'undefined') {
        token = localStorage.getItem('token');
        // DEBUG: Log token and profile fetch result
        console.log('AuthContext verifyToken: token from localStorage', token);
      }
      if (token) {
        // Use new /api/jwt-auth/me endpoint for secure user info
        const meResponse = await fetch('/api/jwt-auth/me', {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });
        if (meResponse.ok) {
          const { user: userPayload } = await meResponse.json();
          setUser(userPayload as ClientUser);
        } else {
          setUser(null);
        }
      } else {
        setUser(null);
      }
      setIsLoading(false);
    };
    verifyToken();
  }, []);

  const login = async (email: string, password: string): Promise<boolean> => {
    setIsLoading(true);
    try {
      const response = await fetch('/api/jwt-auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
      });

      const data = await response.json();

      if (response.ok) {
        if (data.token) localStorage.setItem('token', data.token);
        if (data.refreshToken) localStorage.setItem('refreshToken', data.refreshToken);

        // Use /api/jwt-auth/me for user info after login
        const meResponse = await fetch('/api/jwt-auth/me', {
          headers: {
            Authorization: `Bearer ${data.token}`,
          },
        });
        if (meResponse.ok) {
          const { user: userPayload } = await meResponse.json();
          setUser(userPayload as ClientUser);
        } else {
          setUser(null);
        }
        router.push('/');
        return true;
      } else {
        console.error('Login failed:', data.message);
        return false;
      }
    } catch (error) {
      console.error('Login error:', error);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const register = async (name: string, email: string, password: string): Promise<boolean> => {
    setIsLoading(true);
    try {
      const response = await fetch('/api/jwt-auth/register', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ name, email, password }),
      });

      const data = await response.json();

      if (response.ok) {
        console.log('Registration successful:', data.message);
        return true;
      } else {
        console.error('Registration failed:', data.message);
        return false;
      }
    } catch (error) {
      console.error('Registration error:', error);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    setIsLoading(true);
    try {
      const response = await fetch('/api/jwt-auth/logout', {
        method: 'POST',
      });
      if (response.ok) {
        setUser(null);
        localStorage.removeItem('token');
        localStorage.removeItem('refreshToken');
        router.push('/login');
      } else {
        console.error('Logout failed');
      }
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const refreshAccessToken = async (): Promise<boolean> => {
    try {
      const response = await fetch('/api/jwt-auth/refresh', {
        method: 'POST',
      });
      if (response.ok) {
        console.log('Access token refreshed');
        // Optionally re-verify token here
        return true;
      } else {
        console.error('Failed to refresh access token');
        setUser(null);
        router.push('/login');
        return false;
      }
    } catch (error) {
      console.error('Refresh token error:', error);
      setUser(null);
      router.push('/login');
      return false;
    }
  };

  return (
    <AuthContext.Provider value={{ user, isLoading, login, logout, refreshAccessToken, register }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};