use leptos::*;
use leptos_router::A;
use crate::components::ui::GlitchText;

#[component]
pub fn Hero(visuals_url: String) -> impl IntoView {
    view! {
        <section class="min-h-screen flex flex-col items-center justify-center px-6 md:px-12 py-32 relative group overflow-hidden scanlines">
            // Bevy Visuals Integration — desktop only
            <div class="absolute inset-0 -z-10 hidden md:block">
                <iframe
                    src=visuals_url
                    class="w-full h-full border-0 opacity-60 mix-blend-screen pointer-events-none grayscale brightness-150 dark:brightness-100 dark:mix-blend-screen mix-blend-multiply"
                    title="Neural Particle Simulation"
                />
            </div>

            <div class="absolute inset-0 opacity-40 pointer-events-none">
                <div class="absolute top-1/4 left-1/4 w-[50rem] h-[50rem] bg-primary/20 rounded-full blur-[160px] animate-pulse-slow" />
                <div class="absolute bottom-1/4 right-1/4 w-[40rem] h-[40rem] bg-accent/10 rounded-full blur-[140px] animate-pulse-slow [animation-delay:2s]" />
            </div>

            <div class="max-w-7xl mx-auto w-full flex flex-col items-center text-center space-y-24 relative z-10">
                <div class="space-y-12 animate-slide-up fill-mode-forwards">
                    <h1 class="text-6xl sm:text-8xl md:text-11xl font-black italic tracking-tighter leading-[0.8] uppercase font-display">
                        <GlitchText text="Asep" class="text-foreground/90 block translate-y-8 scale-y-90 tracking-[-0.08em] mb-8" />
                        <div class="block py-8">
                            <GlitchText text="Haryana Saputra" class="text-foreground/90 block" />
                        </div>
                    </h1>
                    <div class="max-w-4xl mx-auto space-y-8 px-4">
                        <p class="text-xl md:text-3xl text-muted-foreground leading-relaxed font-medium italic font-sans tracking-tight">
                            "Crafting robust " <GlitchText text="Backend" class="text-primary font-black drop-shadow-[0_0_15px_hsla(var(--primary),0.4)]" />
                            " systems with high-performance " <GlitchText text="Frontend" class="text-accent font-black drop-shadow-[0_0_15px_hsla(var(--accent),0.4)]" />
                            " solutions to build seamless digital experiences."
                        </p>
                        <div class="flex items-center justify-center gap-4">
                            <div class="w-12 md:w-16 h-1 bg-muted/20 rounded-full" />
                            <div class="w-20 md:w-24 h-1.5 bg-gradient-to-r from-primary to-accent rounded-full shadow-glow" />
                            <div class="w-12 md:w-16 h-1 bg-muted/20 rounded-full" />
                        </div>
                    </div>
                </div>

                // Primary CTAs
                <div class="flex flex-col sm:flex-row items-center justify-center gap-6 md:gap-10 animate-fade-in [animation-delay:400ms]">
                    <A href="/project" class="w-full sm:w-auto group relative px-12 md:px-16 py-6 md:py-8 rounded-[2rem] md:rounded-[2.5rem] bg-foreground text-background font-black text-[10px] md:text-xs uppercase tracking-[0.4em] shadow-[0_30px_60px_rgba(0,0,0,0.3)] hover:scale-105 active:scale-95 transition-all duration-700 overflow-hidden industrial-snap font-display">
                        <div class="absolute inset-0 bg-gradient-to-r from-primary to-accent opacity-0 group-hover:opacity-30 transition-opacity" />
                        <span class="relative z-10 flex items-center justify-center gap-6 chromatic-hover">
                            "View Portfolio"
                            <svg class="w-6 h-6 group-hover:translate-x-3 transition-transform duration-700" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </span>
                    </A>

                    <a href="mailto:superaseph@gmail.com" class="w-full sm:w-auto px-12 md:px-16 py-6 md:py-8 rounded-[2rem] md:rounded-[2.5rem] glass border border-border/20 text-foreground font-black text-[10px] md:text-xs uppercase tracking-[0.4em] hover:bg-muted/50 hover:border-primary/40 transition-all duration-700 hover:scale-105 industrial-snap font-display chromatic-hover text-center">
                        "Contact Me"
                    </a>
                </div>
            </div>

            // Neural Marquee
            <div class="absolute bottom-0 w-full py-16 bg-black/10 border-y border-white/5 backdrop-blur-md overflow-hidden rotate-[-1deg] scale-110">
                <div class="flex whitespace-nowrap animate-marquee hover:[animation-play-state:paused]">
                    {(0..10).map(|_| view! {
                        <div class="flex items-center gap-16 px-8">
                            <span class="text-7xl md:text-9xl font-black italic tracking-tighter uppercase text-foreground/5">"Architecture"</span>
                            <span class="text-7xl md:text-9xl font-black italic tracking-tighter uppercase text-indigo-500/10">"Performance"</span>
                            <span class="text-7xl md:text-9xl font-black italic tracking-tighter uppercase text-foreground/5">"Immersion"</span>
                            <span class="text-7xl md:text-9xl font-black italic tracking-tighter uppercase text-purple-500/10">"Intelligence"</span>
                        </div>
                    }).collect_view()}
                </div>
            </div>

            // Scroll Indicator
            <div class="absolute bottom-32 left-1/2 -translate-x-1/2 animate-bounce opacity-30">
                <div class="w-7 h-12 rounded-full border-2 border-foreground/20 flex justify-center p-2.5">
                    <div class="w-2 h-2 rounded-full bg-primary animate-scroll-pill shadow-[0_0_10px_hsla(var(--primary),1)]" />
                </div>
            </div>
        </section>
    }
}
