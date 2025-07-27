// apps/NextJS/hooks/useGlobalStore.ts
import { create } from 'zustand';
import { persist, devtools } from 'zustand/middleware';

type SosmedUIState = {
  showComments: Record<string, boolean>;
  setShowComments: (postId: string, value: boolean) => void;
  newComments: Record<string, string>;
  setNewComment: (postId: string, value: string) => void;
};

type NavbarState = {
  isMobileNavOpen: boolean;
  setMobileNavOpen: (open: boolean) => void;
};

type Preferences = {
  [key: string]: string | number | boolean | undefined;
};

type GlobalState = SosmedUIState & NavbarState & {
  preferences: Preferences;
  setPreferences: (prefs: Preferences) => void;
};

export const useGlobalStore = create<GlobalState>()(
  devtools(
    persist(
      (set) => ({
        // Sosmed UI state
        showComments: {},
        setShowComments: (postId: string, value: boolean) =>
          set((state) => ({
            showComments: { ...state.showComments, [postId]: value },
          })),
        newComments: {},
        setNewComment: (postId: string, value: string) =>
          set((state) => ({
            newComments: { ...state.newComments, [postId]: value },
          })),
        // Navbar state
        isMobileNavOpen: false,
        setMobileNavOpen: (open: boolean) => set(() => ({ isMobileNavOpen: open })),
        // Preferences
        preferences: {},
        setPreferences: (prefs: Preferences) => set(() => ({ preferences: prefs })),
      }),
      {
        name: 'global-store',
        partialize: (state) => ({
          showComments: state.showComments,
          newComments: state.newComments,
          preferences: state.preferences,
        }),
      }
    ),
    { name: 'GlobalStore' }
  )
);