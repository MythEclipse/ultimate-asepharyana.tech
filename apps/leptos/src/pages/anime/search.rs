use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::anime::{search_anime, SearchAnimeItem};

#[component]
pub fn AnimeSearchPage() -> impl IntoView {
    let query = use_query_map();
    let q = move || query.get().get("q").cloned().unwrap_or_default();
    
    let search_results = create_resource(q, |s| async move {
        if s.is_empty() { return None; }
        search_anime(s).await.ok()
    });

    view! {
        <Title text=move || format!("Searching \"{}\" | Anime Hub", q())/>
        <main class="min-h-screen relative overflow-hidden pb-40">
            <div class="max-w-7xl mx-auto px-6 py-24 space-y-24">
                // Cinematic Search Header
                <header class="flex flex-col md:flex-row md:items-end justify-between gap-8 animate-slide-up">
                    <div class="space-y-6">
                        <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.2em] text-blue-500">
                            "Discovery Engine"
                        </div>
                        <div class="flex items-center gap-6">
                            <div class="w-16 h-16 rounded-[2rem] bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-3xl shadow-2xl relative group overflow-hidden">
                                <div class="absolute inset-0 bg-white/20 scale-0 group-hover:scale-150 transition-transform duration-700 rounded-full blur-3xl" />
                                <span class="relative z-10">"🔍"</span>
                            </div>
                            <div>
                                <h1 class="text-4xl md:text-6xl font-black tracking-tighter uppercase italic line-tight">
                                    "Search Results"
                                </h1>
                                <div class="flex items-center gap-2 mt-2">
                                    <span class="text-muted-foreground/60 text-xs font-black uppercase tracking-widest">"Refining for:"</span>
                                    <span class="text-blue-400 font-black italic tracking-tight">{move || format!("\"{}\"", q())}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </header>

                <Suspense fallback=move || view! { 
                    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-8">
                        {(0..12).map(|_| view! { <div class="aspect-[3/4.2] rounded-[2rem] bg-white/5 animate-pulse" /> }).collect_view()}
                    </div>
                }>
                    {move || search_results.get().flatten().map(|items| {
                        if items.is_empty() {
                            return view! {
                                <div class="glass-card p-24 text-center rounded-[3rem] border border-white/10 max-w-2xl mx-auto space-y-8 animate-fade-in relative overflow-hidden">
                                    <div class="absolute inset-0 bg-blue-500/5 blur-3xl -z-10" />
                                    <div class="w-32 h-32 rounded-[2.5rem] bg-white/5 border border-white/5 flex items-center justify-center text-6xl mx-auto shadow-2xl">"🏜️"</div>
                                    <div class="space-y-4">
                                        <h3 class="text-3xl font-black uppercase tracking-tighter italic">"No Signal Detected"</h3>
                                        <p class="text-muted-foreground/60 font-medium leading-relaxed">"The library archives don't contain any entries matching your query. Please recalibrate your search terms."</p>
                                    </div>
                                    <a href="/anime" class="inline-flex px-8 py-4 rounded-2xl bg-foreground text-background font-black uppercase tracking-widest text-xs hover:scale-105 transition-transform active:scale-95 shadow-2xl">
                                        "Back to Hub"
                                    </a>
                                </div>
                            }.into_view();
                        }
                        
                        view! {
                            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-8">
                                {items.into_iter().enumerate().map(|(i, item)| view! {
                                    <SearchAnimeCard item=item index=i />
                                }).collect_view()}
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! { 
                        <div class="text-center py-40 animate-fade-in space-y-6">
                            <div class="text-6xl opacity-20">"🔭"</div>
                            <p class="text-muted-foreground/40 font-black uppercase tracking-[0.4em] text-xs">"Awaiting Query Input"</p>
                        </div> 
                    }.into_view())}
                </Suspense>
            </div>
        </main>
    }
}

#[component]
fn SearchAnimeCard(item: SearchAnimeItem, index: usize) -> impl IntoView {
    let delay = format!("animation-delay: {}ms", index * 50);
    
    view! {
        <div 
            class="group animate-slide-up opacity-0 fill-mode-forwards" 
            style=delay
        >
            <a href=format!("/anime/detail/{}", item.slug) class="block relative group/card perspective-1000">
                <div class="relative aspect-[3/4.2] rounded-[2rem] overflow-hidden bg-muted border border-white/5 shadow-2xl transition-all duration-700 hover-tilt group-hover:shadow-blue-500/20 group-hover:border-white/20">
                    <img 
                        src=item.poster 
                        class="w-full h-full object-cover transition-transform duration-1000 ease-out group-hover:scale-115" 
                        alt=item.title.clone() 
                        loading="lazy"
                    />
                    
                    <div class="absolute inset-0 bg-gradient-to-t from-black via-black/20 to-transparent opacity-80 group-hover:opacity-60 transition-opacity duration-500" />
                    
                    <div class="absolute bottom-0 left-0 right-0 p-6 space-y-3 transform translate-y-2 group-hover:translate-y-0 transition-transform duration-500">
                        <div class="flex items-center gap-2">
                            <span class="px-2 py-1 rounded-lg bg-blue-500/20 border border-blue-500/30 backdrop-blur-md text-[10px] font-black uppercase tracking-widest text-blue-400">
                                {item.episode}
                            </span>
                            <span class="px-2 py-1 rounded-lg bg-yellow-500/90 text-[10px] font-black uppercase tracking-widest text-black shadow-lg">
                                {item.status}
                            </span>
                        </div>
                        <h3 class="text-sm font-black text-white leading-tight line-clamp-2 [text-shadow:0_4px_12px_rgba(0,0,0,0.5)] group-hover:text-blue-200 transition-colors">
                            {item.title}
                        </h3>
                    </div>

                    // Interaction Glow
                    <div class="absolute inset-0 opacity-0 group-hover:opacity-100 bg-gradient-to-tr from-blue-500/10 via-transparent to-indigo-500/10 transition-opacity duration-500 pointer-events-none" />
                </div>
                
                // Card Shadow Accent
                <div class="absolute -bottom-4 left-1/2 -translate-x-1/2 w-[80%] h-4 bg-blue-500/20 blur-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
            </a>
        </div>
    }
}
