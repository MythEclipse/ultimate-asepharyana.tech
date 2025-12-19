import { Title } from "@solidjs/meta";
import { A, useNavigate } from "@solidjs/router";
import { createEffect, Show } from "solid-js";
import { useAuth } from "~/lib/auth-context";

export default function DashboardPage() {
    const navigate = useNavigate();
    const { user, loading, logout } = useAuth();

    // Redirect if not logged in
    createEffect(() => {
        if (!loading() && !user()) {
            navigate("/login");
        }
    });

    const handleLogout = async () => {
        await logout();
        navigate("/");
    };

    return (
        <>
            <Title>Dashboard | Asepharyana</Title>
            <Show when={loading()}>
                <div class="min-h-screen flex items-center justify-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary" />
                </div>
            </Show>

            <Show when={!loading() && user()}>
                <main class="min-h-screen bg-background text-foreground p-4 md:p-8">
                    <div class="max-w-6xl mx-auto">
                        {/* Header */}
                        <div class="flex items-center justify-between mb-8">
                            <div>
                                <h1 class="text-3xl font-bold">Dashboard</h1>
                                <p class="text-muted-foreground">Welcome back, {user()?.name}!</p>
                            </div>
                            <div class="flex gap-3">
                                <A
                                    href="/settings"
                                    class="px-4 py-2 rounded-lg bg-card border border-border hover:bg-accent transition-colors"
                                >
                                    Settings
                                </A>
                                <button
                                    onClick={handleLogout}
                                    class="px-4 py-2 rounded-lg bg-destructive text-destructive-foreground hover:bg-destructive/90 transition-colors"
                                >
                                    Logout
                                </button>
                            </div>
                        </div>

                        {/* Stats Cards */}
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                            <div class="p-6 rounded-xl bg-card border border-border">
                                <div class="flex items-center gap-4">
                                    <div class="p-3 rounded-lg bg-blue-100 dark:bg-blue-900/50">
                                        <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18" />
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-muted-foreground text-sm">Anime Bookmarks</p>
                                        <p class="text-2xl font-bold">0</p>
                                    </div>
                                </div>
                            </div>

                            <div class="p-6 rounded-xl bg-card border border-border">
                                <div class="flex items-center gap-4">
                                    <div class="p-3 rounded-lg bg-orange-100 dark:bg-orange-900/50">
                                        <svg class="w-6 h-6 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-muted-foreground text-sm">Komik Bookmarks</p>
                                        <p class="text-2xl font-bold">0</p>
                                    </div>
                                </div>
                            </div>

                            <div class="p-6 rounded-xl bg-card border border-border">
                                <div class="flex items-center gap-4">
                                    <div class="p-3 rounded-lg bg-green-100 dark:bg-green-900/50">
                                        <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-muted-foreground text-sm">Chat History</p>
                                        <p class="text-2xl font-bold">0</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        {/* Quick Links */}
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <A href="/anime" class="p-6 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 text-white text-center hover:opacity-90 transition-opacity">
                                <span class="text-3xl mb-2 block">📺</span>
                                <span class="font-medium">Watch Anime</span>
                            </A>
                            <A href="/komik" class="p-6 rounded-xl bg-gradient-to-br from-orange-500 to-red-600 text-white text-center hover:opacity-90 transition-opacity">
                                <span class="text-3xl mb-2 block">📖</span>
                                <span class="font-medium">Read Komik</span>
                            </A>
                            <A href="/chat" class="p-6 rounded-xl bg-gradient-to-br from-green-500 to-teal-600 text-white text-center hover:opacity-90 transition-opacity">
                                <span class="text-3xl mb-2 block">🤖</span>
                                <span class="font-medium">AI Chat</span>
                            </A>
                            <A href="/project" class="p-6 rounded-xl bg-gradient-to-br from-pink-500 to-rose-600 text-white text-center hover:opacity-90 transition-opacity">
                                <span class="text-3xl mb-2 block">💼</span>
                                <span class="font-medium">Projects</span>
                            </A>
                        </div>
                    </div>
                </main>
            </Show>
        </>
    );
}
