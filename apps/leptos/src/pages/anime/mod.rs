pub mod detail;
pub mod search;
pub mod watch;
use leptos::*;
use leptos_meta::*;
use serde::{Serialize, Deserialize};
use crate::api::anime::{
    fetch_anime1_index, fetch_anime2_index
};

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

thread_local! {
    static ANIME1_CACHE: std::cell::RefCell<Option<HomeData>> = std::cell::RefCell::new(None);
    static ANIME2_CACHE: std::cell::RefCell<Option<HomeData>> = std::cell::RefCell::new(None);
}

async fn fetch_anime_data(source: u8) -> Option<HomeData> {
    #[cfg(feature = "csr")]
    {
        let cached = if source == 2 {
            ANIME2_CACHE.with(|cache| cache.borrow().clone())
        } else {
            ANIME1_CACHE.with(|cache| cache.borrow().clone())
        };
        if cached.is_some() {
            return cached;
        }
    }

    if source == 2 {
        let data = fetch_anime2_index().await.ok()?;
        let mapped = HomeData {
            ongoing_anime: data.ongoing_anime.into_iter().map(|item| AnimeItem {
                title: item.title,
                slug: item.slug,
                poster: item.poster,
                current_episode: Some(item.current_episode),
                episode_count: None,
                score: None,
            }).collect(),
            complete_anime: data.complete_anime.into_iter().map(|item| AnimeItem {
                title: item.title,
                slug: item.slug,
                poster: item.poster,
                current_episode: None,
                episode_count: Some(item.episode_count),
                score: None,
            }).collect(),
        };
        #[cfg(feature = "csr")]
        ANIME2_CACHE.with(|cache| *cache.borrow_mut() = Some(mapped.clone()));
        Some(mapped)
    } else {
        let data = fetch_anime1_index().await.ok()?;
        let mapped = HomeData {
            ongoing_anime: data.ongoing_anime.into_iter().map(|item| AnimeItem {
                title: item.title,
                slug: item.slug,
                poster: item.poster,
                current_episode: Some(item.current_episode),
                episode_count: None,
                score: None,
            }).collect(),
            complete_anime: data.complete_anime.into_iter().map(|item| AnimeItem {
                title: item.title,
                slug: item.slug,
                poster: item.poster,
                current_episode: None,
                episode_count: Some(item.episode_count),
                score: None,
            }).collect(),
        };
        #[cfg(feature = "csr")]
        ANIME1_CACHE.with(|cache| *cache.borrow_mut() = Some(mapped.clone()));
        Some(mapped)
    }
}

#[component]
fn AnimeCard(item: AnimeItem, index: usize, source: u8) -> impl IntoView {
    let delay_style = format!("animation-delay: {}ms", index * 50);
    let has_episode = item.current_episode.is_some();
    let current_episode_text = item.current_episode.clone();
    let has_score = item.score.is_some();
    let score_text = item.score.clone();
    let has_count = item.episode_count.is_some();
    let count_text = item.episode_count.clone();

    let prefix = if source == 2 { "anime2" } else { "anime" };

    view! {
        <div 
            class="group animate-slide-up opacity-0 fill-mode-forwards"
            style=delay_style
        >
             <a
                href=format!("/{}/detail/{}", prefix, item.slug)
                class="block relative group/card perspective-1000"
            >
                <div class="relative aspect-[3/4.2] rounded-[2rem] overflow-hidden bg-muted border border-white/5 shadow-2xl transition-all duration-700 hover-tilt group-hover:shadow-blue-500/20 group-hover:border-white/20">
                    // Poster with parallax-like zoom
                    <img
                        src=item.poster
                        alt=item.title.clone()
                        class="w-full h-full object-cover transition-transform duration-1000 ease-out group-hover:scale-115"
                        loading="lazy"
                    />
                    
                    // Glassy Overlay for Info
                    <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/20 to-transparent opacity-60 group-hover:opacity-40 transition-opacity duration-500" />
                    
                    // Top Badges
                    <div class="absolute top-4 left-4 right-4 flex justify-between items-start pointer-events-none">
                        <Show when=move || has_score>
                            <div class="glass-subtle px-3 py-1.5 rounded-xl border border-white/20 text-xs font-black text-yellow-500 flex items-center gap-1.5 shadow-2xl">
                                "⭐" {score_text.clone()}
                            </div>
                        </Show>
                        <Show when=move || has_count>
                            <div class="glass-subtle px-3 py-1.5 rounded-xl border border-white/20 text-[10px] font-black uppercase tracking-widest text-white/90 shadow-2xl">
                                {count_text.clone()} " EPS"
                            </div>
                        </Show>
                    </div>

                    // Bottom Content
                    <div class="absolute bottom-0 left-0 right-0 p-6 space-y-3 transform translate-y-2 group-hover:translate-y-0 transition-transform duration-500">
                        <Show when=move || has_episode>
                             <div class="inline-flex items-center gap-2 px-3 py-1 rounded-lg bg-blue-500/20 border border-blue-500/30 backdrop-blur-md text-[10px] font-black uppercase tracking-wider text-blue-400">
                                <span class="w-1.5 h-1.5 rounded-full bg-blue-400 animate-pulse" />
                                {current_episode_text.clone()}
                            </div>
                        </Show>
                        
                        <h3 class="text-lg font-black text-white leading-tight line-clamp-2 [text-shadow:0_4px_12px_rgba(0,0,0,0.5)] group-hover:text-blue-200 transition-colors">
                            {item.title}
                        </h3>
                    </div>

                    // Interaction Glow
                    <div class="absolute inset-0 opacity-0 group-hover:opacity-100 bg-gradient-to-tr from-blue-500/10 via-transparent to-purple-500/10 transition-opacity duration-500 pointer-events-none" />
                </div>
                
                // Card Shadow Accent
                <div class="absolute -bottom-4 left-1/2 -translate-x-1/2 w-[80%] h-4 bg-blue-500/20 blur-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
            </a>
        </div>
    }
}

#[component]
fn AnimeGrid(items: Vec<AnimeItem>, source: u8) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-8">
            {items.into_iter().enumerate().map(|(i, item)| view! { <AnimeCard item=item index=i source=source/> }).collect_view()}
        </div>
    }
}

#[component]
fn SectionHeader(
    title: &'static str,
    icon: &'static str,
    gradient: &'static str,
    link: String,
    link_gradient: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row md:items-end justify-between gap-6 mb-12 animate-slide-up fill-mode-forwards">
            <div class="space-y-4">
                <div class="flex items-center gap-4">
                    <div class=format!("w-16 h-16 rounded-3xl bg-gradient-to-br {} flex items-center justify-center text-3xl shadow-2xl relative group overflow-hidden", gradient)>
                        <div class="absolute inset-0 bg-white/20 scale-0 group-hover:scale-150 transition-transform duration-700 rounded-full blur-2xl" />
                        <span class="relative z-10">{icon}</span>
                    </div>
                    <div>
                        <h2 class="text-4xl md:text-5xl font-black tracking-tighter uppercase italic text-foreground leading-none drop-shadow-2xl">
                            {title}
                        </h2>
                        <div class=format!("h-2 w-24 bg-gradient-to-r {} rounded-full mt-3", link_gradient) />
                    </div>
                </div>
            </div>
             <a
                href=link
                class="group flex items-center gap-4 px-8 py-4 rounded-2xl glass border border-white/10 text-foreground font-black uppercase tracking-widest text-sm hover:border-white/40 transition-all hover:scale-105 active:scale-95"
            >
                "View Library"
                <svg class="w-5 h-5 group-hover:translate-x-2 transition-transform duration-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                </svg>
            </a>
        </div>
    }
}

#[component]
pub fn AnimePage(#[prop(default = 1)] source: u8) -> impl IntoView {
    let data = create_resource(move || source, |s| fetch_anime_data(s));
    let source_title = if source == 2 { "Source 2" } else { "Source 1" };

    view! {
        <Title text=format!("Anime {} | Media Hub", source_title)/>
        <main class="min-h-screen py-24 px-6 md:px-12 relative overflow-hidden">
            <div class="max-w-7xl mx-auto space-y-32">
                // Cinematic Header
                <header class="text-center space-y-12 animate-fade-in">
                    <div class="space-y-6">
                        <div class="inline-flex items-center gap-3 px-4 py-2 rounded-full glass border border-white/10 shadow-2xl">
                             <div class="w-2 h-2 rounded-full bg-blue-500 animate-pulse" />
                            <span class="text-[10px] font-black uppercase tracking-[0.2em] text-blue-500">{format!("Streaming Library ({})", source_title)}</span>
                        </div>
                        <h1 class="text-6xl md:text-9xl font-black tracking-tighter uppercase italic line-height-1 mt-4">
                            <span class="bg-gradient-to-r from-blue-400 via-purple-500 to-pink-400 bg-clip-text text-transparent animate-gradient-x bg-[length:200%_auto]">
                                "Anime"
                            </span>
                            <span class="text-foreground/20 block translate-y-[-0.5em] scale-y-75 uppercase">"Hub"</span>
                        </h1>
                    </div>

                    // Premium Search Bar
                    <div class="max-w-3xl mx-auto relative group">
                        <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 rounded-[2.5rem] opacity-20 blur-2xl group-focus-within:opacity-50 transition-opacity duration-700" />
                        <form action=format!("/anime{}/search", if source == 2 { "2" } else { "" }) method="get" class="relative flex gap-4 p-2 rounded-[2.5rem] glass border border-white/20 shadow-2xl backdrop-blur-3xl">
                            <input
                                type="text"
                                name="q"
                                placeholder="Search titles, genres, or studios..."
                                class="flex-1 bg-transparent px-8 py-5 focus:outline-none text-lg font-bold placeholder:text-muted-foreground/50"
                            />
                            <button
                                type="submit"
                                class="px-10 py-5 rounded-[2rem] bg-foreground text-background font-black uppercase tracking-widest hover:scale-95 transition-transform"
                            >
                                "Search"
                            </button>
                        </form>
                    </div>
                </header>

                <Suspense fallback=move || view! { 
                    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-8">
                        {(0..12).map(|_| view! { <div class="aspect-[3/4.2] rounded-[2rem] bg-white/5 animate-pulse" /> }).collect_view()}
                    </div>
                }>
                    <Show when=move || data.get().flatten().is_some() fallback=move || view! { 
                        <div class="glass-card p-12 rounded-[2rem] text-center border border-red-500/20">
                            <div class="text-4xl mb-4">"❌"</div>
                            <h3 class="text-xl font-black uppercase italic">"Connection Error"</h3>
                            <p class="text-muted-foreground">"Unable to load anime database. Please try again later."</p>
                        </div>
                    }>
                        {move || {
                            let d = data.get().flatten().unwrap();
                            let prefix = if source == 2 { "anime2" } else { "anime" };
                            view! {
                                <div class="space-y-32">
                                    <section>
                                        <SectionHeader
                                            title="Ongoing"
                                            icon="🔥"
                                            gradient="from-blue-600 to-indigo-700"
                                            link=format!("/{}/ongoing-anime/1", prefix)
                                            link_gradient="from-blue-500 to-indigo-500"
                                        />
                                        <AnimeGrid items=d.ongoing_anime source=source/>
                                    </section>

                                    <section>
                                        <SectionHeader
                                            title="Complete"
                                            icon="✨"
                                            gradient="from-purple-600 to-pink-700"
                                            link=format!("/{}/complete-anime/1", prefix)
                                            link_gradient="from-purple-500 to-pink-500"
                                        />
                                        <AnimeGrid items=d.complete_anime source=source/>
                                    </section>
                                </div>
                            }
                        }}
                    </Show>
                </Suspense>
            </div>
        </main>
    }
}
