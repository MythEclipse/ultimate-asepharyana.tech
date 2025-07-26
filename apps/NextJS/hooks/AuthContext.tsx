'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { useRouter } from 'next/navigation';
import { User } from '@asepharyana/database';



interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  refreshAccessToken: () => Promise<boolean>;
  register: (name: string, email: string, password: string) => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null);
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
        const profileResponse = await fetch('/api/jwt-auth/profile', {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });
        if (profileResponse.ok) {
          const profile = await profileResponse.json();
          setUser(profile.user);
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

        setUser(data.user);
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