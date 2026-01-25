import { Title } from '@solidjs/meta';
import { A, useNavigate } from '@solidjs/router';
import { createEffect, For, Show } from 'solid-js';
import { Motion } from 'solid-motionone';
import { useAuth } from '~/lib/auth-context';

const stats = [
  {
    title: 'Anime Bookmarks',
    value: '0',
    icon: (
      <svg
        class="w-6 h-6"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18"
        />
      </svg>
    ),
    gradient: 'from-blue-500 to-cyan-500',
    bgGlow: 'blue',
  },
  {
    title: 'Komik Bookmarks',
    value: '0',
    icon: (
      <svg
        class="w-6 h-6"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
        />
      </svg>
    ),
    gradient: 'from-orange-500 to-red-500',
    bgGlow: 'orange',
  },
  {
    title: 'Chat History',
    value: '0',
    icon: (
      <svg
        class="w-6 h-6"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
        />
      </svg>
    ),
    gradient: 'from-green-500 to-emerald-500',
    bgGlow: 'green',
  },
];

const quickLinks = [
  {
    href: '/anime',
    icon: '📺',
    label: 'Watch Anime',
    gradient: 'from-blue-500 via-purple-500 to-pink-500',
  },
  {
    href: '/komik',
    icon: '📖',
    label: 'Read Komik',
    gradient: 'from-orange-500 via-red-500 to-pink-500',
  },
  {
    href: '/chat',
    icon: '🤖',
    label: 'AI Chat',
    gradient: 'from-green-500 via-teal-500 to-cyan-500',
  },
  {
    href: '/project',
    icon: '💼',
    label: 'Projects',
    gradient: 'from-pink-500 via-rose-500 to-red-500',
  },
];

export default function DashboardPage() {
  const navigate = useNavigate();
  const { user, loading, logout } = useAuth();

  createEffect(() => {
    if (!loading() && !user()) {
      navigate('/login');
    }
  });

  const handleLogout = async () => {
    await logout();
    navigate('/');
  };

  const getGreeting = () => {
    const hour = new Date().getHours();
    if (hour < 12) return 'Good morning';
    if (hour < 18) return 'Good afternoon';
    return 'Good evening';
  };

  return (
    <>
      <Title>Dashboard | Asepharyana</Title>
      <Show when={loading()}>
        <div class="min-h-screen flex items-center justify-center">
          <div class="relative">
            <div class="w-16 h-16 rounded-full border-4 border-primary/20 border-t-primary animate-spin" />
            <div class="absolute inset-0 w-16 h-16 rounded-full bg-primary/10 animate-ping" />
          </div>
        </div>
      </Show>

      <Show when={!loading() && user()}>
        <main class="min-h-screen bg-background text-foreground p-4 md:p-8 relative overflow-hidden">
          {/* Animated background orbs */}
          <div class="fixed inset-0 overflow-hidden pointer-events-none">
            <div class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-br from-primary/20 to-purple-500/20 rounded-full blur-3xl animate-float" />
            <div
              class="absolute -bottom-40 -left-40 w-80 h-80 bg-gradient-to-tr from-cyan-500/20 to-blue-500/20 rounded-full blur-3xl animate-float"
              style={{ 'animation-delay': '-3s' }}
            />
          </div>

          <div class="max-w-6xl mx-auto relative z-10">
            {/* Header */}
            <Motion.div
              initial={{ opacity: 0, y: -20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5 }}
              class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8"
            >
              <div>
                <p class="text-sm text-muted-foreground mb-1">
                  {getGreeting()}
                </p>
                <h1 class="text-3xl md:text-4xl font-bold gradient-text">
                  Welcome back, {user()?.name?.split(' ')[0]}!
                </h1>
              </div>
              <div class="flex gap-3">
                <A
                  href="/settings"
                  class="px-4 py-2 rounded-xl glass-subtle hover:bg-white/10 transition-all flex items-center gap-2 group"
                >
                  <svg
                    class="w-4 h-4 group-hover:rotate-90 transition-transform duration-300"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                    />
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                    />
                  </svg>
                  Settings
                </A>
                <button
                  onClick={handleLogout}
                  class="px-4 py-2 rounded-xl bg-gradient-to-r from-red-500 to-rose-500 text-white hover:opacity-90 transition-all shadow-lg shadow-red-500/25 flex items-center gap-2"
                >
                  <svg
                    class="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                    />
                  </svg>
                  Logout
                </button>
              </div>
            </Motion.div>

            {/* Stats Cards */}
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
              <For each={stats}>
                {(stat, index) => (
                  <Motion.div
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.5, delay: 0.1 * index() }}
                    class="group relative"
                  >
                    <div class="glass-card rounded-2xl p-6 hover:scale-[1.02] transition-all duration-300">
                      {/* Glow effect */}
                      <div
                        class={`absolute inset-0 rounded-2xl bg-gradient-to-r ${stat.gradient} opacity-0 group-hover:opacity-10 blur-xl transition-opacity duration-300`}
                      />

                      <div class="relative flex items-center gap-4">
                        <div
                          class={`p-3 rounded-xl bg-gradient-to-br ${stat.gradient} text-white shadow-lg`}
                        >
                          {stat.icon}
                        </div>
                        <div>
                          <p class="text-muted-foreground text-sm">
                            {stat.title}
                          </p>
                          <p class="text-3xl font-bold gradient-text">
                            {stat.value}
                          </p>
                        </div>
                      </div>
                    </div>
                  </Motion.div>
                )}
              </For>
            </div>

            {/* Quick Links */}
            <Motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.4 }}
            >
              <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
                <svg
                  class="w-5 h-5 text-primary"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M13 10V3L4 14h7v7l9-11h-7z"
                  />
                </svg>
                Quick Access
              </h2>
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <For each={quickLinks}>
                  {(link, index) => (
                    <Motion.div
                      initial={{ opacity: 0, scale: 0.9 }}
                      animate={{ opacity: 1, scale: 1 }}
                      transition={{ duration: 0.3, delay: 0.5 + 0.1 * index() }}
                    >
                      <A
                        href={link.href}
                        class={`group relative block p-6 rounded-2xl bg-gradient-to-br ${link.gradient} text-white text-center overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105`}
                      >
                        {/* Shine effect */}
                        <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500">
                          <div class="absolute inset-0 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-1000 bg-gradient-to-r from-transparent via-white/20 to-transparent" />
                        </div>

                        <span class="text-4xl mb-3 block group-hover:scale-110 transition-transform duration-300">
                          {link.icon}
                        </span>
                        <span class="font-medium">{link.label}</span>
                      </A>
                    </Motion.div>
                  )}
                </For>
              </div>
            </Motion.div>

            {/* Recent Activity Placeholder */}
            <Motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.6 }}
              class="mt-8"
            >
              <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
                <svg
                  class="w-5 h-5 text-primary"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                Recent Activity
              </h2>
              <div class="glass-card rounded-2xl p-8 text-center">
                <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-muted/50 flex items-center justify-center">
                  <svg
                    class="w-8 h-8 text-muted-foreground"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                    />
                  </svg>
                </div>
                <p class="text-muted-foreground">No recent activity yet</p>
                <p class="text-sm text-muted-foreground/70 mt-1">
                  Start exploring to see your activity here
                </p>
              </div>
            </Motion.div>
          </div>
        </main>
      </Show>
    </>
  );
}
