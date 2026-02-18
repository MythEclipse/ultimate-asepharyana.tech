pub mod detail;
pub mod search;
use leptos::*;
use leptos_meta::*;
use serde::{Serialize, Deserialize};
use crate::api::komik::{fetch_manga, fetch_manhwa, fetch_manhua};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KomikItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub r#type: String,
    pub chapter: Option<String>,
    pub score: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomeData {
    pub manga: Vec<KomikItem>,
    pub manhwa: Vec<KomikItem>,
    pub manhua: Vec<KomikItem>,
}

async fn fetch_komik_data() -> Option<HomeData> {
     // Fetch all 3 sequentially for now
    let manga_res = fetch_manga(1).await;
    let manhwa_res = fetch_manhwa(1).await;
    let manhua_res = fetch_manhua(1).await;

    let convert = |res: Result<crate::api::komik::MangaResponse, String>| -> Vec<KomikItem> {
        match res {
            Ok(r) => r.data.into_iter().map(|i| KomikItem {
                title: i.title,
                slug: i.slug,
                poster: i.poster,
                r#type: i.r#type,
                chapter: Some(i.chapter),
                score: i.score,
            }).collect(),
            Err(_) => vec![],
        }
    };

    let manga = convert(manga_res);
    let manhwa = convert(manhwa_res);
    let manhua = convert(manhua_res);

    Some(HomeData {
        manga,
        manhwa,
        manhua,
    })
}

#[component]
fn KomikCard(item: KomikItem, index: usize) -> impl IntoView {
    let delay_style = format!("animation-delay: {}s", index as f64 * 0.08);
    let type_bg = match item.r#type.as_str() {
        "Manga" => "bg-orange-500/90 text-white",
        "Manhwa" => "bg-blue-500/90 text-white",
        "Manhua" => "bg-red-500/90 text-white",
        _ => "bg-primary/90 text-primary-foreground",
    };

    let has_score = item.score.is_some();
    let score_text = item.score.clone();
    
    let has_chapter = item.chapter.is_some();
    let chapter_text = item.chapter.clone();

    view! {
        <div 
            class="group perspective-1000 animate-fade-in"
            style=delay_style
        >
            <a
                href=format!("/komik/detail?komik_id={}", item.slug)
                class="block relative overflow-hidden rounded-2xl bg-card border border-border shadow-lg hover:shadow-2xl hover:shadow-primary/20 transition-all duration-500 transform-gpu hover:-translate-y-3 hover:scale-[1.02]"
            >
                 // Glow effect on hover
                <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-t from-primary/20 via-transparent to-transparent blur-xl" />

                 // Animated border glow
                <div class="absolute -inset-[1px] rounded-2xl bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10 blur-sm" />

                <div class="aspect-[3/4] overflow-hidden relative">
                    <img
                        src=item.poster
                        alt=item.title.clone()
                        class="w-full h-full object-cover transform-gpu group-hover:scale-110 transition-transform duration-700 ease-out"
                        loading="lazy"
                    />
                     // Shine effect
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-1000 ease-out" />
                </div>

                 // Gradient overlay
                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/40 to-transparent opacity-80 group-hover:opacity-90 transition-opacity duration-300" />

                 // Type Badge
                <div class="absolute top-3 right-3 text-white">
                    <span class=format!("px-2 py-1 rounded-lg text-[10px] font-black shadow-lg {}", type_bg)>
                        {item.r#type.clone()}
                    </span>
                </div>

                 // Score Badge
                <Show when=move || has_score>
                    <div class="absolute top-3 left-3">
                        <span class="px-2 py-1 rounded-lg text-xs font-bold bg-yellow-500/90 text-black shadow-lg backdrop-blur-sm flex items-center gap-1">
                            "⭐ " {score_text.clone()}
                        </span>
                    </div>
                </Show>

                 // Content
                <div class="absolute bottom-0 left-0 right-0 p-4">
                    <h3 class="text-white text-sm font-bold line-clamp-2 drop-shadow-lg mb-2 group-hover:text-orange-200 transition-colors duration-300">
                        {item.title}
                    </h3>
                    <Show when=move || has_chapter>
                        <span class="inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full bg-primary/80 text-white backdrop-blur-sm">
                            <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse" />
                            {chapter_text.clone()}
                        </span>
                    </Show>
                </div>
            </a>
        </div>
    }
}

// ... KomikGrid and SectionHeader preserved ...

#[component]
fn KomikGrid(items: Vec<KomikItem>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-5">
            {items.into_iter().enumerate().map(|(i, item)| view! { <KomikCard item=item index=i/> }).collect_view()}
        </div>
    }
}

#[component]
fn SectionHeader(
    title: &'static str,
    gradient: &'static str,
    emoji: &'static str,
    href: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between mb-6 animate-slide-in-right">
            <div class="flex items-center gap-3">
                <div class=format!("w-12 h-12 rounded-xl bg-gradient-to-br {} flex items-center justify-center text-2xl shadow-lg", gradient)>
                    {emoji}
                </div>
                <h2 class=format!("text-2xl font-bold bg-gradient-to-r {} bg-clip-text text-transparent", gradient)>
                    {title}
                </h2>
            </div>
            <a
                href=href
                class="group flex items-center gap-2 px-4 py-2 rounded-xl bg-primary/10 text-primary hover:bg-primary hover:text-primary-foreground transition-all duration-300"
            >
                "View All"
                <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
            </a>
        </div>
    }
}

#[component]
pub fn KomikPage() -> impl IntoView {
     let data = create_resource(|| (), |_| fetch_komik_data());
     let (search_query, set_search_query) = create_signal("".to_string());

    view! {
        <Title text="Komik | Asepharyana"/>
        <main class="min-h-screen bg-background text-foreground">
             // Hero Section
            <div class="relative overflow-hidden">
                 // Background Orbs
                <div class="absolute inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute -top-40 -right-40 w-80 h-80 rounded-full bg-gradient-to-br from-orange-500/20 to-red-500/20 blur-3xl" />
                    <div class="absolute top-1/2 -left-40 w-60 h-60 rounded-full bg-gradient-to-br from-blue-500/20 to-purple-500/20 blur-3xl" />
                </div>

                <div class="relative z-10 p-4 md:p-8 lg:p-12 max-w-7xl mx-auto">
                    // Header
                    <div class="text-center mb-10 animate-fade-in">
                        <h1 class="text-5xl md:text-6xl font-bold mb-4">
                            <span class="gradient-text">"Komik"</span>
                        </h1>
                        <p class="text-muted-foreground text-lg">
                            "Read your favorite manga, manhwa, and manhua"
                        </p>
                    </div>

                    // Search Bar
                    <form action="/komik/search" method="get" class="mb-12 animate-fade-in">
                        <div class="max-w-2xl mx-auto relative">
                             <div class="absolute inset-0 bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 rounded-2xl blur-sm opacity-50" />
                            <div class="relative flex gap-3 p-2 rounded-2xl glass-card">
                                <input
                                    type="text"
                                    name="q"
                                    prop:value=search_query
                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                    placeholder="Search manga, manhwa, manhua..."
                                    class="flex-1 px-5 py-3 rounded-xl bg-background/50 border-0 focus:outline-none focus:ring-2 focus:ring-primary placeholder:text-muted-foreground"
                                />
                                <button
                                    type="submit"
                                    class="px-6 py-3 rounded-xl bg-gradient-to-r from-orange-500 to-red-500 text-white font-semibold hover:opacity-90 transition-opacity flex items-center gap-2"
                                >
                                     <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                                    </svg>
                                    "Search"
                                </button>
                            </div>
                        </div>
                    </form>

                    // Category Tabs
                    <div class="flex justify-center gap-3 mb-12 flex-wrap animate-fade-in">
                        <a href="/komik/manga/page/1" class="px-6 py-3 rounded-xl glass-subtle hover:bg-orange-500/20 hover:text-orange-500 transition-all duration-300 font-medium">
                            "📚 All Manga"
                        </a>
                        <a href="/komik/manhwa/page/1" class="px-6 py-3 rounded-xl glass-subtle hover:bg-blue-500/20 hover:text-blue-500 transition-all duration-300 font-medium">
                            "🇰🇷 All Manhwa"
                        </a>
                        <a href="/komik/manhua/page/1" class="px-6 py-3 rounded-xl glass-subtle hover:bg-red-500/20 hover:text-red-500 transition-all duration-300 font-medium">
                            "🇨🇳 All Manhua"
                        </a>
                    </div>

                    <Suspense fallback=move || view! { <div class="text-center">"Loading..."</div> }>
                        <Show when=move || data.get().flatten().is_some() fallback=move || view! { <div>"Failed to load"</div> }>
                            {move || {
                                let d = data.get().flatten().unwrap();
                                view! {
                                    <section class="mb-16">
                                        <SectionHeader title="Manga" gradient="from-orange-500 to-red-500" emoji="📚" href="/komik/manga/page/1"/>
                                        <KomikGrid items=d.manga/>
                                    </section>
                                    <section class="mb-16">
                                        <SectionHeader title="Manhwa" gradient="from-blue-500 to-purple-500" emoji="🇰🇷" href="/komik/manhwa/page/1"/>
                                        <KomikGrid items=d.manhwa/>
                                    </section>
                                    <section class="mb-16">
                                        <SectionHeader title="Manhua" gradient="from-red-500 to-yellow-500" emoji="🇨🇳" href="/komik/manhua/page/1"/>
                                        <KomikGrid items=d.manhua/>
                                    </section>
                                }
                            }}
                        </Show>
                    </Suspense>

                </div>
            </div>
        </main>
    }
}
