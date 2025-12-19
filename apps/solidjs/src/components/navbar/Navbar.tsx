import { A, useLocation } from "@solidjs/router";
import { createSignal, Show, For } from "solid-js";
import { useTheme } from "../providers/theme-provider";

const navLinks = [
    { href: "/", label: "Home" },
    { href: "/anime", label: "Anime" },
    { href: "/komik", label: "Komik" },
    { href: "/chat", label: "Chat" },
    { href: "/project", label: "Project" },
];

export function Navbar() {
    const [isOpen, setIsOpen] = createSignal(false);
    const { theme, setTheme, resolvedTheme } = useTheme();
    const location = useLocation();

    const toggleMenu = () => setIsOpen(!isOpen());

    const cycleTheme = () => {
        const themes: Array<"light" | "dark" | "system"> = ["light", "dark", "system"];
        const currentIndex = themes.indexOf(theme());
        const nextIndex = (currentIndex + 1) % themes.length;
        setTheme(themes[nextIndex]);
    };

    return (
        <nav class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="container mx-auto flex h-16 items-center justify-between px-4">
                {/* Logo */}
                <A href="/" class="flex items-center space-x-2">
                    <span class="text-xl font-bold bg-gradient-to-r from-primary to-primary/60 bg-clip-text text-transparent">
                        Asepharyana
                    </span>
                </A>

                {/* Desktop Navigation */}
                <div class="hidden md:flex items-center space-x-6">
                    <For each={navLinks}>
                        {(link) => (
                            <A
                                href={link.href}
                                class={`text-sm font-medium transition-colors hover:text-primary ${location.pathname === link.href
                                        ? "text-primary"
                                        : "text-muted-foreground"
                                    }`}
                            >
                                {link.label}
                            </A>
                        )}
                    </For>

                    {/* Theme Toggle */}
                    <button
                        onClick={cycleTheme}
                        class="p-2 rounded-md hover:bg-accent transition-colors"
                        title={`Current: ${theme()}`}
                    >
                        <Show when={resolvedTheme() === "dark"} fallback={
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                            </svg>
                        }>
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                            </svg>
                        </Show>
                    </button>

                    {/* Auth Links */}
                    <A
                        href="/login"
                        class="text-sm font-medium text-muted-foreground hover:text-primary transition-colors"
                    >
                        Login
                    </A>
                </div>

                {/* Mobile Menu Button */}
                <button
                    class="md:hidden p-2 rounded-md hover:bg-accent transition-colors"
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
                <div class="md:hidden border-t border-border">
                    <div class="container mx-auto px-4 py-4 space-y-3">
                        <For each={navLinks}>
                            {(link) => (
                                <A
                                    href={link.href}
                                    class={`block py-2 text-sm font-medium transition-colors ${location.pathname === link.href
                                            ? "text-primary"
                                            : "text-muted-foreground hover:text-primary"
                                        }`}
                                    onClick={() => setIsOpen(false)}
                                >
                                    {link.label}
                                </A>
                            )}
                        </For>
                        <div class="flex items-center justify-between pt-4 border-t border-border">
                            <button
                                onClick={cycleTheme}
                                class="flex items-center gap-2 text-sm text-muted-foreground"
                            >
                                Theme: {theme()}
                            </button>
                            <A
                                href="/login"
                                class="text-sm font-medium text-primary"
                                onClick={() => setIsOpen(false)}
                            >
                                Login
                            </A>
                        </div>
                    </div>
                </div>
            </Show>
        </nav>
    );
}
