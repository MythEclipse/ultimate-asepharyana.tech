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
        <Title text=move || format!("Search: {} | Asepharyana", q())/>
        <main class="min-h-screen bg-background text-foreground p-8 lg:p-12">
            <div class="max-w-7xl mx-auto">
                <div class="flex items-center gap-6 mb-12 animate-fade-in">
                    <div class="p-5 rounded-3xl bg-blue-500 shadow-lg shadow-blue-500/30">
                        <span class="text-3xl">"🔍"</span>
                    </div>
                    <div>
                        <h1 class="text-4xl md:text-5xl font-black">"Search Results"</h1>
                        <p class="text-muted-foreground text-lg">"Searching for: " <span class="text-blue-400 font-bold">"\"" {q} "\""</span></p>
                    </div>
                </div>

                <Suspense fallback=move || view! { <div class="text-center py-20">"Searching..."</div> }>
                    {move || search_results.get().flatten().map(|items| {
                        if items.is_empty() {
                            return view! {
                                <div class="glass-card p-20 text-center rounded-3xl border border-white/5">
                                    <div class="text-6xl mb-6 opacity-30">"🏜️"</div>
                                    <h3 class="text-2xl font-bold text-muted-foreground">"No results found"</h3>
                                    <p class="mt-2">"Try searching with different keywords"</p>
                                </div>
                            }.into_view();
                        }
                        
                        view! {
                            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6">
                                {items.into_iter().enumerate().map(|(i, item)| view! {
                                    <SearchAnimeCard item=item index=i />
                                }).collect_view()}
                            </div>
                        }.into_view()
                    }).unwrap_or_else(|| view! { <div class="text-center py-20 text-muted-foreground">"Enter a search query to browse anime"</div> }.into_view())}
                </Suspense>
            </div>
        </main>
    }
}

#[component]
fn SearchAnimeCard(item: SearchAnimeItem, index: usize) -> impl IntoView {
    let delay = format!("animation-delay: {}s", index as f64 * 0.05);
    
    view! {
        <div class="group animate-fade-in" style=delay>
            <a href=format!("/anime/detail/{}", item.slug) class="block relative rounded-2xl overflow-hidden bg-card border border-border shadow-lg hover:shadow-2xl transition-all duration-500 hover:-translate-y-2">
                <div class="aspect-[3/4] overflow-hidden">
                    <img src=item.poster class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" alt=item.title.clone() />
                </div>
                <div class="absolute inset-0 bg-gradient-to-t from-black via-black/40 to-transparent opacity-80" />
                <div class="absolute bottom-0 left-0 right-0 p-4">
                    <h3 class="text-white text-xs font-bold line-clamp-2 mb-2 group-hover:text-blue-300 transition-colors">
                        {item.title}
                    </h3>
                    <div class="flex items-center gap-2">
                        <span class="px-2 py-0.5 rounded-md bg-white/10 text-[10px] text-white font-medium backdrop-blur-md">
                            {item.episode}
                        </span>
                        <span class="px-2 py-0.5 rounded-md bg-yellow-500/90 text-[10px] text-black font-black">
                            {item.status}
                        </span>
                    </div>
                </div>
            </a>
        </div>
    }
}
