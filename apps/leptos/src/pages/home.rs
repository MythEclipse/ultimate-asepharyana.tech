use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use crate::components::logo::tech_icons::TECH_STACK;
use crate::components::logo::social_icons::{Instagram, LinkedIn, GitHub};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Identity Protocol | Asep Haryana"/>
        <main class="relative z-10 w-full overflow-hidden">
            // Hero Section: The Grand Reveal
            <section class="min-h-screen flex flex-col items-center justify-center px-6 md:px-12 py-32 relative group overflow-hidden">
                // Bevy Visuals Integration
                <iframe 
                    src="http://localhost:3001" 
                    class="absolute inset-0 w-full h-full border-0 -z-10 opacity-60 mix-blend-screen pointer-events-none"
                    title="Cyberpunk Particles"
                />

                <div class="absolute inset-0 opacity-20 pointer-events-none">
                    <div class="absolute top-1/4 left-1/4 w-[40rem] h-[40rem] bg-indigo-500/10 rounded-full blur-[120px] animate-pulse-slow" />
                </div>

                <div class="max-w-7xl mx-auto w-full flex flex-col items-center text-center space-y-20 relative z-10">
                    // Identity Tag
                    <div class="inline-flex items-center gap-4 px-6 py-2 rounded-full glass border border-white/10 shadow-2xl animate-fade-in hover-magnetic">
                        <span class="relative flex h-2 w-2">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-2 w-2 bg-indigo-500"></span>
                        </span>
                        <span class="text-[10px] font-black uppercase tracking-[0.5em] text-indigo-400 font-sans">
                            "Protocol: Full-Stack Architect"
                        </span>
                    </div>

                    <div class="space-y-8 animate-slide-up fill-mode-forwards">
                        <h1 class="text-6xl sm:text-7xl md:text-10xl font-black italic tracking-tighter leading-[0.9] uppercase font-display">
                            <span class="text-foreground/50 block translate-y-4 scale-y-95 tracking-[-0.05em] glitch" data-text="Developing">"Developing"</span>
                            <span class="gradient-text-animated block py-4 [text-shadow:0_20px_100px_rgba(99,102,241,0.5)]">
                                "Digital" <br/> "Destiny"
                            </span>
                        </h1>
                        <div class="max-w-3xl mx-auto space-y-6">
                            <p class="text-xl md:text-2xl text-muted-foreground/60 leading-relaxed font-medium italic font-sans">
                                "Synthesizing high-performance " <span class="text-indigo-400 font-black">"Rust"</span> 
                                " architecture with immersive " <span class="text-indigo-400 font-black">"Leptos"</span> 
                                " interfaces to bridge the void between concept and reality."
                            </p>
                            <div class="w-24 h-1 bg-gradient-to-r from-indigo-500 to-purple-500 mx-auto rounded-full" />
                        </div>
                    </div>

                    // Primary CTAs
                    <div class="flex flex-wrap items-center justify-center gap-8 animate-fade-in [animation-delay:400ms]">
                        <A href="/project" class="group relative px-12 py-6 rounded-3xl bg-foreground text-background font-black text-sm uppercase tracking-[0.3em] shadow-[0_30px_60px_rgba(0,0,0,0.4)] hover:scale-105 active:scale-95 transition-all duration-700 overflow-hidden hover-magnetic font-display">
                            <div class="absolute inset-0 bg-gradient-to-r from-indigo-500 to-purple-500 opacity-0 group-hover:opacity-20 transition-opacity" />
                            <span class="relative z-10 flex items-center gap-4">
                                "Open Archives"
                                <svg class="w-5 h-5 group-hover:translate-x-2 transition-transform duration-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                </svg>
                            </span>
                        </A>
                        
                        <a href="mailto:superaseph@gmail.com" class="px-12 py-6 rounded-3xl glass border border-white/10 text-foreground font-black text-sm uppercase tracking-[0.3em] hover:bg-white/5 hover:border-indigo-500/40 transition-all duration-500 hover:scale-105 hover-magnetic font-display">
                            "Secure Uplink"
                        </a>
                    </div>
                </div>

                // Neural Marquee: The Moving Text Protocol
                <div class="absolute bottom-0 w-full py-12 bg-black/5 border-y border-white/5 backdrop-blur-sm overflow-hidden rotate-[-2deg] scale-110">
                    <div class="flex whitespace-nowrap animate-marquee hover:[animation-play-state:paused]">
                        {(0..10).map(|_| view! {
                            <div class="flex items-center gap-12 px-6">
                                <span class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase text-foreground/5">"Architecture"</span>
                                <span class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase text-indigo-500/10">"Performance"</span>
                                <span class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase text-foreground/5">"Immersion"</span>
                                <span class="text-6xl md:text-8xl font-black italic tracking-tighter uppercase text-purple-500/10">"Protocol"</span>
                            </div>
                        }).collect_view()}
                    </div>
                </div>

                // Scroll Indicator
                <div class="absolute bottom-24 left-1/2 -translate-x-1/2 animate-bounce opacity-20">
                    <div class="w-6 h-10 rounded-full border-2 border-foreground flex justify-center p-2">
                        <div class="w-1.5 h-1.5 rounded-full bg-foreground animate-scroll-pill" />
                    </div>
                </div>
            </section>

            // The Arsenal: Technical Capabilities
            <section class="py-48 px-6 relative">
                 <div class="max-w-7xl mx-auto space-y-32">
                    <div class="text-center space-y-6">
                        <div class="inline-flex items-center gap-3 px-4 py-1 rounded-full glass border border-white/10 text-[8px] font-black uppercase tracking-[0.5em] text-indigo-400 mb-4">
                            "Infrastructure"
                        </div>
                        <h2 class="text-5xl md:text-7xl font-black italic tracking-tighter uppercase leading-none">
                            "My Tech " <span class="text-indigo-500">"Arsenal"</span>
                        </h2>
                        <div class="h-1.5 w-32 bg-gradient-to-r from-indigo-500 to-purple-500 mx-auto rounded-full" />
                    </div>

                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-8">
                        {TECH_STACK.into_iter().enumerate().map(|(i, skill)| {
                            let delay = format!("animation-delay: {}ms", 150 * i);
                            view! {
                                <div class="group relative animate-slide-up opacity-0 fill-mode-forwards" style=delay>
                                    <div class="absolute -inset-2 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-[2.5rem] opacity-0 group-hover:opacity-20 blur-2xl transition-all duration-700" />
                                    <div class="relative glass-card h-full p-10 rounded-[2.5rem] flex flex-col items-center justify-center gap-6 border border-white/5 hover:border-indigo-500/30 transition-all duration-700 hover:scale-[1.1] hover:-rotate-3 group shadow-2xl backdrop-blur-3xl overflow-hidden">
                                        <div class="absolute -right-10 -top-10 w-24 h-24 bg-white/5 rounded-full blur-2xl group-hover:bg-indigo-500/10 transition-colors" />
                                        <div class="w-20 h-20 relative flex items-center justify-center">
                                            <div class="absolute inset-0 bg-white/5 rounded-2xl blur-xl scale-0 group-hover:scale-150 transition-transform duration-700" />
                                            <img src=skill.image alt=skill.name class="w-14 h-14 relative z-10 drop-shadow-2xl brightness-90 group-hover:brightness-110 transition-all" />
                                        </div>
                                        <div class="space-y-1 text-center">
                                            <span class="text-[10px] font-black uppercase tracking-[0.3em] text-muted-foreground group-hover:text-indigo-400 transition-colors">
                                                {skill.name}
                                            </span>
                                            <div class="h-0.5 w-4 bg-indigo-500 mx-auto scale-x-0 group-hover:scale-x-100 transition-transform" />
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                 </div>
            </section>

            // Neural Connection Section
            <section class="py-48 px-6 overflow-hidden relative">
                <div class="absolute inset-0 pointer-events-none">
                    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full h-[1px] bg-gradient-to-r from-transparent via-indigo-500/20 to-transparent" />
                </div>

                <div class="max-w-7xl mx-auto rounded-[4rem] p-12 md:p-32 relative overflow-hidden glass border border-white/10 shadow-[0_100px_200px_rgba(0,0,0,0.5)]">
                    <div class="absolute -right-60 -top-60 w-[40rem] h-[40rem] bg-indigo-500/10 rounded-full blur-[150px] animate-tilt-slow opacity-60" />
                    <div class="absolute -left-60 -bottom-60 w-[40rem] h-[40rem] bg-purple-500/10 rounded-full blur-[150px] animate-tilt-reverse-slow opacity-40" />
                    
                    <div class="relative z-10 flex flex-col lg:flex-row items-center gap-24">
                        <div class="flex-1 space-y-12 text-center lg:text-left">
                            <div class="space-y-6">
                                <span class="px-6 py-2 rounded-full glass-subtle text-[10px] font-black uppercase tracking-[0.5em] text-indigo-400 shadow-xl">
                                    "Neural Uplink"
                                </span>
                                <h2 class="text-5xl md:text-8xl font-black italic tracking-tighter leading-none uppercase">
                                    "Let's Build " <br/> <span class="text-indigo-500">"The Future"</span>
                                </h2>
                                <p class="text-xl md:text-2xl text-muted-foreground/60 leading-relaxed max-w-xl mx-auto lg:mx-0 font-medium italic">
                                    "Ready to deploy innovative solutions into the digital ecosystem? Authenticate your request below."
                                </p>
                            </div>
                            
                            <div class="flex flex-wrap items-center justify-center lg:justify-start gap-6">
                                <SocialLink href="https://github.com/MythEclipse" icon=view! { <GitHub/> } label="GitHub" />
                                <SocialLink href="https://www.linkedin.com/in/asep-haryana-saputra-2014a5294/" icon=view! { <LinkedIn/> } label="LinkedIn" />
                                <SocialLink href="https://www.instagram.com/asepharyana18/" icon=view! { <Instagram/> } label="Instagram" />
                            </div>
                        </div>
                        
                        // Security Form Interface
                        <div class="flex-1 w-full max-w-xl relative group">
                            <div class="absolute -inset-4 bg-indigo-500/20 rounded-[3.5rem] blur-3xl opacity-0 group-hover:opacity-100 transition-opacity duration-1000" />
                            <div class="glass-card p-12 md:p-16 rounded-[3.5rem] border border-white/20 shadow-2xl relative z-10 transform scale-100 lg:group-hover:scale-105 lg:group-hover:-rotate-2 transition-all duration-700 overflow-hidden">
                                <div class="absolute top-0 right-0 p-8">
                                    <div class="w-3 h-3 rounded-full bg-indigo-500 animate-pulse" />
                                </div>
                                
                                <form class="space-y-10">
                                    <div class="space-y-4">
                                        <div class="flex items-center justify-between px-2">
                                            <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40">"Project ID / Title"</label>
                                            <span class="text-[8px] font-black text-indigo-500 tracking-widest uppercase">"Optional"</span>
                                        </div>
                                        <input type="text" placeholder="Initiating sequence..." class="w-full bg-white/2 border border-white/5 rounded-3xl p-6 focus:outline-none focus:border-indigo-500/50 transition-all font-bold text-lg placeholder:text-muted-foreground/20 italic" />
                                    </div>
                                    <div class="space-y-4">
                                        <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2">"Identification Portal"</label>
                                        <input type="email" placeholder="uplink@identity.io" class="w-full bg-white/2 border border-white/5 rounded-3xl p-6 focus:outline-none focus:border-indigo-500/50 transition-all font-bold text-lg placeholder:text-muted-foreground/20 italic" />
                                    </div>
                                    <button class="w-full py-7 rounded-[2rem] bg-foreground text-background font-black uppercase tracking-[0.5em] text-xs shadow-2xl hover:scale-95 active:scale-90 transition-all group/btn relative overflow-hidden">
                                        <span class="relative z-10 transition-transform group-hover/btn:scale-110 block">"Transmit Signal"</span>
                                        <div class="absolute inset-0 bg-gradient-to-r from-indigo-500 to-purple-500 opacity-0 group-hover/btn:opacity-20 transition-opacity" />
                                    </button>
                                </form>
                                
                                <div class="mt-8 pt-8 border-t border-white/5 text-center">
                                    <p class="text-[8px] font-black uppercase tracking-[0.5em] text-muted-foreground/20">"Secure End-to-End Encryption Active"</p>
                                </div>
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
