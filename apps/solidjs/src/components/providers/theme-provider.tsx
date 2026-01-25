import {
  createSignal,
  createContext,
  useContext,
  type ParentProps,
  onMount,
  createEffect,
} from 'solid-js';

type Theme = 'light' | 'dark' | 'system';

interface ThemeContextValue {
  theme: () => Theme;
  setTheme: (theme: Theme) => void;
  resolvedTheme: () => 'light' | 'dark';
}

const ThemeContext = createContext<ThemeContextValue>();

export function ThemeProvider(props: ParentProps) {
  const [theme, setTheme] = createSignal<Theme>('system');
  const [resolvedTheme, setResolvedTheme] = createSignal<'light' | 'dark'>(
    'light',
  );

  const getSystemTheme = (): 'light' | 'dark' => {
    if (typeof window === 'undefined') return 'light';
    return window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light';
  };

  const applyTheme = (newTheme: Theme) => {
    if (typeof document === 'undefined') return;

    const resolved = newTheme === 'system' ? getSystemTheme() : newTheme;
    setResolvedTheme(resolved);

    document.documentElement.classList.remove('light', 'dark');
    document.documentElement.classList.add(resolved);

    localStorage.setItem('theme', newTheme);
  };

  onMount(() => {
    const stored = localStorage.getItem('theme') as Theme | null;
    const initial = stored || 'system';
    setTheme(initial);
    applyTheme(initial);

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = () => {
      if (theme() === 'system') {
        applyTheme('system');
      }
    };
    mediaQuery.addEventListener('change', handleChange);
  });

  createEffect(() => {
    applyTheme(theme());
  });

  const value: ThemeContextValue = {
    theme,
    setTheme: (newTheme: Theme) => {
      setTheme(newTheme);
      applyTheme(newTheme);
    },
    resolvedTheme,
  };

  return (
    <ThemeContext.Provider value={value}>
      {props.children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}
