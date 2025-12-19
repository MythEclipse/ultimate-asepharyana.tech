import { Title } from "@solidjs/meta";
import { A, useNavigate } from "@solidjs/router";
import { createEffect, createSignal, Show } from "solid-js";
import { useAuth } from "~/lib/auth-context";
import { useTheme } from "~/components/providers/theme-provider";

export default function SettingsPage() {
    const navigate = useNavigate();
    const { user, loading, logout } = useAuth();
    const { theme, setTheme } = useTheme();

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
            <Title>Settings | Asepharyana</Title>
            <Show when={loading()}>
                <div class="min-h-screen flex items-center justify-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary" />
                </div>
            </Show>

            <Show when={!loading() && user()}>
                <main class="min-h-screen bg-background text-foreground p-4 md:p-8">
                    <div class="max-w-2xl mx-auto">
                        <h1 class="text-3xl font-bold mb-8">Settings</h1>

                        {/* Profile Section */}
                        <section class="mb-8 p-6 rounded-xl bg-card border border-border">
                            <h2 class="text-xl font-semibold mb-4">Profile</h2>
                            <div class="space-y-4">
                                <div>
                                    <label class="text-sm text-muted-foreground">Name</label>
                                    <p class="font-medium">{user()?.name}</p>
                                </div>
                                <div>
                                    <label class="text-sm text-muted-foreground">Email</label>
                                    <p class="font-medium">{user()?.email}</p>
                                </div>
                            </div>
                        </section>

                        {/* Appearance Section */}
                        <section class="mb-8 p-6 rounded-xl bg-card border border-border">
                            <h2 class="text-xl font-semibold mb-4">Appearance</h2>
                            <div class="flex gap-3">
                                <button
                                    onClick={() => setTheme("light")}
                                    class={`flex-1 p-4 rounded-lg border-2 transition-all ${theme() === "light" ? "border-primary bg-primary/10" : "border-border hover:border-primary/50"
                                        }`}
                                >
                                    <span class="text-2xl mb-2 block">☀️</span>
                                    <span class="font-medium">Light</span>
                                </button>
                                <button
                                    onClick={() => setTheme("dark")}
                                    class={`flex-1 p-4 rounded-lg border-2 transition-all ${theme() === "dark" ? "border-primary bg-primary/10" : "border-border hover:border-primary/50"
                                        }`}
                                >
                                    <span class="text-2xl mb-2 block">🌙</span>
                                    <span class="font-medium">Dark</span>
                                </button>
                                <button
                                    onClick={() => setTheme("system")}
                                    class={`flex-1 p-4 rounded-lg border-2 transition-all ${theme() === "system" ? "border-primary bg-primary/10" : "border-border hover:border-primary/50"
                                        }`}
                                >
                                    <span class="text-2xl mb-2 block">💻</span>
                                    <span class="font-medium">System</span>
                                </button>
                            </div>
                        </section>

                        {/* Danger Zone */}
                        <section class="p-6 rounded-xl bg-destructive/10 border border-destructive/20">
                            <h2 class="text-xl font-semibold mb-4 text-destructive">Danger Zone</h2>
                            <button
                                onClick={handleLogout}
                                class="px-6 py-3 rounded-lg bg-destructive text-destructive-foreground hover:bg-destructive/90 transition-colors"
                            >
                                Logout
                            </button>
                        </section>
                    </div>
                </main>
            </Show>
        </>
    );
}
