import { A } from "@solidjs/router";
import { createSignal, For, onMount, onCleanup, Show } from "solid-js";
import { Motion } from "solid-motionone";

const socialLinks = [
    {
        name: "GitHub",
        href: "https://github.com/asepharyana",
        icon: (
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
        ),
    },
    {
        name: "LinkedIn",
        href: "https://linkedin.com/in/asepharyana",
        icon: (
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
            </svg>
        ),
    },
    {
        name: "Twitter",
        href: "https://twitter.com/asepharyana",
        icon: (
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
            </svg>
        ),
    },
];

const footerLinks = [
    { label: "Home", href: "/" },
    { label: "Anime", href: "/anime" },
    { label: "Komik", href: "/komik" },
    { label: "Chat", href: "/chat" },
    { label: "Tools", href: "/compressor" },
    { label: "Project", href: "/project" },
];

export function Footer() {
    const currentYear = new Date().getFullYear();
    const [showScrollTop, setShowScrollTop] = createSignal(false);

    const handleScroll = () => {
        setShowScrollTop(window.scrollY > 300);
    };

    const scrollToTop = () => {
        window.scrollTo({ top: 0, behavior: "smooth" });
    };

    onMount(() => {
        window.addEventListener("scroll", handleScroll);
    });

    onCleanup(() => {
        if (typeof window !== "undefined") {
            window.removeEventListener("scroll", handleScroll);
        }
    });

    return (
        <>
            <footer class="glass-subtle border-t border-white/10 mt-auto relative">
                {/* Decorative gradient */}
                <div class="absolute inset-0 bg-gradient-to-t from-primary/5 via-transparent to-transparent pointer-events-none" />

                <div class="container mx-auto px-4 py-12 relative">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-8">
                        {/* Brand Section */}
                        <div>
                            <A href="/" class="inline-flex items-center gap-2 group">
                                <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary to-purple-600 flex items-center justify-center shadow-lg shadow-primary/30 group-hover:scale-110 transition-transform">
                                    <span class="text-white font-bold text-lg">A</span>
                                </div>
                                <span class="text-xl font-bold gradient-text">Asepharyana</span>
                            </A>
                            <p class="text-sm text-muted-foreground mt-3 max-w-xs">
                                Your digital entertainment hub. Stream anime, read manga, chat with AI, and discover amazing tools.
                            </p>
                        </div>

                        {/* Quick Links */}
                        <div>
                            <h3 class="font-semibold mb-4 text-foreground">Quick Links</h3>
                            <div class="grid grid-cols-2 gap-2">
                                <For each={footerLinks}>
                                    {(link) => (
                                        <A
                                            href={link.href}
                                            class="text-sm text-muted-foreground hover:text-primary transition-colors py-1"
                                        >
                                            {link.label}
                                        </A>
                                    )}
                                </For>
                            </div>
                        </div>

                        {/* Social & Contact */}
                        <div>
                            <h3 class="font-semibold mb-4 text-foreground">Connect</h3>
                            <div class="flex items-center gap-2 mb-4">
                                <For each={socialLinks}>
                                    {(social) => (
                                        <a
                                            href={social.href}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="p-2.5 rounded-xl glass-subtle hover:bg-white/10 text-muted-foreground hover:text-primary hover:scale-110 transition-all"
                                            title={social.name}
                                        >
                                            {social.icon}
                                        </a>
                                    )}
                                </For>
                            </div>
                            <p class="text-sm text-muted-foreground">
                                Built with <span class="text-primary font-medium">SolidStart</span>
                            </p>
                        </div>
                    </div>

                    {/* Copyright */}
                    <div class="pt-8 border-t border-border/30 flex flex-col sm:flex-row items-center justify-between gap-4">
                        <p class="text-xs text-muted-foreground">
                            © {currentYear} Asepharyana. All rights reserved.
                        </p>
                        <p class="text-xs text-muted-foreground flex items-center gap-1">
                            Made with <span class="text-red-500 animate-pulse">❤️</span> in Indonesia
                        </p>
                    </div>
                </div>
            </footer>

            {/* Scroll to Top Button */}
            <Show when={showScrollTop()}>
                <Motion.button
                    initial={{ opacity: 0, scale: 0.5, y: 20 }}
                    animate={{ opacity: 1, scale: 1, y: 0 }}
                    exit={{ opacity: 0, scale: 0.5, y: 20 }}
                    onClick={scrollToTop}
                    class="fixed bottom-6 right-6 z-50 p-3 rounded-full bg-gradient-to-r from-primary to-purple-600 text-white shadow-lg shadow-primary/30 hover:shadow-xl hover:scale-110 transition-all group"
                    title="Scroll to top"
                >
                    <svg class="w-5 h-5 group-hover:-translate-y-0.5 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
                    </svg>
                </Motion.button>
            </Show>
        </>
    );
}
