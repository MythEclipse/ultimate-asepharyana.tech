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
        <Title text=move || format!("Search: {} | Asepharyana", q())/>
        <main class="min-h-screen bg-background text-foreground p-8 lg:p-12">
            <div class="max-w-7xl mx-auto">
                <div class="flex items-center gap-6 mb-12 animate-fade-in">
                    <div class="p-5 rounded-3xl bg-orange-500 shadow-lg shadow-orange-500/30">
                        <span class="text-3xl">"🔍"</span>
                    </div>
                    <div>
                        <h1 class="text-4xl md:text-5xl font-black">"Komik Search"</h1>
                        <p class="text-muted-foreground text-lg">"Showing results for: " <span class="text-orange-400 font-bold">"\"" {q} "\""</span></p>
                    </div>
                </div>

                <Suspense fallback=move || view! { <div class="text-center py-20 text-xl">"Searching the library..."</div> }>
                    {move || search_results.get().flatten().map(|res| {
                        let items = res.data;
                        if items.is_empty() {
                            return view! {
                                <div class="glass-card p-20 text-center rounded-3xl border border-white/5 max-w-2xl mx-auto">
                                    <div class="text-8xl mb-8">"🤷‍♂️"</div>
                                    <h3 class="text-3xl font-bold mb-4">"No Comics Found"</h3>
                                    <p class="text-muted-foreground text-lg">"We couldn't find any comics matching your search. Try different keywords or check your spelling."</p>
                                </div>
                            }.into_view();
                        }
                        
                        view! {
                            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-8 mb-12">
                                {items.into_iter().enumerate().map(|(i, item)| view! {
                                    <SearchKomikCard item=item index=i />
                                }).collect_view()}
                            </div>
                            
                            // Simple Pagination
                            <div class="flex justify-center items-center gap-4">
                                <Show when=move || res.pagination.has_previous_page>
                                    <a 
                                        href=move || format!("/komik/search?q={}&page={}", q(), page() - 1)
                                        class="px-6 py-3 rounded-xl bg-white/5 border border-white/10 hover:bg-orange-500 hover:text-white transition-all font-bold"
                                    >
                                        "Previous"
                                    </a>
                                </Show>
                                <span class="text-lg font-black px-6 py-3 rounded-xl bg-orange-500/10 text-orange-400 border border-orange-500/20">
                                    {page}
                                </span>
                                <Show when=move || res.pagination.has_next_page>
                                    <a 
                                        href=move || format!("/komik/search?q={}&page={}", q(), page() + 1)
                                        class="px-6 py-3 rounded-xl bg-white/5 border border-white/10 hover:bg-orange-500 hover:text-white transition-all font-bold"
                                    >
                                        "Next"
                                    </a>
                                </Show>
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! { <div class="text-center py-20 text-muted-foreground">"Search for your favorite manga, manhwa, or manhua"</div> }.into_view())}
                </Suspense>
            </div>
        </main>
    }
}

#[component]
fn SearchKomikCard(item: MangaItem, index: usize) -> impl IntoView {
    let delay = format!("animation-delay: {}s", index as f64 * 0.05);
    
    let score = item.score.clone();
    let score_val = score.clone();
    let r_type = item.r#type.clone();
    
    view! {
        <div class="group animate-fade-in" style=delay>
            <a href=format!("/komik/detail?komik_id={}", item.slug) class="block relative rounded-2xl overflow-hidden bg-card border border-border shadow-lg hover:shadow-2xl transition-all duration-500 hover:-translate-y-2 hover:scale-[1.02]">
                <div class="aspect-[3/4] overflow-hidden">
                    <img src=item.poster class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" alt=item.title.clone() />
                </div>
                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/40 to-transparent opacity-90 transition-opacity group-hover:opacity-100" />
                
                <div class="absolute top-3 right-3">
                    <span class="px-2 py-1 rounded-lg bg-orange-500 text-[10px] font-black text-white shadow-lg">
                        {r_type}
                    </span>
                </div>

                <div class="absolute bottom-0 left-0 right-0 p-4">
                    <h3 class="text-white text-sm font-bold line-clamp-2 mb-2 group-hover:text-orange-300 transition-colors">
                        {item.title}
                    </h3>
                    <div class="flex items-center justify-between">
                         <span class="text-[10px] items-center gap-1 inline-flex text-orange-400 font-bold">
                            "🆕 " {item.chapter}
                        </span>
                        <Show when=move || score.is_some()>
                            <span class="text-[10px] text-yellow-500 font-bold">
                                "⭐ " {score_val.clone().unwrap()}
                            </span>
                        </Show>
                    </div>
                </div>
            </a>
        </div>
    }
}
