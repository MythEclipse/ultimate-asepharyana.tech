use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::anime::fetch_anime_detail;

#[component]
pub fn AnimeDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    
    let anime_data = create_resource(slug, |s| async move {
        if s.is_empty() { return None; }
        fetch_anime_detail(s).await.ok()
    });

    view! {
        <main class="min-h-screen bg-background text-foreground pb-20">
            <Suspense fallback=move || view! { <div class="p-20 text-center">"Loading anime details..."</div> }>
                {move || anime_data.get().flatten().map(|data| view! {
                    <Title text=format!("{} | Asepharyana", data.title)/>
                    
                    // Banner/Hero Section
                    <div class="relative h-[400px] w-full overflow-hidden">
                        <div class="absolute inset-0 bg-cover bg-center blur-2xl opacity-30 scale-110" style=format!("background-image: url('{}')", data.poster) />
                        <div class="absolute inset-0 bg-gradient-to-t from-background via-background/50 to-transparent" />
                        
                        <div class="container mx-auto px-4 h-full flex items-end pb-12 relative z-10">
                            <div class="flex flex-col md:flex-row gap-8 items-center md:items-end">
                                <div class="w-64 aspect-[3/4] rounded-2xl shadow-2xl overflow-hidden border-4 border-white/10 shrink-0 transform -rotate-2 hover:rotate-0 transition-transform duration-500">
                                    <img src=data.poster.clone() class="w-full h-full object-cover" alt=data.title.clone() />
                                </div>
                                <div class="flex-1 text-center md:text-left">
                                    <h1 class="text-4xl md:text-6xl font-black mb-4 gradient-text drop-shadow-lg">
                                        {data.title.clone()}
                                    </h1>
                                    <p class="text-xl text-muted-foreground mb-6 font-medium italic">
                                        {data.alternative_title.clone()}
                                    </p>
                                    <div class="flex flex-wrap gap-3 justify-center md:justify-start">
                                        {data.genres.iter().map(|g| view! {
                                            <span class="px-4 py-1.5 rounded-full bg-blue-500/20 border border-blue-500/30 text-blue-400 text-sm font-bold backdrop-blur-md">
                                                {g.name.clone()}
                                            </span>
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="container mx-auto px-4 mt-8 grid grid-cols-1 lg:grid-cols-3 gap-12">
                        // Main Content
                        <div class="lg:col-span-2 space-y-12">
                            // Synopsis
                            <section class="glass-card p-8 rounded-3xl relative overflow-hidden group">
                                <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:scale-110 transition-transform duration-500">
                                    <span class="text-6xl">"📝"</span>
                                </div>
                                <h2 class="text-2xl font-bold mb-6 flex items-center gap-3">
                                    <span class="w-2 h-8 bg-blue-500 rounded-full" />
                                    "Synopsis"
                                </h2>
                                <p class="text-muted-foreground leading-relaxed text-lg text-justify whitespace-pre-line">
                                    {data.synopsis.clone()}
                                </p>
                            </section>

                            // Episode List
                            <section>
                                <h2 class="text-2xl font-bold mb-6 flex items-center gap-3">
                                    <span class="w-2 h-8 bg-purple-500 rounded-full" />
                                    "Episode List"
                                </h2>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                    {data.episode_lists.iter().map(|ep| {
                                        let ep_slug = ep.slug.clone();
                                        view! {
                                            <a 
                                                href=format!("/anime/watch/{}", ep_slug)
                                                class="flex items-center justify-between p-5 rounded-2xl glass-subtle hover:bg-white/10 border border-white/5 transition-all group"
                                            >
                                                <span class="font-medium group-hover:text-blue-400 transition-colors">
                                                    {ep.episode.clone()}
                                                </span>
                                                <div class="w-10 h-10 rounded-xl bg-blue-500/20 flex items-center justify-center text-blue-400 group-hover:bg-blue-500 group-hover:text-white transition-all">
                                                    <svg class="w-5 h-5 fill-current" viewBox="0 0 24 24">
                                                        <path d="M8 5v14l11-7z" />
                                                    </svg>
                                                </div>
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            </section>
                        </div>

                        // Sidebar
                        <div class="space-y-12">
                            // Info Card
                            <section class="glass-card p-6 rounded-3xl border border-white/10">
                                <h3 class="text-xl font-bold mb-6 pb-4 border-b border-white/5">"Information"</h3>
                                <div class="space-y-5">
                                    <InfoItem label="Status" value=data.status.clone().unwrap_or_default() icon="📊" />
                                    <InfoItem label="Type" value=data.r#type.clone().unwrap_or_default() icon="📺" />
                                    <InfoItem label="Released" value=data.release_date.clone() icon="📅" />
                                    <InfoItem label="Studio" value=data.studio.clone() icon="🏢" />
                                </div>
                            </section>

                            // Recommendations
                            <section>
                                <h3 class="text-xl font-bold mb-6">"Recommendations"</h3>
                                <div class="space-y-4">
                                    {data.recommendations.iter().take(4).map(|rec| view! {
                                        <a href=format!("/anime/detail/{}", rec.slug) class="flex gap-4 p-3 rounded-2xl hover:bg-white/5 transition-all group">
                                            <div class="w-20 aspect-[3/4] rounded-xl overflow-hidden shadow-lg shrink-0">
                                                <img src=rec.poster.clone() class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500" />
                                            </div>
                                            <div class="flex-1 py-1">
                                                <h4 class="font-bold text-sm line-clamp-2 group-hover:text-blue-400 transition-colors">{rec.title.clone()}</h4>
                                                <span class="text-xs text-muted-foreground mt-2 block">"Series"</span>
                                            </div>
                                        </a>
                                    }).collect_view()}
                                </div>
                            </section>
                        </div>
                    </div>
                }).collect_view()}
            </Suspense>
        </main>
    }
}

#[component]
fn InfoItem(label: &'static str, value: String, icon: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-4 group">
            <div class="w-10 h-10 rounded-xl bg-background/50 flex items-center justify-center text-lg border border-white/5 group-hover:scale-110 transition-transform duration-300">
                {icon}
            </div>
            <div>
                <p class="text-[10px] uppercase tracking-widest text-muted-foreground font-black">
                    {label}
                </p>
                <p class="font-bold text-sm">{value}</p>
            </div>
        </div>
    }
}
