use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use crate::api::anime::fetch_anime_stream;
use crate::providers::use_auth;

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
        <main class="min-h-screen relative overflow-hidden pb-32">
            <Suspense fallback=move || view! { 
                <div class="max-w-7xl mx-auto px-6 pt-12 space-y-12 animate-pulse">
                    <div class="aspect-video w-full bg-white/5 rounded-[3rem]" />
                    <div class="h-12 w-1/2 bg-white/5 rounded-2xl" />
                    <div class="h-24 w-full bg-white/5 rounded-3xl" />
                </div>
            }>
                {move || stream_data.get().flatten().map(|data| {
                    let base_watch_path = if is_anime2() { "/anime2/watch" } else { "/anime/watch" };

                    view! {
                    <Title text=format!("Watching {} | Anime Hub", data.episode)/>
                    
                    <div class="max-w-7xl mx-auto px-6 pt-12 space-y-12">
                        // Cinematic Immersion Header
                        <div class="flex flex-col md:flex-row md:items-end justify-between gap-8 animate-slide-up">
                            <div class="space-y-4">
                                <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-widest text-blue-400">
                                    "Now Streaming"
                                </div>
                                <h1 class="text-4xl md:text-6xl font-black italic tracking-tighter uppercase leading-tight">
                                    {data.episode.clone()}
                                </h1>
                            </div>
                            
                            <div class="flex items-center gap-4">
                                <div class="text-right hidden md:block">
                                    <p class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/60">"Chapter Index"</p>
                                    <p class="text-2xl font-black italic text-blue-500">
                                        "EP. " {data.episode_number.clone()}
                                    </p>
                                </div>
                                <div class="w-16 h-16 rounded-2xl bg-blue-500/20 flex items-center justify-center text-3xl shadow-2xl relative">
                                    <span class="relative z-10">"💎"</span>
                                    <div class="absolute inset-0 bg-blue-500/20 blur-xl animate-pulse" />
                                </div>
                            </div>
                        </div>

                        // Premium Video Player with Ambient Glow
                        <div class="relative group perspective-1000 animate-fade-in [animation-delay:200ms]">
                            // Ambient Glow Effect
                            <div class="absolute -inset-4 bg-gradient-to-r from-blue-600/10 via-purple-600/10 to-blue-600/10 rounded-[4rem] blur-[80px] opacity-40 group-hover:opacity-60 transition-opacity duration-1000" />
                            
                            <div class="relative aspect-video w-full rounded-[3rem] overflow-hidden bg-black border-4 border-white/10 shadow-[0_50px_100px_rgba(0,0,0,0.6)] group-hover:border-blue-500/30 transition-all duration-700">
                                <iframe
                                    src=data.stream_url
                                    class="w-full h-full"
                                    allowfullscreen
                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                                ></iframe>
                                
                                // Interactive Overlay (Visible on hover)
                                <div class="absolute inset-0 pointer-events-none bg-gradient-to-t from-black/60 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                            </div>
                        </div>

                        // Cinematic Navigation Module
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 animate-slide-up [animation-delay:400ms]">
                            <div class="flex-1">
                                {move || if data.has_previous_episode && data.previous_episode.is_some() {
                                    let prev = data.previous_episode.clone().unwrap();
                                    view! {
                                        <a href=format!("{}/{}", base_watch_path, prev.slug) class="flex items-center justify-between px-8 py-6 rounded-[2rem] glass border border-white/5 hover:border-white/20 hover:bg-white/5 transition-all group lg:min-w-[280px]">
                                            <div class="flex items-center gap-4">
                                                <div class="w-12 h-12 rounded-2xl bg-white/5 flex items-center justify-center text-xl group-hover:bg-blue-500 group-hover:text-white transition-all">
                                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M15 19l-7-7 7-7" />
                                                    </svg>
                                                </div>
                                                <div class="text-left">
                                                    <p class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/60">"Go Back"</p>
                                                    <p class="font-black text-sm uppercase italic">"Previous"</p>
                                                </div>
                                            </div>
                                        </a>
                                    }.into_view()
                                } else {
                                    view! { <div class="px-8 py-6 rounded-[2rem] bg-white/2 opacity-30 border border-dashed border-white/10" /> }.into_view()
                                }}
                            </div>

                            <div class="glass flex items-center justify-center px-12 py-6 rounded-[2rem] border border-white/10 shadow-xl overflow-hidden relative">
                                <div class="absolute inset-0 bg-blue-500/5 blur-3xl" />
                                <div class="relative z-10 text-center space-y-1">
                                    <span class="text-[10px] font-black uppercase tracking-[0.4em] text-blue-500">"Session"</span>
                                    <p class="text-2xl font-black italic tracking-tighter">
                                        "EP. "{data.episode_number.clone()}
                                    </p>
                                </div>
                            </div>

                            <div class="flex-1 flex justify-end">
                                {move || if data.has_next_episode && data.next_episode.is_some() {
                                    let next = data.next_episode.clone().unwrap();
                                    view! {
                                        <a href=format!("{}/{}", base_watch_path, next.slug) class="flex items-center justify-between px-8 py-6 rounded-[2rem] bg-foreground text-background hover:scale-105 active:scale-95 transition-all group lg:min-w-[280px] shadow-2xl">
                                            <div class="text-right">
                                                <p class="text-[10px] font-black uppercase tracking-[0.2em] opacity-40">"Continue"</p>
                                                <p class="font-black text-sm uppercase italic">"Next Episode"</p>
                                            </div>
                                            <div class="w-12 h-12 rounded-2xl bg-background/10 flex items-center justify-center text-xl group-hover:translate-x-1 transition-all">
                                                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M9 5l7 7-7 7" />
                                                </svg>
                                            </div>
                                        </a>
                                    }.into_view()
                                } else {
                                    view! { <div class="px-8 py-6 rounded-[2rem] bg-white/2 opacity-30 border border-dashed border-white/10" /> }.into_view()
                                }}
                            </div>
                        </div>

                        // Cinematic Download Vault
                        <section class="space-y-8 animate-fade-in [animation-delay:600ms]">
                            <div class="flex items-center gap-4">
                                <div class="w-12 h-12 rounded-2xl bg-orange-500/20 flex items-center justify-center text-2xl shadow-2xl shadow-orange-500/10">"💾"</div>
                                <h3 class="text-3xl font-black uppercase tracking-tighter italic">"Download Vault"</h3>
                            </div>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                {data.download_urls.iter().map(|(res, links)| view! {
                                    <div class="glass-card rounded-[2.5rem] p-8 space-y-6 border border-white/10 relative overflow-hidden group">
                                        <div class="absolute -right-10 -top-10 w-40 h-40 bg-blue-500/5 rounded-full blur-3xl opacity-0 group-hover:opacity-100 transition-opacity" />
                                        
                                        <div class="flex items-center justify-between border-b border-white/5 pb-4">
                                            <div class="space-y-1">
                                                <span class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground/60">"Resolution Level"</span>
                                                <p class="text-2xl font-black italic text-blue-400">{res}</p>
                                            </div>
                                            <div class="px-4 py-2 rounded-xl bg-blue-500/20 text-blue-400 text-[10px] font-black uppercase tracking-widest border border-blue-500/30">
                                                "Active Mirrors"
                                            </div>
                                        </div>
                                        
                                        <div class="grid grid-cols-2 lg:grid-cols-3 gap-3">
                                            {links.iter().map(|link| view! {
                                                <a href=link.url.clone() target="_blank" class="flex items-center justify-center px-4 py-3 rounded-2xl glass-subtle border border-white/5 hover:border-blue-500/50 hover:bg-blue-500/10 hover:text-blue-400 transition-all text-[11px] font-black uppercase tracking-widest text-center shadow-lg group/link">
                                                    <span class="group-hover/link:translate-y-[-1px] transition-transform">{link.server.clone()}</span>
                                                </a>
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }).collect_view()}
                            </div>
                        </section>
                    </div>
                }}).collect_view()}
            </Suspense>
        </main>
    }
}
