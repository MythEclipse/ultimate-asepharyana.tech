use leptos::*;
use crate::components::logo::tech_icons::TECH_STACK;
use crate::components::ui::{GlitchText, CachedImage};

#[component]
pub fn Arsenal() -> impl IntoView {
    view! {
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
    }
}
