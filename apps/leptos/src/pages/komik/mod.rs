pub mod detail;
pub mod search;
pub mod read;
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

thread_local! {
    static KOMIK_CACHE: std::cell::RefCell<Option<HomeData>> = std::cell::RefCell::new(None);
}

async fn fetch_komik_data() -> Option<HomeData> {
    #[cfg(feature = "csr")]
    {
        let cached = KOMIK_CACHE.with(|cache| cache.borrow().clone());
        if cached.is_some() {
            return cached;
        }
    }
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

    let data = HomeData {
        manga,
        manhwa,
        manhua,
    };
    
    #[cfg(feature = "csr")]
    KOMIK_CACHE.with(|cache| *cache.borrow_mut() = Some(data.clone()));

    Some(data)
}

#[component]
fn KomikCard(item: KomikItem, index: usize) -> impl IntoView {
    let delay_style = format!("animation-delay: {}ms", index * 50);
    let type_bg = match item.r#type.as_str() {
        "Manga" => "from-orange-500 to-red-600 shadow-orange-500/20",
        "Manhwa" => "from-blue-500 to-indigo-600 shadow-blue-500/20",
        "Manhua" => "from-red-500 to-pink-600 shadow-red-500/20",
        _ => "from-primary to-primary/80 shadow-primary/20",
    };

    let has_score = item.score.is_some();
    let score_text = item.score.clone();
    let has_chapter = item.chapter.is_some();
    let chapter_text = item.chapter.clone();

    view! {
        <div 
            class="group animate-slide-up opacity-0 fill-mode-forwards"
            style=delay_style
        >
            <a
                href=format!("/komik/detail?komik_id={}", item.slug)
                class="block relative group/card perspective-1000"
            >
                <div class="relative aspect-[3/4.2] rounded-[2rem] overflow-hidden bg-muted border border-white/5 shadow-2xl transition-all duration-700 hover-tilt group-hover:shadow-orange-500/20 group-hover:border-white/20">
                    // Poster with parallax zoom
                    <img
                        src=item.poster
                        alt=item.title.clone()
                        class="w-full h-full object-cover transition-transform duration-1000 ease-out group-hover:scale-115"
                        loading="lazy"
                    />
                    
                    // Glassy Overlay
                    <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/20 to-transparent opacity-60 group-hover:opacity-40 transition-opacity duration-500" />
                    
                    // Top Badges
                    <div class="absolute top-4 left-4 right-4 flex justify-between items-start pointer-events-none">
                        <Show when=move || has_score>
                            <div class="glass-subtle px-3 py-1.5 rounded-xl border border-white/20 text-xs font-black text-yellow-500 flex items-center gap-1.5 shadow-2xl">
                                "⭐" {score_text.clone()}
                            </div>
                        </Show>
                        <div class=format!("glass px-3 py-1.5 rounded-xl border border-white/10 text-[10px] font-black uppercase tracking-widest text-white shadow-2xl bg-gradient-to-br {}", type_bg)>
                            {item.r#type.clone()}
                        </div>
                    </div>

                    // Bottom Content
                    <div class="absolute bottom-0 left-0 right-0 p-6 space-y-3 transform translate-y-2 group-hover:translate-y-0 transition-transform duration-500">
                        <Show when=move || has_chapter>
                             <div class="inline-flex items-center gap-2 px-3 py-1 rounded-lg bg-orange-500/20 border border-orange-500/30 backdrop-blur-md text-[10px] font-black uppercase tracking-wider text-orange-400">
                                <span class="w-1.5 h-1.5 rounded-full bg-orange-400 animate-pulse" />
                                {chapter_text.clone()}
                            </div>
                        </Show>
                        
                        <h3 class="text-lg font-black text-white leading-tight line-clamp-2 [text-shadow:0_4px_12px_rgba(0,0,0,0.5)] group-hover:text-orange-200 transition-colors">
                            {item.title}
                        </h3>
                    </div>

                    // Interaction Glow
                    <div class="absolute inset-0 opacity-0 group-hover:opacity-100 bg-gradient-to-tr from-orange-500/10 via-transparent to-red-500/10 transition-opacity duration-500 pointer-events-none" />
                </div>
                
                // Card Shadow Accent
                <div class="absolute -bottom-4 left-1/2 -translate-x-1/2 w-[80%] h-4 bg-orange-500/20 blur-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
            </a>
        </div>
    }
}

#[component]
fn KomikGrid(items: Vec<KomikItem>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-8">
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
        <div class="flex flex-col md:flex-row md:items-end justify-between gap-6 mb-12 animate-slide-up fill-mode-forwards">
            <div class="space-y-4">
                <div class="flex items-center gap-4">
                    <div class=format!("w-16 h-16 rounded-3xl bg-gradient-to-br {} flex items-center justify-center text-3xl shadow-2xl relative group overflow-hidden", gradient)>
                        <div class="absolute inset-0 bg-white/20 scale-0 group-hover:scale-150 transition-transform duration-700 rounded-full blur-2xl" />
                        <span class="relative z-10">{emoji}</span>
                    </div>
                    <div>
                        <h2 class="text-4xl md:text-5xl font-black tracking-tighter uppercase italic text-foreground leading-none drop-shadow-2xl">
                            {title}
                        </h2>
                        <div class=format!("h-2 w-24 bg-gradient-to-r {} rounded-full mt-3", gradient) />
                    </div>
                </div>
            </div>
             <a
                href=href
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
pub fn KomikPage() -> impl IntoView {
    let data = create_resource(|| (), |_| fetch_komik_data());
    let (search_query, set_search_query) = create_signal("".to_string());

    view! {
        <Title text="Comics | Media Hub"/>
        <main class="min-h-screen py-24 px-6 md:px-12 relative overflow-hidden">
            <div class="max-w-7xl mx-auto space-y-32">
                // Cinematic Header
                <header class="text-center space-y-12 animate-fade-in">
                    <div class="space-y-6">
                        <div class="inline-flex items-center gap-3 px-4 py-2 rounded-full glass border border-white/10 shadow-2xl">
                             <div class="w-2 h-2 rounded-full bg-orange-500 animate-pulse" />
                            <span class="text-[10px] font-black uppercase tracking-[0.2em] text-orange-500">"Latest Updates"</span>
                        </div>
                        <h1 class="text-6xl md:text-9xl font-black tracking-tighter uppercase italic line-height-1 mt-4">
                            <span class="bg-gradient-to-r from-orange-400 via-red-500 to-pink-400 bg-clip-text text-transparent animate-gradient-x bg-[length:200%_auto]">
                                "Digital"
                            </span>
                            <span class="text-foreground/20 block translate-y-[-0.5em] scale-y-75 uppercase">"Library"</span>
                        </h1>
                    </div>

                    // Premium Search Bar
                    <div class="max-w-3xl mx-auto relative group">
                        <div class="absolute -inset-1 bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 rounded-[2.5rem] opacity-20 blur-2xl group-focus-within:opacity-50 transition-opacity duration-700" />
                        <form action="/komik/search" method="get" class="relative flex gap-4 p-2 rounded-[2.5rem] glass border border-white/20 shadow-2xl backdrop-blur-3xl">
                            <input
                                type="text"
                                name="q"
                                prop:value=search_query
                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                placeholder="Search comics, manga, or authors..."
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

                    // Refined Category Tabs
                    <div class="flex justify-center gap-4 flex-wrap animate-fade-in [animation-delay:200ms]">
                        <a href="/komik/manga/page/1" class="px-8 py-4 rounded-2xl glass border border-white/10 text-xs font-black uppercase tracking-widest hover:bg-orange-500/10 hover:text-orange-500 transition-all hover:-translate-y-1">
                            "📚 Manga"
                        </a>
                        <a href="/komik/manhwa/page/1" class="px-8 py-4 rounded-2xl glass border border-white/10 text-xs font-black uppercase tracking-widest hover:bg-blue-500/10 hover:text-blue-500 transition-all hover:-translate-y-1">
                            "🇰🇷 Manhwa"
                        </a>
                        <a href="/komik/manhua/page/1" class="px-8 py-4 rounded-2xl glass border border-white/10 text-xs font-black uppercase tracking-widest hover:bg-red-500/10 hover:text-red-500 transition-all hover:-translate-y-1">
                            "🇨🇳 Manhua"
                        </a>
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
                            <p class="text-muted-foreground">"Unable to load comic database. Please try again later."</p>
                        </div>
                    }>
                        {move || {
                            let d = data.get().flatten().unwrap();
                            view! {
                                <div class="space-y-32">
                                    <section>
                                        <SectionHeader title="Manga" gradient="from-orange-500 to-red-600" emoji="📚" href="/komik/manga/page/1"/>
                                        <KomikGrid items=d.manga/>
                                    </section>
                                    <section>
                                        <SectionHeader title="Manhwa" gradient="from-blue-500 to-indigo-600" emoji="🇰🇷" href="/komik/manhwa/page/1"/>
                                        <KomikGrid items=d.manhwa/>
                                    </section>
                                    <section>
                                        <SectionHeader title="Manhua" gradient="from-red-500 to-pink-600" emoji="🇨🇳" href="/komik/manhua/page/1"/>
                                        <KomikGrid items=d.manhua/>
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
