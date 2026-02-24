use leptos::*;
use leptos_meta::*;
use crate::providers::use_theme;
use crate::components::ui::GlitchText;

#[derive(Clone, Debug)]
struct Project {
    title: &'static str,
    description: &'static str,
    images: ProjectImages,
    link_url: &'static str,
    tag: &'static str,
}

#[derive(Clone, Debug)]
struct ProjectImages {
    light: &'static str,
    dark: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        title: "Elysia API",
        tag: "Backend",
        description: "A robust backend API implementation using ElysiaJS, optimized for speed and developer experience with full OpenAPI support.",
        images: ProjectImages { light: "/public/project-elysia.png", dark: "/public/project-elysia.png" },
        link_url: "https://elysia.asepharyana.tech/docs",
    },
    Project {
        title: "Rust API",
        tag: "Backend",
        description: "High-performance, memory-safe backend infrastructure developed with Rust and Axum, featuring interactive API documentation.",
        images: ProjectImages { light: "/public/project-rust.png", dark: "/public/project-rust.png" },
        link_url: "https://ws.asepharyana.tech/docs",
    },
    Project {
        title: "Anime",
        tag: "Otakudesu",
        description: "High-performance anime streaming engine, leveraging real-time data extraction and aggregation from Otakudesu.",
        images: ProjectImages { light: "/public/project-anime.png", dark: "/public/project-anime.png" },
        link_url: "/anime",
    },
    Project {
        title: "Anime2",
        tag: "Alqanime",
        description: "Next-generation digital anime archive, featuring intelligent content scraping and parsing from Alqanime.",
        images: ProjectImages { light: "/public/project-anime2.png", dark: "/public/project-anime2.png" },
        link_url: "/anime2",
    },
    Project {
        title: "Komik",
        tag: "Komiku",
        description: "Immersive, high-resolution manga and comic reader, seamlessly indexing chapters from Komiku.",
        images: ProjectImages { light: "/public/project-komik.png", dark: "/public/project-komik.png" },
        link_url: "/komik",
    },
];

#[component]
fn CardSkeleton() -> impl IntoView {
    view! {
        <div class="w-full rounded-2xl bg-white/5 animate-pulse border border-white/5 h-[420px]" />
    }
}

#[component]
fn ProjectCard(project: Project, delay_ms: usize) -> impl IntoView {
    let theme = use_theme();
    let is_external = project.link_url.starts_with("http");
    let image_src = move || {
        if theme.theme.get() == crate::providers::Theme::Light {
            project.images.light
        } else {
            project.images.dark
        }
    };

    let inner = move || {
        let img_src = image_src();
        view! {
            // Glow halo behind card
            <div class="absolute -inset-px bg-gradient-to-br from-cyan-500/30 via-blue-500/20 to-purple-500/30 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500 blur-sm -z-10" />

            <article class="relative flex flex-col h-full bg-card/70 rounded-2xl overflow-hidden border border-white/8 shadow-2xl transition-all duration-500 hover:-translate-y-1 group-hover:border-cyan-500/40 backdrop-blur-sm">
                // ── Image ──────────────────────────────────────────────
                <div class="relative h-52 shrink-0 overflow-hidden">
                    <img
                        src=img_src
                        alt=project.title
                        loading="lazy"
                        class="w-full h-full object-cover transition-transform duration-700 ease-out group-hover:scale-105 opacity-80 group-hover:opacity-100"
                    />
                    // Dark fade to card body
                    <div class="absolute inset-0 bg-gradient-to-t from-card/95 via-card/30 to-transparent pointer-events-none" />

                    // Source/External badge — top-right
                    <div class="absolute top-4 right-4 flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-background/50 backdrop-blur-md border border-primary/20 text-[9px] font-black uppercase tracking-widest text-primary">
                        <span class="w-1 h-1 rounded-full bg-primary animate-pulse" />
                        {project.tag}
                    </div>
                </div>

                // ── Body ───────────────────────────────────────────────
                <div class="flex flex-col flex-1 p-6 gap-4">
                    <div class="flex flex-col gap-1.5 flex-1">
                        <h3 class="text-2xl font-black uppercase tracking-tighter text-foreground group-hover:text-cyan-300 transition-colors duration-300 chromatic-hover">
                            {project.title}
                        </h3>
                        <p class="text-sm text-muted-foreground/70 leading-relaxed">
                            {project.description}
                        </p>
                    </div>

                    // Footer CTA
                    <div class="relative flex items-center gap-2 pt-4 border-t border-white/8 mt-auto">
                        <div class="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-cyan-500/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                        <span class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/40 group-hover:text-cyan-400 transition-colors">
                            {if is_external { "View Docs" } else { "Explore" }}
                        </span>
                        <svg class="w-4 h-4 text-primary transition-transform duration-300 group-hover:translate-x-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                        </svg>
                    </div>
                </div>

                // Hover glow overlay
                <div class="absolute inset-0 bg-gradient-to-tr from-cyan-500/5 via-transparent to-purple-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none" />
            </article>
        }
    };

    let style = format!("animation-delay: {}ms", delay_ms);

    view! {
        <Show
            when=move || is_external
            fallback={
                let style_fb = style.clone();
                move || view! {
                    <a
                        href=project.link_url
                        class="relative block h-full group animate-slide-up"
                        style=style_fb.clone()
                    >
                        {inner()}
                    </a>
                }
            }
        >
            <a
                href=project.link_url
                target="_blank"
                rel="noopener noreferrer"
                class="relative block h-full group animate-slide-up"
                style=style.clone()
            >
                {inner()}
            </a>
        </Show>
    }
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let (mounted, set_mounted) = create_signal(false);
    create_effect(move |_| { set_mounted.set(true); });

    view! {
        <Title text="Projects | Portfolio Showcase"/>
        <main class="min-h-screen relative overflow-hidden pb-40">
            // ── Ambient background orbs ─────────────────────────────────
            <div class="fixed inset-0 pointer-events-none -z-10">
                <div class="absolute top-[8%] left-[4%]  w-[36rem] h-[36rem] bg-cyan-500/6   rounded-full blur-[140px] animate-pulse-slow" />
                <div class="absolute top-[55%] right-[4%] w-[32rem] h-[32rem] bg-purple-500/6 rounded-full blur-[140px] animate-pulse-slow" style="animation-delay:-5s" />
                <div class="absolute top-[35%] left-[40%] w-[24rem] h-[24rem] bg-blue-500/6   rounded-full blur-[120px] animate-pulse-slow" style="animation-delay:-2.5s" />
            </div>

            <div class="max-w-7xl mx-auto px-6 py-24 space-y-24">
                // ── Header ──────────────────────────────────────────────
                <header class="flex flex-col items-center text-center space-y-8">
                    // Pill badge
                    <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-cyan-500/25 bg-cyan-500/5 text-[10px] font-black uppercase tracking-[0.25em] text-cyan-400">
                        <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse" />
                        "Data Archive · Selected Works"
                    </div>

                    // Title — block-level so everything stacks and centers naturally
                    <div class="space-y-2">
                        <h1 class="text-7xl md:text-[9rem] font-black tracking-tighter uppercase leading-[0.9] text-center">
                            <GlitchText
                                text="Project"
                                class="bg-gradient-to-r from-primary via-accent to-primary bg-clip-text text-transparent"
                            />
                        </h1>
                        <p class="text-3xl md:text-4xl font-black uppercase tracking-[0.3em] text-foreground/10 text-center">
                            "Showcase"
                        </p>
                    </div>

                    <p class="max-w-xl text-muted-foreground/60 text-base font-medium leading-relaxed">
                        "A curated collection of software engineering projects built with"
                        <span class="text-cyan-400 font-bold"> " Rust" </span>
                        "and modern"
                        <span class="text-purple-400 font-bold"> " Frontend" </span>
                        "technologies."
                    </p>

                    // Stat strip
                    <div class="flex items-center gap-4 text-xs font-black uppercase tracking-[0.35em] text-muted-foreground/40">
                        <div class="h-px w-16 bg-gradient-to-r from-transparent to-border/50" />
                        "Total"
                        <span class="text-foreground/80 px-3 py-1 bg-muted/50 rounded-md border border-border/50">{PROJECTS.len()}</span>
                        "Projects"
                        <div class="h-px w-16 bg-gradient-to-l from-transparent to-border/50" />
                    </div>
                </header>

                // ── Grid ─────────────────────────────────────────────────
                // 5 cards: use 2-col on md so rows are even (3+2→2x2+1 centered)
                // On lg: 3-col with last row centered via justify-items-center on grid
                <Show
                    when=move || mounted.get()
                    fallback=move || view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <CardSkeleton/> <CardSkeleton/> <CardSkeleton/>
                            <CardSkeleton/> <CardSkeleton/>
                        </div>
                    }
                >
                    // Wrap in a relative container to center orphan card in the last row on lg
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 [&>*]:h-full">
                        {PROJECTS.iter().enumerate().map(|(i, project)| view! {
                            <ProjectCard project=project.clone() delay_ms=i * 120 />
                        }).collect_view()}
                    </div>
                </Show>
            </div>
        </main>
    }
}
