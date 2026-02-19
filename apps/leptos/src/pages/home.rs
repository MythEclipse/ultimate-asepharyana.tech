use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use crate::components::logo::tech_icons::TECH_STACK;
use crate::components::logo::social_icons::{Instagram, LinkedIn, GitHub};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Full-Stack Developer | Asep Haryana"/>
        <main class="relative z-10 w-full overflow-hidden">
            // Hero Section: Professional Identity
            <section class="min-h-screen flex flex-col items-center justify-center px-6 md:px-12 py-32 relative group overflow-hidden scanlines">
                // Bevy Visuals Integration
                <iframe 
                    src="http://localhost:3001" 
                    class="absolute inset-0 w-full h-full border-0 -z-10 opacity-60 mix-blend-screen pointer-events-none grayscale brightness-150"
                    title="Neural Particle Simulation"
                />

                <div class="absolute inset-0 opacity-40 pointer-events-none">
                    <div class="absolute top-1/4 left-1/4 w-[50rem] h-[50rem] bg-indigo-500/20 rounded-full blur-[160px] animate-pulse-slow" />
                    <div class="absolute bottom-1/4 right-1/4 w-[40rem] h-[40rem] bg-purple-500/10 rounded-full blur-[140px] animate-pulse-slow [animation-delay:2s]" />
                </div>

                <div class="max-w-7xl mx-auto w-full flex flex-col items-center text-center space-y-24 relative z-10">
                    // Signature Protocol
                    <div class="inline-flex items-center gap-4 px-6 py-2 rounded-full glass border border-white/10 shadow-3xl animate-fade-in hover-magnetic cyber-glow">
                        <span class="relative flex h-2 w-2">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-2 w-2 bg-indigo-500"></span>
                        </span>
                        <span class="text-[10px] font-black uppercase tracking-[0.6em] text-indigo-400 font-sans">
                            "Available for New Projects"
                        </span>
                    </div>

                    <div class="space-y-12 animate-slide-up fill-mode-forwards">
                        <h1 class="text-7xl sm:text-8xl md:text-11xl font-black italic tracking-tighter leading-[0.8] uppercase font-display">
                            <span class="text-foreground/80 block translate-y-8 scale-y-90 tracking-[-0.08em] glitch-heavy mb-8" data-text="Asep">
                                "Asep"
                                <div class="glitch-layer whitespace-nowrap" data-text="Asep"></div>
                            </span>
                            <div class="block py-8">
                                <span class="text-foreground/80 block glitch-heavy" data-text="Haryana Saputra">
                                    "Haryana Saputra"
                                    <div class="glitch-layer whitespace-nowrap" data-text="Haryana Saputra"></div>
                                </span>
                            </div>
                        </h1>
                        <div class="max-w-4xl mx-auto space-y-8">
                            <p class="text-2xl md:text-3xl text-muted-foreground/50 leading-relaxed font-medium italic font-sans tracking-tight">
                                "Crafting robust " <span class="text-indigo-400 font-black">"Backend"</span> 
                                " systems with high-performance " <span class="text-indigo-400 font-black">"Frontend"</span> 
                                " solutions to build seamless digital experiences."
                            </p>
                            <div class="flex items-center justify-center gap-4">
                                <div class="w-16 h-1 px-4 bg-muted/10 rounded-full" />
                                <div class="w-24 h-1.5 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full shadow-[0_0_20px_rgba(99,102,241,0.5)]" />
                                <div class="w-16 h-1 px-4 bg-muted/10 rounded-full" />
                            </div>
                        </div>
                    </div>

                    // Primary CTAs (Elite Variant)
                    <div class="flex flex-wrap items-center justify-center gap-10 animate-fade-in [animation-delay:400ms]">
                        <A href="/project" class="group relative px-16 py-8 rounded-[2.5rem] bg-foreground text-background font-black text-xs uppercase tracking-[0.4em] shadow-[0_50px_100px_rgba(0,0,0,0.6)] hover:scale-105 active:scale-95 transition-all duration-700 overflow-hidden industrial-snap font-display">
                            <div class="absolute inset-0 bg-gradient-to-r from-indigo-600 to-purple-600 opacity-0 group-hover:opacity-30 transition-opacity" />
                            <span class="relative z-10 flex items-center gap-6 chromatic-hover">
                                "View Portfolio"
                                <svg class="w-6 h-6 group-hover:translate-x-3 transition-transform duration-700" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                </svg>
                            </span>
                        </A>
                        
                        <a href="mailto:superaseph@gmail.com" class="px-16 py-8 rounded-[2.5rem] glass border border-white/10 text-foreground font-black text-xs uppercase tracking-[0.4em] hover:bg-white/5 hover:border-indigo-500/40 transition-all duration-700 hover:scale-105 industrial-snap font-display chromatic-hover">
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
                        <div class="w-2 h-2 rounded-full bg-indigo-500 animate-scroll-pill shadow-[0_0_10px_rgba(99,102,241,1)]" />
                    </div>
                </div>
            </section>

            // The Arsenal: Bento-Style Grid
            <section class="py-64 px-6 relative">
                 <div class="max-w-7xl mx-auto space-y-40">
                    <div class="text-center space-y-8">
                        <div class="inline-flex items-center gap-4 px-5 py-1.5 rounded-full glass border border-white/10 text-[9px] font-black uppercase tracking-[0.6em] text-indigo-400 mb-6 shadow-2xl">
                            "Professional Background"
                        </div>
                        <h2 class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase leading-none">
                            "Technical " <span class="text-indigo-500">"Stack"</span>
                        </h2>
                        <div class="h-2 w-48 bg-gradient-to-r from-indigo-500 via-purple-500 to-indigo-500 mx-auto rounded-full shadow-glow" />
                    </div>

                    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-8">
                        {TECH_STACK.into_iter().enumerate().map(|(i, skill)| {
                            let delay = format!("animation-delay: {}ms", 100 * i);
                            view! {
                                <div class="group relative animate-slide-up opacity-0 fill-mode-forwards" style=delay>
                                    <div class="absolute -inset-2 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-[3rem] opacity-0 group-hover:opacity-20 blur-3xl transition-all duration-1000" />
                                    <div class="relative glass-card h-full p-12 rounded-[3rem] flex flex-col items-center justify-center gap-8 border border-white/5 hover:border-indigo-500/40 transition-all duration-700 hover:scale-[1.15] hover:-rotate-2 group shadow-3xl backdrop-blur-4xl overflow-hidden">
                                        <div class="absolute -right-12 -top-12 w-32 h-32 bg-white/5 rounded-full blur-3xl group-hover:bg-indigo-500/20 transition-colors" />
                                        <div class="w-24 h-24 relative flex items-center justify-center">
                                            <div class="absolute inset-0 bg-white/5 rounded-3xl blur-2xl scale-0 group-hover:scale-175 transition-transform duration-1000" />
                                            <img src=skill.image alt=skill.name class="w-16 h-16 relative z-10 drop-shadow-33xl brightness-90 group-hover:brightness-110 transition-all filter group-hover:drop-shadow-[0_0_20px_rgba(255,255,255,0.4)]" />
                                        </div>
                                        <div class="space-y-2 text-center">
                                            <span class="text-[11px] font-black uppercase tracking-[0.4em] text-muted-foreground group-hover:text-indigo-400 transition-colors block">
                                                {skill.name}
                                            </span>
                                            <div class="h-0.5 w-6 bg-indigo-500 mx-auto scale-x-0 group-hover:scale-x-100 transition-transform duration-500" />
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                 </div>
            </section>

            // Featured Projects Preview
            <section class="py-64 px-6 bg-white/2 relative">
                <div class="max-w-7xl mx-auto space-y-40">
                    <div class="text-center space-y-8">
                        <div class="inline-flex items-center gap-4 px-5 py-1.5 rounded-full glass border border-white/10 text-[9px] font-black uppercase tracking-[0.6em] text-indigo-400 mb-6 shadow-2xl">
                            "Showcase"
                        </div>
                        <h2 class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase leading-none">
                            "Featured " <span class="text-indigo-500">"Projects"</span>
                        </h2>
                        <div class="h-2 w-48 bg-gradient-to-r from-indigo-500 via-purple-500 to-indigo-500 mx-auto rounded-full shadow-glow" />
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-12">
                         <div class="glass-card p-12 rounded-[3rem] border border-white/5 space-y-6 group hover:border-indigo-500/30 transition-all duration-700">
                            <div class="h-48 rounded-[2rem] bg-indigo-500/10 border border-white/5 overflow-hidden">
                                <img src="/public/project-rust.png" alt="Rust API" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-3xl font-black italic uppercase tracking-tighter">"Rust Infrastructure"</h3>
                                <p class="text-muted-foreground/60 text-sm font-medium leading-relaxed">"High-performance backend systems built with memory-safe Rust architecture."</p>
                            </div>
                         </div>
                         <div class="glass-card p-12 rounded-[3rem] border border-white/5 space-y-6 group hover:border-indigo-500/30 transition-all duration-700">
                            <div class="h-48 rounded-[2rem] bg-indigo-500/10 border border-white/5 overflow-hidden">
                                <img src="/public/project-elysia.png" alt="Elysia API" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-3xl font-black italic uppercase tracking-tighter">"Elysia Discovery"</h3>
                                <p class="text-muted-foreground/60 text-sm font-medium leading-relaxed">"Scalable API services featuring ultra-fast response times and full OpenAPI documentation."</p>
                            </div>
                         </div>
                         <div class="glass-card p-12 rounded-[3rem] border border-white/5 space-y-6 group hover:border-indigo-500/30 transition-all duration-700">
                            <div class="h-48 rounded-[2rem] bg-indigo-500/10 border border-white/5 overflow-hidden">
                                <img src="/public/project-anime.png" alt="Anime Streaming" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                            </div>
                            <div class="space-y-4">
                                <h3 class="text-3xl font-black italic uppercase tracking-tighter">"Media Ecosystem"</h3>
                                <p class="text-muted-foreground/60 text-sm font-medium leading-relaxed">"Cinematic frontend experiences designed for high-end content delivery and interaction."</p>
                            </div>
                         </div>
                    </div>

                    <div class="text-center">
                        <A href="/project" class="inline-flex items-center gap-4 px-12 py-6 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.4em] hover:bg-white/5 hover:border-indigo-500/40 transition-all">
                            "Explore Full Archive"
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </A>
                    </div>
                </div>
            </section>

            // Connection Section
            <section class="py-64 px-6 overflow-hidden relative">
                <div class="absolute inset-0 pointer-events-none">
                    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full h-[1px] bg-gradient-to-r from-transparent via-indigo-500/30 to-transparent" />
                </div>

                <div class="max-w-7xl mx-auto rounded-[5rem] p-12 md:p-32 relative overflow-hidden glass border border-white/10 shadow-[0_120px_250px_rgba(0,0,0,0.6)]">
                    <div class="absolute -right-60 -top-60 w-[50rem] h-[50rem] bg-indigo-500/10 rounded-full blur-[180px] animate-tilt-slow opacity-60" />
                    <div class="absolute -left-60 -bottom-60 w-[50rem] h-[50rem] bg-purple-500/10 rounded-full blur-[180px] animate-tilt-reverse-slow opacity-40" />
                    
                    <div class="relative z-10 flex flex-col items-center text-center space-y-24">
                        <div class="space-y-14">
                            <div class="space-y-8">
                                <span class="px-8 py-2.5 rounded-full glass-subtle text-[11px] font-black uppercase tracking-[0.6em] text-indigo-400 shadow-3xl">
                                    "Communication"
                                </span>
                                <h2 class="text-6xl md:text-9xl font-black italic tracking-tighter leading-none uppercase">
                                    "Get In " <br/> <span class="text-indigo-500">"Touch"</span>
                                </h2>
                                <p class="text-2xl md:text-3xl text-muted-foreground/50 leading-relaxed max-w-2xl mx-auto font-medium italic tracking-tight">
                                    "I am always open to discussing new projects, creative ideas or professional opportunities."
                                </p>
                            </div>
                            
                            <div class="flex flex-wrap items-center justify-center gap-8">
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
            class="group relative p-6 rounded-3xl glass border border-white/10 hover:bg-white/5 transition-all hover:scale-110 active:scale-95 shadow-2xl"
        >
            <div class="absolute inset-0 bg-indigo-500/10 rounded-3xl scale-0 group-hover:scale-110 transition-transform blur-xl" />
            <div class="relative z-10 w-6 h-6 flex items-center justify-center grayscale group-hover:grayscale-0 transition-all duration-500">
                {icon}
            </div>
            <span class="absolute -top-10 left-1/2 -translate-x-1/2 px-3 py-1 rounded-lg bg-indigo-500 text-[8px] font-black uppercase tracking-widest text-white opacity-0 group-hover:opacity-100 transition-all pointer-events-none">
                {label}
            </span>
        </a>
    }
}
