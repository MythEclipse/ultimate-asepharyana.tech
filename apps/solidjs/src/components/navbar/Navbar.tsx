import { A, useLocation } from "@solidjs/router";
import { createSignal, Show, For } from "solid-js";
import { useTheme } from "../providers/theme-provider";
import { useAuth } from "~/lib/auth-context";

const navLinks = [
    { href: "/project", label: "Project" },
];

export function Navbar() {
    const [isOpen, setIsOpen] = createSignal(false);
    const { theme, setTheme, resolvedTheme } = useTheme();
    const { user, logout } = useAuth();
    const location = useLocation();

    const toggleMenu = () => setIsOpen(!isOpen());

    const cycleTheme = () => {
        const themes: Array<"light" | "dark" | "system"> = ["light", "dark", "system"];
        const currentIndex = themes.indexOf(theme());
        const nextIndex = (currentIndex + 1) % themes.length;
        setTheme(themes[nextIndex]);
    };

    const handleLogout = async () => {
        await logout();
        setIsOpen(false);
    };

    return (
        <nav class="sticky top-0 z-50 w-full glass-subtle border-b border-white/10">
            <div class="container mx-auto flex h-16 items-center justify-between px-4">
                {/* Logo */}
                <A href="/" class="flex items-center space-x-2 group">
                    <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary via-accent to-neon-cyan flex items-center justify-center shadow-lg group-hover:shadow-primary/30 transition-shadow">
                        <span class="text-white font-bold text-sm">A</span>
                    </div>
                    <span class="text-xl font-bold gradient-text-animated">
                        Asepharyana
                    </span>
                </A>

                {/* Desktop Navigation */}
                <div class="hidden md:flex items-center space-x-1">
                    <For each={navLinks}>
                        {(link) => (
                            <A
                                href={link.href}
                                class={`relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 ${location.pathname === link.href ||
                                    (link.href !== "/" && location.pathname.startsWith(link.href))
                                    ? "text-primary"
                                    : "text-muted-foreground hover:text-foreground"
                                    }`}
                            >
                                {link.label}
                                <Show when={location.pathname === link.href ||
                                    (link.href !== "/" && location.pathname.startsWith(link.href))}>
                                    <span class="absolute bottom-0 left-1/2 -translate-x-1/2 w-4 h-0.5 bg-gradient-to-r from-primary to-accent rounded-full" />
                                </Show>
                            </A>
                        )}
                    </For>

                    {/* Theme Toggle */}
                    <button
                        onClick={cycleTheme}
                        class="p-2.5 rounded-lg hover:bg-white/10 transition-all group"
                        title={`Theme: ${theme()}`}
                    >
                        <div class="relative w-5 h-5">
                            <Show when={resolvedTheme() === "dark"} fallback={
                                <svg class="w-5 h-5 text-amber-500 group-hover:rotate-45 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                </svg>
                            }>
                                <svg class="w-5 h-5 text-indigo-400 group-hover:-rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                </svg>
                            </Show>
                        </div>
                    </button>

                    {/* Auth */}
                    <Show when={user()} fallback={
                        <A
                            href="/login"
                            class="ml-2 px-4 py-2 text-sm font-medium rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all hover:shadow-lg hover:shadow-primary/25"
                        >
                            Sign In
                        </A>
                    }>
                        <div class="flex items-center gap-2 ml-2">
                            <A
                                href="/dashboard"
                                class="px-4 py-2 text-sm font-medium rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
                            >
                                Dashboard
                            </A>
                        </div>
                    </Show>
                </div>

                {/* Mobile Menu Button */}
                <button
                    class="md:hidden p-2.5 rounded-lg hover:bg-white/10 transition-colors"
                    onClick={toggleMenu}
                >
                    <Show when={isOpen()} fallback={
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                        </svg>
                    }>
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </Show>
                </button>
            </div>

            {/* Mobile Menu */}
            <Show when={isOpen()}>
                <div class="md:hidden glass-card border-t border-white/10 animate-slide-down">
                    <div class="container mx-auto px-4 py-4 space-y-1">
                        <For each={navLinks}>
                            {(link) => (
                                <A
                                    href={link.href}
                                    class={`block py-3 px-4 text-sm font-medium rounded-lg transition-all ${location.pathname === link.href
                                        ? "bg-primary/10 text-primary"
                                        : "text-muted-foreground hover:bg-white/5 hover:text-foreground"
                                        }`}
                                    onClick={() => setIsOpen(false)}
                                >
                                    {link.label}
                                </A>
                            )}
                        </For>
                        <div class="flex items-center justify-between pt-4 mt-4 border-t border-border/50">
                            <button
                                onClick={cycleTheme}
                                class="flex items-center gap-2 text-sm text-muted-foreground px-4 py-2 rounded-lg hover:bg-white/5"
                            >
                                <Show when={resolvedTheme() === "dark"} fallback={
                                    <svg class="w-4 h-4 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                }>
                                    <svg class="w-4 h-4 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                    </svg>
                                </Show>
                                {theme() === "system" ? "System" : theme() === "dark" ? "Dark" : "Light"}
                            </button>
                            <Show when={user()} fallback={
                                <A
                                    href="/login"
                                    class="px-4 py-2 text-sm font-medium rounded-lg bg-primary text-primary-foreground"
                                    onClick={() => setIsOpen(false)}
                                >
                                    Sign In
                                </A>
                            }>
                                <div class="flex items-center gap-2">
                                    <A
                                        href="/dashboard"
                                        class="px-3 py-2 text-sm font-medium rounded-lg bg-primary/10 text-primary"
                                        onClick={() => setIsOpen(false)}
                                    >
                                        Dashboard
                                    </A>
                                    <button
                                        onClick={handleLogout}
                                        class="px-3 py-2 text-sm font-medium rounded-lg bg-destructive/10 text-destructive"
                                    >
                                        Logout
                                    </button>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </Show>
        </nav>
    );
}

