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
        <div class="bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800 rounded-2xl shadow-xl overflow-hidden border border-gray-200 dark:border-gray-700 animate-pulse">
            <div class="h-56 bg-gradient-to-br from-gray-300 to-gray-400 dark:from-gray-600 dark:to-gray-700" />
            <div class="p-6 space-y-4">
                <div class="h-7 bg-gray-300 dark:bg-gray-600 rounded-lg w-2/3" />
                <div class="space-y-2">
                    <div class="h-4 bg-gray-300 dark:bg-gray-600 rounded w-full" />
                    <div class="h-4 bg-gray-300 dark:bg-gray-600 rounded w-4/5" />
                </div>
            </div>
        </div>
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
        let is_external = is_external; // Capture
        let image_src = image_src; // Capture
        view! {
            <article class="relative bg-gradient-to-br from-white to-gray-50 dark:from-gray-800 dark:to-gray-900 rounded-2xl shadow-xl overflow-hidden border border-gray-200 dark:border-gray-700 hover:shadow-2xl hover:border-blue-400 dark:hover:border-blue-500 transition-all duration-500 transform group-hover:scale-[1.03] group-hover:-translate-y-1">
                 // Image Container with Overlay
                <div class="relative h-56 overflow-hidden bg-gradient-to-br from-blue-500 to-purple-600">
                    <img
                        src=image_src
                        alt=project.title
                        loading="lazy"
                        class="w-full h-full object-cover transition-all duration-700 group-hover:scale-110 group-hover:brightness-110"
                    />
                     // Gradient Overlay
                    <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-black/20 to-transparent opacity-60 group-hover:opacity-40 transition-opacity duration-300" />
                     // External badge
                    <Show when=move || is_external>
                        <div class="absolute top-3 right-3 px-2 py-1 bg-blue-500 text-white text-xs font-bold rounded-full flex items-center gap-1">
                             <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                            </svg>
                            "API"
                        </div>
                    </Show>
                </div>

                <div class="p-6 space-y-3">
                    <h3 class="text-2xl font-bold text-gray-900 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors duration-300 flex items-center gap-2">
                        {project.title}
                         <svg class="w-5 h-5 transform transition-transform duration-300 group-hover:translate-x-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                        </svg>
                    </h3>
                    <p class="text-gray-600 dark:text-gray-400 text-sm leading-relaxed">
                        {project.description}
                    </p>
                </div>
                 // Bottom Accent Line
                <div class="h-1 w-0 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 group-hover:w-full transition-all duration-500 ease-out" />
            </article>
        }
    };

    view! {
        <Show
            when=move || is_external
            fallback=move || view! {
                <a href=project.link_url class="block group">
                    {card_content()}
                </a>
            }
        >
             <a
                href=project.link_url
                target="_blank"
                rel="noopener noreferrer"
                class="block group"
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
        <Title text="Project Terbaru | Asepharyana"/>
        <main class="min-h-screen p-6 bg-background text-foreground">
            <div class="max-w-7xl mx-auto space-y-8">
                // Header
                <div class="flex items-center gap-4 mb-8 animate-fade-in">
                    <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center shadow-lg">
                        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                        </svg>
                    </div>
                    <div>
                        <h1 class="text-3xl md:text-4xl font-bold text-foreground">"Project Terbaru"</h1>
                        <p class="text-muted-foreground">"Berikut adalah kumpulan project yang saya buat"</p>
                    </div>
                </div>

                // Project Grid
                <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
                     <Show
                        when=move || mounted.get()
                        fallback=move || view! {
                            <CardSkeleton/>
                            <CardSkeleton/>
                            <CardSkeleton/>
                            <CardSkeleton/>
                            <CardSkeleton/>
                            <CardSkeleton/>
                        }
                    >
                        {PROJECTS.iter().map(|project| view! {
                            <ProjectCard project=project.clone()/>
                        }).collect_view()}
                    </Show>
                </div>
            </div>
        </main>
    }
}
