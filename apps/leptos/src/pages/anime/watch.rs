use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::anime::fetch_anime_stream;

#[component]
#[component]
pub fn WatchPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    
    let location = use_location();
    let is_anime2 = move || location.pathname.get().contains("/anime2/");

    let stream_data = create_resource(
        move || (slug(), is_anime2()), 
        |(s, is_a2)| async move {
            if s.is_empty() { return None; }
            if is_a2 {
                crate::api::anime::fetch_anime2_stream(s).await.ok()
            } else {
                fetch_anime_stream(s).await.ok()
            }
        }
    );

    view! {
        <main class="min-h-screen bg-background text-foreground pb-20">
            <Suspense fallback=move || view! { <div class="p-20 text-center">"Loading stream..."</div> }>
                {move || stream_data.get().flatten().map(|data| {
                    let base_watch_path = if is_anime2() { "/anime2/watch" } else { "/anime/watch" };

                    view! {
                    <Title text=format!("{} | Asepharyana", data.episode)/>
                    
                    <div class="container mx-auto px-4 pt-8">
                        // Video Player container
                        <div class="aspect-video w-full rounded-2xl overflow-hidden bg-black mb-8 shadow-2xl relative group border border-white/10">
                            <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 to-purple-500 opacity-20 group-hover:opacity-40 blur transition-opacity duration-500"></div>
                            <iframe
                                src=data.stream_url
                                class="w-full h-full relative z-10"
                                allowfullscreen
                                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            ></iframe>
                        </div>

                        // Episode Title
                        <h1 class="text-2xl md:text-3xl font-bold gradient-text mb-6">
                            {data.episode.clone()}
                        </h1>

                        // Navigation
                        <div class="flex items-center justify-between gap-4 glass-card rounded-xl p-4 mb-8">
                            <div class="flex-1">
                                {move || if data.has_previous_episode && data.previous_episode.is_some() {
                                    let prev = data.previous_episode.clone().unwrap();
                                    view! {
                                        <a href=format!("{}/{}", base_watch_path, prev.slug) class="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 transition-colors w-fit">
                                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                            </svg>
                                            "Previous"
                                        </a>
                                    }.into_view()
                                } else {
                                    view! { <div/> }.into_view()
                                }}
                            </div>

                            <div class="text-center">
                                <span class="text-sm text-muted-foreground">"Episode"</span>
                                <p class="font-bold text-lg">{data.episode_number.clone()}</p>
                            </div>

                            <div class="flex-1 flex justify-end">
                                {move || if data.has_next_episode && data.next_episode.is_some() {
                                    let next = data.next_episode.clone().unwrap();
                                    view! {
                                        <a href=format!("{}/{}", base_watch_path, next.slug) class="flex items-center gap-2 px-4 py-2 rounded-lg bg-blue-500 hover:bg-blue-600 transition-colors w-fit">
                                            "Next"
                                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                            </svg>
                                        </a>
                                    }.into_view()
                                } else {
                                    view! { <div/> }.into_view()
                                }}
                            </div>
                        </div>

                        // Download Links
                        <div class="glass-card rounded-2xl overflow-hidden mb-12">
                            <div class="bg-white/5 p-5 border-b border-white/5">
                                <h3 class="text-xl font-bold flex items-center gap-3">
                                    <span class="text-2xl">"⬇️"</span>
                                    "Download Episode"
                                </h3>
                            </div>
                            <div class="p-5 space-y-4">
                                {data.download_urls.iter().map(|(res, links)| view! {
                                    <div class="bg-white/5 rounded-xl p-4 border border-white/5">
                                        <div class="font-bold mb-3 flex items-center gap-2">
                                            <span class="px-3 py-1 bg-blue-500/20 text-blue-400 rounded-lg text-sm">{res}</span>
                                        </div>
                                        <div class="flex flex-wrap gap-2">
                                            {links.iter().map(|link| view! {
                                                <a href=link.url.clone() target="_blank" class="px-4 py-2 bg-white/10 hover:bg-blue-500/80 rounded-lg text-sm font-medium transition-colors">
                                                    {link.server.clone()}
                                                </a>
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }).collect_view()}
                            </div>
                        </div>
                    </div>
                }}).collect_view()}
            </Suspense>
        </main>
    }
}
