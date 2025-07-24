'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode, useCallback } from 'react';
import { useRouter } from 'next/navigation';

interface User {
  id: string;
  email: string;
  image?: string; // Add optional image property
  // Add other user properties as needed
}

interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  login: (email: string, password: string) => Promise<boolean>;
  logout: () => Promise<void>;
  refreshAccessToken: () => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  const verifyToken = useCallback(async () => {
    // In a real application, you'd verify the token with your backend or decode it
    // and check its expiration. For now, we'll assume a token means authenticated.
    const token = document.cookie.split('; ').find(row => row.startsWith('authToken='))?.split('=')[1];
    if (token) {
      // Simulate user data retrieval from token
      // For now, we'll use dummy data for image. In a real app, this would come from JWT or an API call.
      setUser({ id: 'simulated_user_id', email: 'user@example.com', image: '/user_placeholder.png' });
    } else {
      setUser(null);
    }
    setIsLoading(false);
  }, []);

  useEffect(() => {
    verifyToken();
  }, [verifyToken]);

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
        // Update user state with actual user data from response if available
        // For image, using a placeholder until a real image URL is available from the API
        setUser({ id: 'simulated_user_id', email, image: '/user_placeholder.png' });
        router.push('/'); // Redirect on successful login
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

  const logout = async (): Promise<void> => {
    setIsLoading(true);
    try {
      const response = await fetch('/api/jwt-auth/logout', {
        method: 'POST',
      });
      if (response.ok) {
        setUser(null);
        router.push('/login'); // Redirect to login page after logout
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
        // Assuming refresh returns new token and updates cookies
        console.log('Access token refreshed');
        verifyToken(); // Re-verify token to update user state if needed
        return true;
      } else {
        console.error('Failed to refresh access token');
        setUser(null); // Clear user if refresh fails (e.g., refresh token expired)
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
    <AuthContext.Provider value={{ user, isLoading, login, logout, refreshAccessToken }}>
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