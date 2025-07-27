// apps/NextJS/hooks/SettingsContext.tsx
'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

type Theme = 'light' | 'dark' | 'system';

// Define a stricter Preferences type
type Preferences = Record<string, string | number | boolean | null>;

interface SettingsContextType {
  theme: Theme;
  setTheme: (theme: Theme) => void;
  preferences: Preferences;
  setPreferences: (prefs: Preferences) => void;
}

const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export const SettingsProvider = ({ children }: { children: ReactNode }) => {
  const [theme, setThemeState] = useState<Theme>('system');
  const [preferences, setPreferencesState] = useState<Preferences>({});

  useEffect(() => {
    // Load persisted settings
    const storedTheme = typeof window !== 'undefined' ? localStorage.getItem('theme') : null;
    const storedPrefs = typeof window !== 'undefined' ? localStorage.getItem('preferences') : null;
    if (storedTheme) setThemeState(storedTheme as Theme);
    if (storedPrefs) setPreferencesState(JSON.parse(storedPrefs));
  }, []);

  const setTheme = (theme: Theme) => {
    setThemeState(theme);
    if (typeof window !== 'undefined') localStorage.setItem('theme', theme);
  };

  const setPreferences = (prefs: Preferences) => {
    setPreferencesState(prefs);
    if (typeof window !== 'undefined') localStorage.setItem('preferences', JSON.stringify(prefs));
  };

  return (
    <SettingsContext.Provider value={{ theme, setTheme, preferences, setPreferences }}>
      {children}
    </SettingsContext.Provider>
  );
};

export const useSettings = () => {
  const context = useContext(SettingsContext);
  if (context === undefined) {
    throw new Error('useSettings must be used within a SettingsProvider');
  }
  return context;
};