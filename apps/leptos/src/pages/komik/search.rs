use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::komik::{search_komik, MangaItem};

#[component]
pub fn KomikSearchPage() -> impl IntoView {
    let query_map = use_query_map();
    let q = move || query_map.get().get("q").cloned().or_else(|| query_map.get().get("query").cloned()).unwrap_or_default();
    let page = move || query_map.get().get("page").and_then(|p| p.parse::<u32>().ok()).unwrap_or(1);
    
    let search_results = create_resource(move || (q(), page()), |(s, p)| async move {
        if s.is_empty() { return None; }
        search_komik(s, p).await.ok()
    });

    view! {
        <Title text=move || format!("Searching \"{}\" | Reader Hub", q())/>
        <main class="min-h-screen relative overflow-hidden pb-40">
            <div class="max-w-7xl mx-auto px-6 py-24 space-y-24">
                // Cinematic Search Header
                <header class="flex flex-col md:flex-row md:items-end justify-between gap-8 animate-slide-up">
                    <div class="space-y-6">
                        <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.2em] text-orange-500">
                            "Archive Reconnaissance"
                        </div>
                        <div class="flex items-center gap-6">
                            <div class="w-16 h-16 rounded-[2rem] bg-gradient-to-br from-orange-500 to-red-600 flex items-center justify-center text-3xl shadow-2xl relative group overflow-hidden">
                                <div class="absolute inset-0 bg-white/20 scale-0 group-hover:scale-150 transition-transform duration-700 rounded-full blur-3xl" />
                                <span class="relative z-10">"🔍"</span>
                            </div>
                            <div>
                                <h1 class="text-4xl md:text-6xl font-black tracking-tighter uppercase italic line-tight">
                                    "Komik Search"
                                </h1>
                                <div class="flex items-center gap-2 mt-2">
                                    <span class="text-muted-foreground/60 text-xs font-black uppercase tracking-widest">"Filtering for:"</span>
                                    <span class="text-orange-400 font-black italic tracking-tight">{move || format!("\"{}\"", q())}</span>
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
                    {move || search_results.get().flatten().map(|res| {
                        let items = res.data;
                        if items.is_empty() {
                            return view! {
                                <div class="glass-card p-24 text-center rounded-[3rem] border border-white/10 max-w-2xl mx-auto space-y-8 animate-fade-in relative overflow-hidden">
                                    <div class="absolute inset-0 bg-orange-500/5 blur-3xl -z-10" />
                                    <div class="w-32 h-32 rounded-[2.5rem] bg-white/5 border border-white/5 flex items-center justify-center text-8xl mx-auto shadow-2xl">"🤷‍♂️"</div>
                                    <div class="space-y-4">
                                        <h3 class="text-3xl font-black uppercase tracking-tighter italic">"No Trace Found"</h3>
                                        <p class="text-muted-foreground/60 font-medium leading-relaxed">"The reader universe doesn't seem to contain any scrolls matching your encryption. Try clearing your filters."</p>
                                    </div>
                                    <a href="/komik" class="inline-flex px-8 py-4 rounded-2xl bg-foreground text-background font-black uppercase tracking-widest text-xs hover:scale-105 transition-transform active:scale-95 shadow-2xl">
                                        "Back to Library"
                                    </a>
                                </div>
                            }.into_view();
                        }
                        
                        view! {
                            <div class="space-y-24">
                                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-8">
                                    {items.into_iter().enumerate().map(|(i, item)| view! {
                                        <SearchKomikCard item=item index=i />
                                    }).collect_view()}
                                </div>
                                
                                // Premium Pagination
                                <div class="flex justify-center items-center gap-6 animate-fade-in">
                                    <Show when=move || res.pagination.has_previous_page>
                                        <a 
                                            href=move || format!("/komik/search?q={}&page={}", q(), page() - 1)
                                            class="group flex items-center gap-3 px-8 py-4 rounded-2xl glass border border-white/10 hover:border-orange-500/40 transition-all font-black uppercase text-xs tracking-widest hover:bg-orange-500/10 hover:text-orange-500"
                                        >
                                            <svg class="w-4 h-4 group-hover:-translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M15 19l-7-7 7-7" />
                                            </svg>
                                            "Rewind"
                                        </a>
                                    </Show>

                                    <div class="glass px-8 py-4 rounded-2xl border border-white/10 flex flex-col items-center">
                                        <span class="text-[8px] font-black uppercase tracking-[0.4em] text-orange-500 opacity-60">"Index"</span>
                                        <span class="text-lg font-black italic tracking-tighter">
                                            {format!("{:02}", page())}
                                        </span>
                                    </div>

                                    <Show when=move || res.pagination.has_next_page>
                                        <a 
                                            href=move || format!("/komik/search?q={}&page={}", q(), page() + 1)
                                            class="group flex items-center gap-3 px-8 py-4 rounded-2xl bg-foreground text-background font-black uppercase text-xs tracking-widest hover:scale-105 active:scale-95 transition-all shadow-2xl"
                                        >
                                            "Forward"
                                            <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M9 5l7 7-7 7" />
                                            </svg>
                                        </a>
                                    </Show>
                                </div>
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! { 
                        <div class="text-center py-40 animate-fade-in space-y-6">
                            <div class="text-8xl opacity-20">"📚"</div>
                            <p class="text-muted-foreground/40 font-black uppercase tracking-[0.4em] text-xs">"Awaiting Library Command"</p>
                        </div> 
                    }.into_view())}
                </Suspense>
            </div>
        </main>
    }
}

#[component]
fn SearchKomikCard(item: MangaItem, index: usize) -> impl IntoView {
    let delay = format!("animation-delay: {}ms", index * 50);
    let score = item.score.clone();
    let r_type = item.r#type.clone();
    let type_bg = match r_type.as_str() {
        "Manga" => "from-orange-500 to-red-600 shadow-orange-500/20",
        "Manhwa" => "from-blue-500 to-indigo-600 shadow-blue-500/20",
        "Manhua" => "from-red-500 to-pink-600 shadow-red-500/20",
        _ => "from-primary to-primary/80 shadow-primary/20",
    };
    
    view! {
        <div 
            class="group animate-slide-up opacity-0 fill-mode-forwards" 
            style=delay
        >
            <a href=format!("/komik/detail?komik_id={}", item.slug) class="block relative group/card perspective-1000">
                <div class="relative aspect-[3/4.2] rounded-[2rem] overflow-hidden bg-muted border border-white/5 shadow-2xl transition-all duration-700 hover-tilt group-hover:shadow-orange-500/20 group-hover:border-white/20">
                    <img 
                        src=item.poster 
                        class="w-full h-full object-cover transition-transform duration-1000 ease-out group-hover:scale-115" 
                        alt=item.title.clone() 
                        loading="lazy"
                    />
                    
                    <div class="absolute inset-0 bg-gradient-to-t from-black via-black/20 to-transparent opacity-90 group-hover:opacity-70 transition-opacity duration-500" />
                    
                    <div class="absolute top-4 right-4">
                        <div class=format!("glass px-3 py-1 rounded-lg border border-white/10 text-[10px] font-black uppercase tracking-widest text-white shadow-2xl bg-gradient-to-br {}", type_bg)>
                            {r_type}
                        </div>
                    </div>

                    <div class="absolute bottom-0 left-0 right-0 p-6 space-y-3 transform translate-y-2 group-hover:translate-y-0 transition-transform duration-500">
                        <div class="flex items-center justify-between">
                            <div class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-orange-500/20 border border-orange-500/30 backdrop-blur-md text-[10px] font-black uppercase tracking-wider text-orange-400">
                                <span class="w-1.5 h-1.5 rounded-full bg-orange-400 animate-pulse" />
                                {item.chapter}
                            </div>
                            <Show when={
                                let score = score.clone();
                                move || score.is_some()
                            }>
                                <div class="glass-subtle px-2 py-1 rounded-lg border border-white/20 text-[10px] font-black text-yellow-500 flex items-center gap-1 shadow-2xl">
                                    "⭐" {score.clone().unwrap_or_default()}
                                </div>
                            </Show>
                        </div>
                        <h3 class="text-sm font-black text-white leading-tight line-clamp-2 [text-shadow:0_4px_12px_rgba(0,0,0,0.5)] group-hover:text-orange-200 transition-colors">
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
