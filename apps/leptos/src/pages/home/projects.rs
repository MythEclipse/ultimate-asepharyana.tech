use leptos::*;
use leptos_router::A;
use crate::components::ui::GlitchText;
use crate::data::projects::FEATURED_PROJECTS;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
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
                    {FEATURED_PROJECTS.iter().map(|project| view! {
                        <div class="group relative">
                            <div class="absolute -inset-px rounded-[2.5rem] md:rounded-[3rem] bg-gradient-to-br from-primary/40 via-accent/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-700 blur-xl pointer-events-none" />
                            <div class="relative glass-card rounded-[2.5rem] md:rounded-[3rem] border border-border/10 group-hover:border-primary/50 transition-all duration-700 overflow-hidden flex flex-col">
                                <div class="relative h-52 md:h-60 overflow-hidden">
                                    <img src=project.image alt=project.title class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-1000" />
                                    <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />
                                    <span class="absolute top-4 left-4 px-3 py-1 rounded-full bg-black/60 backdrop-blur-sm border border-white/10 text-[10px] font-black uppercase tracking-[0.35em] text-primary">
                                        {project.id}
                                    </span>
                                    <span class="absolute top-4 right-4 px-3 py-1 rounded-full bg-primary/20 backdrop-blur-sm border border-primary/30 text-[9px] font-black uppercase tracking-[0.3em] text-primary">
                                        {project.category}
                                    </span>
                                </div>
                                <div class="p-8 md:p-10 flex flex-col gap-5 flex-1">
                                    <div class="space-y-2">
                                        <h3 class=format!("text-2xl md:text-3xl font-black italic uppercase tracking-tighter leading-tight group-hover:{} transition-colors duration-500", project.color_class)>
                                            {project.title}
                                        </h3>
                                        <p class="text-muted-foreground/80 text-sm font-medium leading-relaxed">
                                            {project.description}
                                        </p>
                                    </div>
                                    <div class="flex flex-wrap gap-2">
                                        {project.tags.iter().map(|tag| view! {
                                            <span class="px-2.5 py-1 rounded-lg bg-muted/40 border border-border/20 text-[9px] font-black uppercase tracking-[0.25em] text-muted-foreground">
                                                {tag.to_string()}
                                            </span>
                                        }).collect_view()}
                                    </div>
                                    <div class="mt-auto pt-4 border-t border-border/10">
                                        <A href=project.link class=format!("inline-flex items-center gap-2 text-[10px] font-black uppercase tracking-[0.35em] {} group-hover:gap-4 transition-all duration-500", project.color_class)>
                                            "View Project"
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                            </svg>
                                        </A>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }).collect_view()}
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
    }
}
