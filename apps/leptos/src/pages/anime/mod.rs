pub mod detail;
pub mod search;
pub mod watch;
use leptos::*;
use leptos_meta::*;
use serde::{Serialize, Deserialize};
use crate::api::anime::{fetch_ongoing_anime, fetch_complete_anime};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: Option<String>,
    pub episode_count: Option<String>,
    pub score: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomeData {
    pub ongoing_anime: Vec<AnimeItem>,
    pub complete_anime: Vec<AnimeItem>,
}

async fn fetch_anime_data() -> Option<HomeData> {
    let ongoing_res = fetch_ongoing_anime(1).await;
    let complete_res = fetch_complete_anime(1).await;


    let ongoing_anime = match ongoing_res {
        Ok((data, _)) => data.into_iter().map(|item| AnimeItem {
            title: item.title,
            slug: item.slug,
            poster: item.poster,
            current_episode: None, // Endpoint has score, not episode
            episode_count: None,
            score: Some(item.score),
        }).collect(),
        Err(_) => vec![],
    };

    let complete_anime = match complete_res {
        Ok((data, _)) => data.into_iter().map(|item| AnimeItem {
            title: item.title,
            slug: item.slug,
            poster: item.poster,
            current_episode: None,
            episode_count: Some(item.episode_count),
            score: None,
        }).collect(),
        Err(_) => vec![],
    };

    Some(HomeData {
        ongoing_anime,
        complete_anime,
    })
}

#[component]
fn AnimeCard(item: AnimeItem, index: usize) -> impl IntoView {
    let delay_style = format!("animation-delay: {}s", index as f64 * 0.06);
    let has_episode = item.current_episode.is_some();
    let current_episode_text = item.current_episode.clone();
    let has_score = item.score.is_some();
    let score_text = item.score.clone();
    let has_count = item.episode_count.is_some();
    let count_text = item.episode_count.clone();

    view! {
        <div 
            class="group perspective-1000 animate-fade-in"
            style=delay_style
        >
             <a
                href=format!("/anime2/detail/{}", item.slug)
                class="block relative overflow-hidden rounded-2xl bg-card border border-border shadow-lg hover:shadow-2xl hover:shadow-blue-500/20 transition-all duration-500 transform-gpu hover:-translate-y-4 hover:rotate-1"
            >
                // ... (Visual effects preserved) ...
                <div class="absolute -inset-[2px] rounded-2xl bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10 blur-sm animate-gradient-rotate" />
                <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-700 bg-gradient-to-br from-blue-500/30 via-transparent to-purple-500/30 blur-2xl" />

                <div class="aspect-[3/4] overflow-hidden relative bg-gradient-to-br from-blue-900/20 to-purple-900/20">
                    <img
                        src=item.poster
                        alt=item.title.clone()
                        class="w-full h-full object-cover transform-gpu group-hover:scale-115 group-hover:rotate-2 transition-all duration-700 ease-out"
                        loading="lazy"
                    />
                    <div class="absolute inset-0 bg-gradient-to-tr from-transparent via-white/30 to-transparent opacity-0 group-hover:opacity-100 -translate-x-full group-hover:translate-x-full transition-all duration-1000 ease-out" />
                    <div class="absolute inset-0 bg-gradient-to-bl from-blue-500/20 via-transparent to-purple-500/20 opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                </div>

                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/50 to-transparent opacity-80" />

                <div class="absolute bottom-0 left-0 right-0 p-4">
                    <h3 class="text-white text-sm font-bold line-clamp-2 drop-shadow-lg mb-2 group-hover:text-blue-200 transition-colors duration-300">
                        {item.title}
                    </h3>
                    
                    <div class="flex flex-wrap gap-2">
                        <Show when=move || has_episode>
                             <span class="inline-flex items-center gap-1.5 text-xs font-bold px-3 py-1 rounded-full bg-gradient-to-r from-blue-500 to-purple-500 text-white shadow-lg">
                                <span class="w-2 h-2 rounded-full bg-white animate-pulse" />
                                {current_episode_text.clone()}
                            </span>
                        </Show>
                        <Show when=move || has_score>
                             <span class="inline-flex items-center gap-1.5 text-xs font-bold px-3 py-1 rounded-full bg-yellow-500/90 text-black shadow-lg">
                                "⭐ " {score_text.clone()}
                            </span>
                        </Show>
                        <Show when=move || has_count>
                             <span class="inline-flex items-center gap-1.5 text-xs font-bold px-3 py-1 rounded-full bg-green-500/90 text-white shadow-lg">
                                "📺 " {count_text.clone()}
                            </span>
                        </Show>
                    </div>
                </div>
            </a>
        </div>
    }
}

// ... AnimeGrid and SectionHeader preserved ...

#[component]
fn AnimeGrid(items: Vec<AnimeItem>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6">
            {items.into_iter().enumerate().map(|(i, item)| view! { <AnimeCard item=item index=i/> }).collect_view()}
        </div>
    }
}

#[component]
fn SectionHeader(
    title: &'static str,
    icon: &'static str,
    gradient: &'static str,
    link: &'static str,
    link_gradient: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between mb-10 animate-slide-in-right">
            <div class="flex items-center gap-5">
                <div class=format!("p-5 rounded-3xl bg-gradient-to-br {} shadow-2xl relative overflow-hidden group", gradient)>
                    <div class="absolute inset-0 bg-white/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                    <span class="text-3xl relative z-10">{icon}</span>
                </div>
                <h2 class="text-2xl md:text-4xl font-black text-foreground">
                    {title}
                </h2>
            </div>
             <a
                href=link
                class=format!("group relative overflow-hidden flex items-center gap-2 px-6 py-3 rounded-2xl bg-gradient-to-r {} text-white font-bold shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105", link_gradient)
            >
                <span class="relative z-10">"View All"</span>
                <svg class="w-5 h-5 relative z-10 transform group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                </svg>
                <div class="absolute inset-0 bg-white/20 translate-x-[-100%] group-hover:translate-x-0 transition-transform duration-500" />
            </a>
        </div>
    }
}

#[component]
pub fn AnimePage() -> impl IntoView {
    let data = create_resource(|| (), |_| fetch_anime_data());

    view! {
        <Title text="Anime | Asepharyana"/>
        <main class="min-h-screen bg-background text-foreground overflow-hidden relative">
            // Animated background
             <div class="fixed inset-0 -z-10 overflow-hidden">
                <div class="absolute top-[-10%] left-[-5%] w-[500px] h-[500px] bg-blue-500/15 rounded-full blur-3xl animate-float-slow" />
                <div class="absolute bottom-[-10%] right-[-5%] w-[600px] h-[600px] bg-purple-500/15 rounded-full blur-3xl animate-float-medium" />
                <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[400px] h-[400px] bg-pink-500/10 rounded-full blur-3xl animate-float-fast" />
                <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,.02)_1px,transparent_1px)] bg-[size:50px_50px]" />
            </div>

            <div class="p-4 md:p-8 lg:p-12 relative">
                <div class="max-w-7xl mx-auto">
                    // Hero Header
                    <div class="text-center mb-16 perspective-1000 animate-fade-in">
                         <div class="inline-block mb-6">
                            <span class="px-6 py-2 rounded-full bg-gradient-to-r from-blue-500/20 to-purple-500/20 border border-blue-500/30 text-blue-400 font-medium text-sm">
                                "✨ Otakudesu Source"
                            </span>
                        </div>
                        <h1 class="text-6xl md:text-8xl font-black mb-6">
                             <span class="bg-gradient-to-r from-blue-400 via-purple-500 to-pink-400 bg-clip-text text-transparent animate-gradient-x bg-[length:200%_auto] drop-shadow-2xl">
                                "Anime"
                            </span>
                        </h1>
                        <p class="text-muted-foreground text-xl max-w-md mx-auto">
                            "Streaming anime dari Otakudesu dengan kualitas terbaik"
                        </p>
                    </div>

                    // Search Bar
                    <div class="mb-16 animate-fade-in">
                         <form action="/anime/search" method="get" class="flex gap-4 max-w-3xl mx-auto">
                            <div class="relative flex-1 group">
                                <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 rounded-3xl opacity-50 group-focus-within:opacity-100 blur-lg transition-all duration-500" />
                                <input
                                    type="text"
                                    name="q"
                                    placeholder="🔍 Cari anime favoritmu..."
                                    class="relative w-full px-8 py-5 rounded-2xl border-2 border-white/10 bg-background/80 backdrop-blur-xl focus:outline-none focus:border-blue-500 transition-all duration-300 text-lg font-medium"
                                />
                            </div>
                            <button
                                type="submit"
                                class="px-10 py-5 rounded-2xl bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 text-white font-bold text-lg shadow-2xl shadow-blue-500/40 hover:shadow-purple-500/50 hover:scale-110 hover:rotate-2 active:scale-95 transition-all duration-300 relative overflow-hidden"
                            >
                                <span class="relative z-10">"Search"</span>
                                <div class="absolute inset-0 bg-white/20 translate-y-full hover:translate-y-0 transition-transform duration-300" />
                            </button>
                        </form>
                    </div>

                    <Suspense fallback=move || view! { <div class="text-center">"Loading..."</div> }>
                        <Show when=move || data.get().flatten().is_some() fallback=move || view! { <div>"Failed to load"</div> }>
                            {move || {
                                let d = data.get().flatten().unwrap();
                                view! {
                                    <div class="space-y-24">
                                        <section>
                                            <SectionHeader
                                                title="Ongoing Anime"
                                                icon="🔥"
                                                gradient="from-blue-500 to-purple-500"
                                                link="/anime/ongoing-anime/1"
                                                link_gradient="from-blue-500 to-purple-500"
                                            />
                                            <AnimeGrid items=d.ongoing_anime/>
                                        </section>

                                        <section>
                                            <SectionHeader
                                                title="Complete Anime"
                                                icon="✨"
                                                gradient="from-green-500 to-emerald-500"
                                                link="/anime/complete-anime/1"
                                                link_gradient="from-green-500 to-emerald-500"
                                            />
                                            <AnimeGrid items=d.complete_anime/>
                                        </section>
                                    </div>
                                }
                            }}
                        </Show>
                    </Suspense>
                </div>
            </div>
        </main>
    }
}

