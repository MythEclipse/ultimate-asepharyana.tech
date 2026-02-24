use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::anime::{fetch_anime_detail, EpisodeList};
use crate::components::ui::CachedImage;

#[component]
pub fn AnimeDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    
    let location = use_location();
    let is_anime2 = move || location.pathname.get().contains("/anime2/");

    let anime_data = create_resource(
        move || (slug(), is_anime2()), 
        |(s, is_a2)| async move {
            if s.is_empty() { return None; }
            if is_a2 {
                crate::api::anime::fetch_anime2_detail(s).await.ok()
            } else {
                fetch_anime_detail(s).await.ok()
            }
        }
    );

    view! {
        <main class="min-h-screen relative overflow-hidden">
            <Suspense fallback=move || view! { 
                <div class="p-24 text-center space-y-8 animate-pulse">
                    <div class="h-[400px] w-full bg-muted/50 rounded-[3rem]" />
                    <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-3 gap-12">
                        <div class="lg:col-span-2 h-96 bg-muted/50 rounded-[2rem]" />
                        <div class="h-96 bg-muted/50 rounded-[2rem]" />
                    </div>
                </div>
            }>
                {move || anime_data.get().flatten().map(|data| {
                    let episodes = if !data.episode_lists.is_empty() {
                        data.episode_lists.clone()
                    } else if !data.downloads.is_empty() {
                        data.downloads.iter().map(|d| {
                            let ep_num = d.resolution.replace("Episode ", "").replace(" ", "");
                            EpisodeList {
                                episode: d.resolution.clone(),
                                slug: format!("{}-episode-{}", slug(), ep_num),
                            }
                        }).collect()
                    } else {
                        vec![]
                    };

                    let base_watch_path = if is_anime2() { "/anime2/watch" } else { "/anime/watch" };
                    let base_detail_path = if is_anime2() { "/anime2/detail" } else { "/anime/detail" };

                    view! {
                    <Title text=format!("{} | Anime Details", data.title)/>
                    
                    // Cinematic Cinema Banner
                    <div class="relative h-[500px] md:h-[600px] w-full overflow-hidden">
                        // Blurred Background
                        <div class="absolute inset-0 z-0 opacity-40 scale-125 blur-[100px]">
                            <CachedImage src=data.poster.clone() alt="".to_string() />
                            <div class="absolute inset-0 bg-gradient-to-t from-background via-background/40 to-transparent" />
                        </div>
                        
                        <div class="container mx-auto px-6 h-full flex items-end pb-16 relative z-10">
                            <div class="flex flex-col md:flex-row gap-12 items-center md:items-end w-full">
                                // Sharp Floating Poster
                                <div class="w-64 md:w-80 aspect-[3/4.2] rounded-[2.5rem] shadow-[0_50px_100px_rgba(0,0,0,0.5)] overflow-hidden border-2 border-border/50 shrink-0 transform -rotate-2 hover:rotate-0 transition-all duration-700 hover-tilt">
                                    <CachedImage src=data.poster.clone() alt=data.title.clone() class="w-full h-full object-cover".to_string() />
                                </div>
                                
                                <div class="flex-1 text-center md:text-left space-y-6 animate-slide-up">
                                    <div class="space-y-2">
                                        <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-border/10 text-[10px] font-black uppercase tracking-widest text-primary">
                                            "Series Details"
                                        </div>
                                        <h1 class="text-5xl md:text-7xl font-black italic tracking-tighter leading-none [text-shadow:0_10px_30px_rgba(0,0,0,0.5)]">
                                            {data.title.clone()}
                                        </h1>
                                        <p class="text-xl md:text-2xl text-muted-foreground/80 font-bold italic opacity-60">
                                            {data.alternative_title.clone()}
                                        </p>
                                    </div>

                                    <div class="flex flex-wrap gap-3 justify-center md:justify-start">
                                        {data.genres.iter().map(|g| view! {
                                            <span class="px-5 py-2 rounded-xl glass border border-border/10 text-xs font-black uppercase tracking-widest hover:border-primary/40 transition-colors">
                                                {g.name.clone()}
                                            </span>
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="max-w-7xl mx-auto px-6 mt-16 grid grid-cols-1 lg:grid-cols-3 gap-16 pb-32">
                        // Main Cinematic Content
                        <div class="lg:col-span-2 space-y-20">
                            // Synopsis with Glass design
                            <section class="space-y-8 animate-fade-in">
                                <div class="flex items-center gap-4">
                                    <div class="w-12 h-12 rounded-2xl bg-blue-500/20 flex items-center justify-center text-2xl shadow-2xl shadow-blue-500/10">"📝"</div>
                                    <h2 class="text-3xl font-black uppercase tracking-tighter italic">"The Story"</h2>
                                </div>
                                <div class="glass-card p-10 rounded-[3rem] border border-border/10 relative overflow-hidden group">
                                    <div class="absolute -right-20 -top-20 w-64 h-64 bg-blue-500/5 rounded-full blur-[80px]" />
                                    <p class="text-muted-foreground/90 leading-relaxed text-lg font-medium text-justify whitespace-pre-line relative z-10">
                                        {data.synopsis.clone()}
                                    </p>
                                </div>
                            </section>

                            // Episode List with Premium Buttons
                            <section class="space-y-8 animate-fade-in [animation-delay:200ms]">
                                <div class="flex items-center gap-4">
                                    <div class="w-12 h-12 rounded-2xl bg-purple-500/20 flex items-center justify-center text-2xl shadow-2xl shadow-purple-500/10">"🎞️"</div>
                                    <h2 class="text-3xl font-black uppercase tracking-tighter italic">"Episodes"</h2>
                                </div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                    {episodes.iter().map(|ep| {
                                        let ep_slug = ep.slug.clone();
                                        view! {
                                            <a 
                                                href=format!("{}/{}", base_watch_path, ep_slug)
                                                class="group flex items-center justify-between p-6 rounded-[2rem] glass border border-border/5 hover:border-border/20 transition-all hover:scale-[1.03] active:scale-95 shadow-xl hover:shadow-primary/10"
                                            >
                                                <div class="flex items-center gap-4">
                                                    <div class="w-12 h-12 rounded-2xl bg-muted flex items-center justify-center font-black text-primary group-hover:bg-primary group-hover:text-white transition-colors">
                                                        "▶"
                                                    </div>
                                                    <span class="font-black uppercase tracking-wide text-sm group-hover:text-primary transition-colors">
                                                        {ep.episode.clone()}
                                                    </span>
                                                </div>
                                                <svg class="w-6 h-6 opacity-0 group-hover:opacity-100 group-hover:translate-x-1 transition-all" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                                </svg>
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            </section>
                        </div>

                        // Cinematic Sidebar
                        <div class="space-y-20 animate-fade-in [animation-delay:400ms]">
                            // Info Module
                            <section class="space-y-8">
                                <h3 class="text-xl font-black uppercase tracking-[0.2em] opacity-40 italic">"Overview"</h3>
                                <div class="glass-card p-8 rounded-[3rem] border border-border/10 space-y-8 shadow-2xl">
                                    <InfoItem label="Current Status" value=data.status.clone().unwrap_or_default() icon="📊" accent="text-green-500" />
                                    <InfoItem label="Media Type" value=data.r#type.clone().unwrap_or_default() icon="📺" accent="text-blue-500" />
                                    <InfoItem label="Air Date" value=data.release_date.clone() icon="📅" accent="text-purple-500" />
                                    <InfoItem label="Production" value=data.studio.clone() icon="🏢" accent="text-pink-500" />
                                </div>
                            </section>

                            // Visual Recommendations
                            <section class="space-y-8">
                                <h3 class="text-xl font-black uppercase tracking-[0.2em] opacity-40 italic">"Related"</h3>
                                <div class="space-y-6">
                                    {data.recommendations.iter().take(5).map(|rec| view! {
                                        <a href=format!("{}/{}", base_detail_path, rec.slug) class="flex gap-6 p-4 rounded-[2rem] glass border border-border/5 hover:border-border/20 transition-all group shadow-xl">
                                            <div class="w-24 aspect-[3/4.2] rounded-2xl overflow-hidden shadow-2xl shrink-0 border border-border/10">
                                                <CachedImage src=rec.poster.clone() alt="".to_string() class="w-full h-full object-cover group-hover:scale-115 transition-transform duration-700".to_string() />
                                            </div>
                                            <div class="flex-1 flex flex-col justify-center gap-2">
                                                <h4 class="font-black text-sm uppercase tracking-tight line-clamp-2 group-hover:text-primary transition-colors leading-tight">{rec.title.clone()}</h4>
                                                <div class="h-1 w-8 bg-primary rounded-full scale-x-0 group-hover:scale-x-100 transition-transform origin-left" />
                                                <span class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/60">"Library Item"</span>
                                            </div>
                                        </a>
                                    }).collect_view()}
                                </div>
                            </section>
                        </div>
                    </div>
                }}).collect_view()}
            </Suspense>
        </main>
    }
}

#[component]
fn InfoItem(label: &'static str, value: String, icon: &'static str, accent: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-5 group">
            <div class=format!("w-14 h-14 rounded-2xl bg-muted flex items-center justify-center text-2xl border border-border/50 group-hover:scale-110 group-hover:bg-muted/80 transition-all duration-500 shadow-xl {}", accent)>
                {icon}
            </div>
            <div class="space-y-1">
                <p class="text-[10px] uppercase tracking-[0.2em] text-muted-foreground/60 font-black">
                    {label}
                </p>
                <p class="font-black text-base italic tracking-tight">{value}</p>
            </div>
        </div>
    }
}
