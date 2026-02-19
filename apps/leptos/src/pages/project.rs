use leptos::*;
use leptos_meta::*;
use crate::providers::use_theme;

#[derive(Clone, Debug)]
struct Project {
    title: &'static str,
    description: &'static str,
    images: ProjectImages,
    link_url: &'static str,
}

#[derive(Clone, Debug)]
struct ProjectImages {
    light: &'static str,
    dark: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        title: "Elysia API",
        description: "API Backend menggunakan ElysiaJS dengan dokumentasi Swagger",
        images: ProjectImages { light: "/project-elysia.png", dark: "/project-elysia.png" },
        link_url: "https://elysia.asepharyana.tech/docs",
    },
    Project {
        title: "Rust API",
        description: "API performa tinggi menggunakan Rust dengan dokumentasi Swagger",
        images: ProjectImages { light: "/project-rust.png", dark: "/project-rust.png" },
        link_url: "https://ws.asepharyana.tech/docs",
    },
    Project {
        title: "Anime",
        description: "Nonton dan download anime dari otakudesu.cloud",
        images: ProjectImages { light: "/project-anime.png", dark: "/project-anime.png" },
        link_url: "/anime",
    },
    Project {
        title: "Anime2",
        description: "Nonton dan download anime dari alqanime.net",
        images: ProjectImages { light: "/project-anime2.png", dark: "/project-anime2.png" },
        link_url: "/anime2",
    },
    Project {
        title: "Komik",
        description: "Baca komik, manga, manhwa dari komikindo1.com",
        images: ProjectImages { light: "/project-komik.png", dark: "/project-komik.png" },
        link_url: "/komik",
    },
];

#[component]
fn CardSkeleton() -> impl IntoView {
    view! {
        <div class="aspect-video lg:aspect-[3/4.2] rounded-[2.5rem] bg-white/5 animate-pulse border border-white/5" />
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    let theme = use_theme();
    let is_external = project.link_url.starts_with("http");
    let image_src = move || {
         if theme.theme.get() == crate::providers::Theme::Light {
            project.images.light
         } else {
            project.images.dark
         }
    };

    let card_content = move || {
        let is_external_val = is_external;
        let img_src = image_src();
        view! {
            <div class="relative group/card perspective-1000 h-full">
                <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 rounded-[2.5rem] opacity-20 blur-2xl group-hover/card:opacity-40 transition-opacity duration-700" />
                
                <article class="relative h-full flex flex-col bg-muted/30 rounded-[2.5rem] overflow-hidden border border-white/10 shadow-2xl transition-all duration-700 hover-tilt group-hover/card:border-white/20">
                    // Image Container with 3D Parallax
                    <div class="relative h-64 overflow-hidden">
                        <img
                            src=img_src
                            alt=project.title
                            loading="lazy"
                            class="w-full h-full object-cover transition-transform duration-1000 ease-out group-hover/card:scale-115"
                        />
                        
                        // Glassy Overlay
                        <div class="absolute inset-0 bg-gradient-to-t from-black via-black/20 to-transparent opacity-80 group-hover/card:opacity-60 transition-opacity duration-500" />
                        
                        // External Badge
                        <Show when=move || is_external_val>
                            <div class="absolute top-6 right-6 glass px-3 py-1.5 rounded-xl border border-white/20 text-[10px] font-black uppercase tracking-widest text-blue-400 flex items-center gap-2 shadow-2xl">
                                <span class="w-1.5 h-1.5 rounded-full bg-blue-400 animate-pulse" />
                                "External Link"
                            </div>
                        </Show>
                    </div>

                    <div class="flex-1 p-8 flex flex-col justify-between space-y-4 relative z-10">
                        <div class="space-y-3">
                            <h3 class="text-3xl font-black italic tracking-tighter uppercase leading-none group-hover/card:text-blue-400 transition-colors">
                                {project.title}
                            </h3>
                            <p class="text-muted-foreground/80 text-sm font-medium leading-relaxed line-clamp-3">
                                {project.description}
                            </p>
                        </div>
                        
                        <div class="flex items-center gap-3 pt-4 border-t border-white/5">
                            <span class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/40 group-hover/card:text-blue-500 transition-colors">"Analyze Entry"</span>
                            <svg class="w-5 h-5 transform transition-transform duration-500 group-hover/card:translate-x-2 text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </div>
                    </div>

                    // Interaction Glow
                    <div class="absolute inset-0 opacity-0 group-hover/card:opacity-100 bg-gradient-to-tr from-blue-500/10 via-transparent to-purple-500/10 transition-opacity duration-700 pointer-events-none" />
                </article>
            </div>
        }
    };

    view! {
        <Show
            when=move || is_external
            fallback=move || view! {
                <a href=project.link_url class="block group animate-slide-up opacity-0 fill-mode-forwards" style="animation-duration: 0.8s">
                    {card_content()}
                </a>
            }
        >
             <a
                href=project.link_url
                target="_blank"
                rel="noopener noreferrer"
                class="block group animate-slide-up opacity-0 fill-mode-forwards"
                style="animation-duration: 0.8s"
            >
                {card_content()}
            </a>
        </Show>
    }
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let (mounted, set_mounted) = create_signal(false);
    
    create_effect(move |_| {
        set_mounted.set(true);
    });

    view! {
        <Title text="Archive | Portfolio Showcase"/>
        <main class="min-h-screen relative overflow-hidden pb-40">
            // Background Orbs
            <div class="fixed inset-0 pointer-events-none z-0">
                <div class="absolute top-[10%] left-[5%] w-[40rem] h-[40rem] bg-blue-500/10 rounded-full blur-[120px] animate-tilt" />
                <div class="absolute bottom-[20%] right-[5%] w-[35rem] h-[35rem] bg-purple-500/10 rounded-full blur-[120px] animate-tilt-reverse" />
            </div>

            <div class="max-w-7xl mx-auto px-6 py-24 space-y-32 relative z-10">
                // Cinematic Header
                <header class="text-center space-y-12 animate-fade-in">
                    <div class="space-y-6">
                        <div class="inline-flex items-center gap-3 px-4 py-2 rounded-full glass border border-white/10 shadow-2xl">
                             <div class="w-2 h-2 rounded-full bg-blue-500 animate-pulse" />
                            <span class="text-[10px] font-black uppercase tracking-[0.2em] text-blue-500">"Production Archives"</span>
                        </div>
                        <h1 class="text-6xl md:text-9xl font-black tracking-tighter uppercase italic line-height-1 mt-4">
                            <span class="bg-gradient-to-r from-blue-400 via-purple-500 to-pink-400 bg-clip-text text-transparent animate-gradient-x bg-[length:200%_auto]">
                                "Creative"
                            </span>
                            <span class="text-foreground/20 block translate-y-[-0.5em] scale-y-75 uppercase">"Vault"</span>
                        </h1>
                        <p class="max-w-2xl mx-auto text-muted-foreground/60 text-lg font-medium tracking-tight">
                            "A collection of engineering prototypes and digital artifacts built with "
                            <span class="text-white font-black">"Rust, Leptos, and High-Performance"</span>
                            " logic."
                        </p>
                    </div>

                    <div class="flex flex-col md:flex-row items-center justify-center gap-6 animate-fade-in [animation-delay:200ms]">
                        <div class="h-px w-20 bg-gradient-to-r from-transparent to-white/20 hidden md:block" />
                        <div class="flex items-center gap-4 text-xs font-black uppercase tracking-[0.4em] text-muted-foreground/40">
                            "Total Deployments"
                            <span class="text-white px-3 py-1 bg-white/5 rounded-lg border border-white/10">{PROJECTS.len()}</span>
                        </div>
                        <div class="h-px w-20 bg-gradient-to-l from-transparent to-white/20 hidden md:block" />
                    </div>
                </header>

                // Project Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-12">
                     <Show
                        when=move || mounted.get()
                        fallback=move || view! {
                            <CardSkeleton/>
                            <CardSkeleton/>
                            <CardSkeleton/>
                        }
                    >
                        {PROJECTS.iter().enumerate().map(|(i, project)| view! {
                            <div style=format!("animation-delay: {}ms", i * 150)>
                                <ProjectCard project=project.clone()/>
                            </div>
                        }).collect_view()}
                    </Show>
                </div>
            </div>
        </main>
    }
}
