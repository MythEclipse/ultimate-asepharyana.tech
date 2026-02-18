use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::komik::fetch_komik_detail;

#[component]
pub fn KomikDetailPage() -> impl IntoView {
    let query = use_query_map();
    let komik_id = move || query.get().get("komik_id").cloned().unwrap_or_default();
    
    let komik_data = create_resource(komik_id, |id| async move {
        if id.is_empty() { return None; }
        fetch_komik_detail(id).await.ok()
    });

    view! {
        <main class="min-h-screen bg-background text-foreground pb-20">
            <Suspense fallback=move || view! { <div class="p-20 text-center">"Loading komik details..."</div> }>
                {move || komik_data.get().flatten().map(|data| view! {
                    <Title text=format!("{} | Asepharyana", data.title)/>
                    
                    <div class="relative pt-12">
                        <div class="container mx-auto px-4 relative z-10">
                            <div class="flex flex-col md:flex-row gap-12">
                                // Sidebar with Poster info
                                <div class="w-full md:w-80 shrink-0 space-y-6">
                                    <div class="aspect-[3/4] rounded-2xl overflow-hidden shadow-2xl border border-white/10 group">
                                        <img src=data.poster.clone() class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-700" alt=data.title.clone() />
                                    </div>
                                    <div class="glass-card p-6 rounded-2xl space-y-4 border border-white/10">
                                        <InfoBlock label="Status" value=data.status.clone() icon="📌" />
                                        <InfoBlock label="Type" value=data.r#type.clone() icon="🏷️" />
                                        <InfoBlock label="Author" value=data.author.clone() icon="✍️" />
                                        <InfoBlock label="Released" value=data.release_date.clone() icon="📅" />
                                    </div>
                                </div>

                                // Main Detail Area
                                <div class="flex-1 space-y-12">
                                    <div>
                                        <h1 class="text-4xl md:text-6xl font-black mb-6 leading-tight gradient-text">
                                            {data.title.clone()}
                                        </h1>
                                        <div class="flex flex-wrap gap-2 mb-8">
                                            {data.genres.iter().map(|g| view! {
                                                <span class="px-4 py-1.5 rounded-xl bg-orange-500/10 border border-orange-500/20 text-orange-400 text-sm font-bold">
                                                    {g.clone()}
                                                </span>
                                            }).collect_view()}
                                        </div>
                                        
                                        <section class="glass-subtle p-8 rounded-3xl border border-white/5 relative overflow-hidden">
                                            <div class="absolute -top-10 -right-10 text-9xl opacity-5 select-none">"🗯️"</div>
                                            <h2 class="text-2xl font-bold mb-4">"Summary"</h2>
                                            <p class="text-muted-foreground leading-relaxed text-lg whitespace-pre-line">
                                                {data.description.clone()}
                                            </p>
                                        </section>
                                    </div>

                                    // Chapters
                                    <section>
                                        <div class="flex items-center justify-between mb-8">
                                            <h2 class="text-3xl font-bold flex items-center gap-3">
                                                <span class="w-3 h-10 bg-orange-500 rounded-full" />
                                                "Chapter List"
                                            </h2>
                                            <span class="px-4 py-2 rounded-2xl bg-white/5 text-sm font-medium border border-white/10">
                                                {data.chapters.len()} " Chapters"
                                            </span>
                                        </div>

                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-[800px] overflow-y-auto pr-2 custom-scrollbar">
                                            {data.chapters.iter().map(|ch| view! {
                                                <a 
                                                    href=format!("/komik/read/{}", ch.chapter_id)
                                                    class="flex items-center justify-between p-5 rounded-2xl glass-card hover:bg-orange-500/10 border border-white/5 hover:border-orange-500/30 transition-all group"
                                                >
                                                    <div class="flex flex-col">
                                                        <span class="font-black text-lg group-hover:text-orange-400 transition-colors">
                                                            "Chapter " {ch.chapter.clone()}
                                                        </span>
                                                        <span class="text-xs text-muted-foreground mt-1">
                                                            {ch.date.clone()}
                                                        </span>
                                                    </div>
                                                    <div class="w-12 h-12 rounded-xl bg-orange-500/10 flex items-center justify-center text-orange-400 group-hover:bg-orange-500 group-hover:text-white transition-all shadow-lg">
                                                        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                                                        </svg>
                                                    </div>
                                                </a>
                                            }).collect_view()}
                                        </div>
                                    </section>
                                </div>
                            </div>
                        </div>
                    </div>
                }).collect_view()}
            </Suspense>
        </main>
    }
}

#[component]
fn InfoBlock(label: &'static str, value: String, icon: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 group">
            <span class="text-[10px] uppercase tracking-widest text-muted-foreground font-black flex items-center gap-2">
                <span>{icon}</span> {label}
            </span>
            <p class="font-bold text-sm bg-white/5 p-2 rounded-lg border border-white/5 transition-colors group-hover:bg-white/10">
                {value}
            </p>
        </div>
    }
}
