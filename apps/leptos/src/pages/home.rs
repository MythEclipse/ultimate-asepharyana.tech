use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use crate::components::logo::tech_icons::TECH_STACK;
use crate::components::logo::social_icons::{Instagram, LinkedIn, GitHub};
use crate::components::ui::{GlitchText, CachedImage};
use std::sync::atomic::{AtomicBool, Ordering};

static VISUALS_READY: AtomicBool = AtomicBool::new(false);

#[component]
pub fn HomePage() -> impl IntoView {
    let (visuals_ready, set_visuals_ready) = create_signal(VISUALS_READY.load(Ordering::Relaxed));

    // Listen for Bevy's Readiness Signal (Local Background)
    create_effect(move |_| {
        if visuals_ready.get_untracked() {
            return;
        }

        #[cfg(feature = "csr")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            let handle_message = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
                if let Some(msg) = ev.data().as_string() {
                    if msg == "PROTOCOL_READY" {
                        set_visuals_ready.set(true);
                        VISUALS_READY.store(true, Ordering::Relaxed);
                    }
                }
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);

            window()
                .add_event_listener_with_callback("message", handle_message.as_ref().unchecked_ref())
                .unwrap();

            handle_message.forget(); // Keep the listener alive
        }

        // Fallback for asset readiness
        set_timeout(
            move || {
                if !visuals_ready.get_untracked() {
                    set_visuals_ready.set(true);
                    VISUALS_READY.store(true, Ordering::Relaxed);
                }
            },
            std::time::Duration::from_millis(8000),
        );
    });

    // determine visuals iframe URL at compile time (env override allowed)
    let visuals_url = option_env!("VISUALS_URL").unwrap_or("https://visuals.asepharyana.tech/");

    view! {
        <Title text="Full-Stack Developer | Asep Haryana"/>


        <main class="relative z-10 w-full overflow-hidden">
            // Hero Section: Professional Identity
            <section class="min-h-screen flex flex-col items-center justify-center px-6 md:px-12 py-32 relative group overflow-hidden scanlines">
                // Bevy Visuals Integration
                <iframe
                    src={visuals_url}
                    class="absolute inset-0 w-full h-full border-0 -z-10 opacity-60 mix-blend-screen pointer-events-none grayscale brightness-150 dark:brightness-100 dark:mix-blend-screen mix-blend-multiply"
                    title="Neural Particle Simulation"
                />

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

                    // Primary CTAs (Elite Variant)
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

                // Neural Marquee: The Architecture Protocol
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

            // The Arsenal: Bento-Style Grid
            <section class="py-24 md:py-40 lg:py-56 px-6 relative">
                 <div class="max-w-7xl mx-auto space-y-20 md:space-y-32">
                    <div class="text-center space-y-8">
                        <div class="inline-flex items-center gap-4 px-5 py-1.5 rounded-full glass border border-white/10 text-[9px] font-black uppercase tracking-[0.6em] text-primary mb-6 shadow-2xl">
                            "Professional Background"
                        </div>
                        <h2 class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase leading-none">
                            "Technical " <GlitchText text="Stack" class="text-primary" />
                        </h2>
                        <div class="h-2 w-48 bg-gradient-to-r from-primary via-accent to-primary mx-auto rounded-full shadow-glow" />
                    </div>

                    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-6 md:gap-8">
                        {TECH_STACK.into_iter().enumerate().map(|(i, skill)| {
                            let delay = format!("animation-delay: {}ms", 100 * i);
                            view! {
                                <div class="group relative animate-slide-up opacity-0 fill-mode-forwards" style=delay>
                                    <div class="absolute -inset-2 bg-gradient-to-br from-primary to-accent rounded-[2rem] md:rounded-[3rem] opacity-0 group-hover:opacity-20 blur-3xl transition-all duration-1000" />
                                    <div class="relative glass-card h-full p-8 md:p-12 rounded-[2rem] md:rounded-[3rem] flex flex-col items-center justify-center gap-6 md:gap-8 border border-border/10 hover:border-primary/40 transition-all duration-700 hover:scale-[1.1] hover:-rotate-1 group shadow-xl backdrop-blur-3xl overflow-hidden">
                                        <div class="absolute -right-8 -top-8 w-24 h-24 bg-primary/5 rounded-full blur-2xl group-hover:bg-primary/20 transition-colors" />
                                        <div class="w-16 h-16 md:w-20 md:h-20 relative flex items-center justify-center">
                                            <div class="absolute inset-0 bg-primary/5 rounded-2xl blur-xl scale-0 group-hover:scale-150 transition-transform duration-1000" />
                                            <CachedImage 
                                                src=skill.image.to_string() 
                                                alt=skill.name.to_string() 
                                                class="w-12 h-12 md:w-16 md:h-16 relative z-10 drop-shadow-2xl brightness-90 group-hover:brightness-110 transition-all filter group-hover:drop-shadow-[0_0_20px_rgba(255,255,255,0.4)]".to_string() 
                                            />
                                        </div>
                                        <div class="space-y-2 text-center">
                                            <span class="text-[10px] md:text-[11px] font-black uppercase tracking-[0.4em] text-muted-foreground group-hover:text-primary transition-colors block">
                                                {skill.name}
                                            </span>
                                            <div class="h-0.5 w-6 bg-primary mx-auto scale-x-0 group-hover:scale-x-100 transition-transform duration-500" />
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                 </div>
            </section>

            // Featured Projects Preview
            <section class="py-24 md:py-40 lg:py-56 px-6 bg-accent/[0.02] relative">
                    <div class="max-w-7xl mx-auto space-y-20 md:space-y-32">
                    <div class="text-center space-y-8">
                        <div class="inline-flex items-center gap-4 px-5 py-1.5 rounded-full glass border border-border/20 text-[9px] font-black uppercase tracking-[0.6em] text-primary mb-6 shadow-2xl">
                            "Showcase"
                        </div>
                        <h2 class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase leading-none">
                            "Featured " <GlitchText text="Projects" class="text-primary" />
                        </h2>
                        <div class="h-2 w-48 bg-gradient-to-r from-primary via-accent to-primary mx-auto rounded-full shadow-glow" />
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10 md:gap-12">
                         <div class="glass-card p-10 md:p-12 rounded-[2.5rem] md:rounded-[3rem] border border-border/10 space-y-6 group hover:border-primary/30 transition-all duration-700">
                            <div class="h-40 md:h-48 rounded-[1.5rem] md:rounded-[2rem] bg-primary/5 border border-border/10 overflow-hidden">
                                <img src="/public/project-rust.png" alt="Rust API" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-2xl md:text-3xl font-black italic uppercase tracking-tighter">"Rust Infrastructure"</h3>
                                <p class="text-muted-foreground/80 text-sm font-medium leading-relaxed">"High-performance backend systems built with memory-safe Rust architecture."</p>
                            </div>
                         </div>
                         <div class="glass-card p-10 md:p-12 rounded-[2.5rem] md:rounded-[3rem] border border-border/10 space-y-6 group hover:border-primary/30 transition-all duration-700">
                            <div class="h-40 md:h-48 rounded-[1.5rem] md:rounded-[2rem] bg-primary/5 border border-border/10 overflow-hidden">
                                <img src="/public/project-elysia.png" alt="Elysia API" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-2xl md:text-3xl font-black italic uppercase tracking-tighter">"Elysia Discovery"</h3>
                                <p class="text-muted-foreground/80 text-sm font-medium leading-relaxed">"Scalable API services featuring ultra-fast response times and full OpenAPI documentation."</p>
                            </div>
                         </div>
                         <div class="glass-card p-10 md:p-12 rounded-[2.5rem] md:rounded-[3rem] border border-border/10 space-y-6 group hover:border-primary/30 transition-all duration-700">
                            <div class="h-40 md:h-48 rounded-[1.5rem] md:rounded-[2rem] bg-primary/5 border border-border/10 overflow-hidden">
                                <img src="/public/project-anime.png" alt="Anime Streaming" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-2xl md:text-3xl font-black italic uppercase tracking-tighter">"Media Ecosystem"</h3>
                                <p class="text-muted-foreground/80 text-sm font-medium leading-relaxed">"Cinematic frontend experiences designed for high-end content delivery and interaction."</p>
                            </div>
                         </div>
                    </div>

                    <div class="text-center">
                        <A href="/project" class="inline-flex items-center gap-4 px-10 md:px-12 py-5 md:py-6 rounded-full glass border border-border/20 text-[10px] font-black uppercase tracking-[0.4em] hover:bg-muted font-display hover:border-primary/40 transition-all">
                            "Explore Full Archive"
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </A>
                    </div>
                </div>
            </section>

            // Connection Section
            <section class="py-24 md:py-40 lg:py-56 px-6 overflow-hidden relative">
                <div class="absolute inset-0 pointer-events-none">
                    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full h-[1px] bg-gradient-to-r from-transparent via-primary/30 to-transparent" />
                </div>

                <div class="max-w-7xl mx-auto rounded-[3rem] md:rounded-[5rem] p-8 md:p-32 relative overflow-hidden glass border border-border/10 shadow-[0_80px_160px_rgba(0,0,0,0.2)] dark:shadow-[0_120px_250px_rgba(0,0,0,0.6)]">
                    <div class="absolute -right-60 -top-60 w-[50rem] h-[50rem] bg-primary/10 rounded-full blur-[180px] animate-tilt-slow opacity-60" />
                    <div class="absolute -left-60 -bottom-60 w-[50rem] h-[50rem] bg-accent/10 rounded-full blur-[180px] animate-tilt-reverse-slow opacity-40" />

                    <div class="relative z-10 flex flex-col items-center text-center space-y-20 md:space-y-24">
                        <div class="space-y-10 md:space-y-14">
                            <div class="space-y-8">
                                <span class="px-6 md:py-2.5 rounded-full glass-subtle text-[11px] font-black uppercase tracking-[0.6em] text-primary shadow-xl">
                                    "Communication"
                                </span>
                                <h2 class="text-5xl md:text-9xl font-black italic tracking-tighter leading-none uppercase">
                                    "Get In " <br/> <GlitchText text="Touch" class="text-primary" />
                                </h2>
                                <p class="text-lg md:text-3xl text-muted-foreground leading-relaxed max-w-2xl mx-auto font-medium italic tracking-tight">
                                    "I am always open to discussing new projects, creative ideas or professional opportunities."
                                </p>
                            </div>

                            <div class="flex flex-wrap items-center justify-center gap-6 md:gap-8">
                                <SocialLink href="https://github.com/MythEclipse" icon=view! { <GitHub/> } label="GitHub" />
                                <SocialLink href="https://www.linkedin.com/in/asep-haryana-saputra-2014a5294/" icon=view! { <LinkedIn/> } label="LinkedIn" />
                                <SocialLink href="https://www.instagram.com/asepharyana18/" icon=view! { <Instagram/> } label="Instagram" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn SocialLink(href: &'static str, icon: impl IntoView, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            class="group relative p-5 md:p-6 rounded-[1.5rem] md:rounded-3xl glass border border-border/10 hover:bg-muted font-display transition-all hover:scale-110 active:scale-95 shadow-xl"
        >
            <div class="absolute inset-0 bg-primary/10 rounded-3xl scale-0 group-hover:scale-110 transition-transform blur-xl" />
            <div class="relative z-10 w-6 h-6 flex items-center justify-center grayscale group-hover:grayscale-0 transition-all duration-500">
                {icon}
            </div>
            <span class="absolute -top-10 left-1/2 -translate-x-1/2 px-3 py-1 rounded-lg bg-primary text-[8px] font-black uppercase tracking-widest text-white opacity-0 group-hover:opacity-100 transition-all pointer-events-none">
                {label}
            </span>
        </a>
    }
}
