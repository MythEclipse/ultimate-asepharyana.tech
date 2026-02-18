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
        <main class="min-h-screen relative overflow-hidden pb-40">
            <Suspense fallback=move || view! { 
                <div class="max-w-7xl mx-auto px-6 pt-12 animate-pulse space-y-12">
                    <div class="flex flex-col md:flex-row gap-12">
                        <div class="w-80 h-[450px] bg-white/5 rounded-[2.5rem]" />
                        <div class="flex-1 space-y-8">
                            <div class="h-16 w-3/4 bg-white/5 rounded-2xl" />
                            <div class="h-32 w-full bg-white/5 rounded-3xl" />
                        </div>
                    </div>
                </div>
            }>
                {move || komik_data.get().flatten().map(|data| view! {
                    <Title text=format!("{} | Reader Hub", data.title)/>
                    
                    <div class="max-w-7xl mx-auto px-6 pt-12 relative z-10">
                        <div class="flex flex-col lg:flex-row gap-16 item-start">
                            // Sidebar with 3D Poster info
                            <div class="w-full lg:w-[22rem] shrink-0 space-y-12 animate-slide-up">
                                <div class="relative group perspective-1000">
                                    <div class="absolute -inset-4 bg-orange-500/10 rounded-[3rem] blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity duration-1000" />
                                    <div class="aspect-[3/4.2] rounded-[2.5rem] overflow-hidden shadow-[0_50px_100px_rgba(0,0,0,0.5)] border-2 border-white/20 transform -rotate-1 group-hover:rotate-0 transition-all duration-700 hover-tilt">
                                        <img src=data.poster.clone() class="w-full h-full object-cover" alt=data.title.clone() />
                                        <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-60" />
                                    </div>
                                </div>
                                
                                <div class="glass-card p-8 rounded-[3rem] border border-white/10 space-y-8 shadow-2xl relative overflow-hidden">
                                    <div class="absolute -right-10 -bottom-10 w-32 h-32 bg-orange-500/5 rounded-full blur-3xl" />
                                    <InfoBlock label="Serialization" value=data.status.clone() icon="📌" accent="text-orange-500" />
                                    <InfoBlock label="Archive Type" value=data.r#type.clone() icon="🏷️" accent="text-blue-500" />
                                    <InfoBlock label="Origin Author" value=data.author.clone() icon="✍️" accent="text-purple-500" />
                                    <InfoBlock label="Release Cycle" value=data.release_date.clone() icon="📅" accent="text-pink-500" />
                                </div>
                            </div>

                            // Main Detail Area
                            <div class="flex-1 space-y-16 animate-fade-in [animation-delay:200ms]">
                                <div class="space-y-8">
                                    <div class="space-y-4">
                                        <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.2em] text-orange-500">
                                            "Library Entry"
                                        </div>
                                        <h1 class="text-5xl md:text-8xl font-black italic tracking-tighter leading-none italic uppercase">
                                            {data.title.clone()}
                                        </h1>
                                    </div>

                                    <div class="flex flex-wrap gap-3">
                                        {data.genres.iter().map(|g| view! {
                                            <span class="px-5 py-2 rounded-xl glass border border-white/10 text-[10px] font-black uppercase tracking-widest hover:border-orange-500/30 hover:text-orange-500 transition-all">
                                                {g.clone()}
                                            </span>
                                        }).collect_view()}
                                    </div>
                                    
                                    <section class="space-y-6">
                                        <div class="flex items-center gap-4">
                                            <div class="w-10 h-10 rounded-xl bg-orange-500/20 flex items-center justify-center text-xl">"📖"</div>
                                            <h2 class="text-2xl font-black uppercase tracking-tighter italic">"Overview"</h2>
                                        </div>
                                        <div class="glass-subtle p-10 rounded-[3rem] border border-white/5 relative overflow-hidden group">
                                            <div class="absolute -top-12 -right-12 text-[12rem] text-white/5 font-black italic select-none">"🗯️"</div>
                                            <p class="text-muted-foreground/80 leading-relaxed text-lg font-medium whitespace-pre-line relative z-10 text-justify">
                                                {data.description.clone()}
                                            </p>
                                        </div>
                                    </section>
                                </div>

                                // Chapters Module
                                <section class="space-y-10">
                                    <div class="flex items-center justify-between">
                                        <div class="flex items-center gap-4">
                                            <div class="w-12 h-12 rounded-2xl bg-orange-500/20 flex items-center justify-center text-2xl shadow-2xl shadow-orange-500/10">"📑"</div>
                                            <h2 class="text-3xl font-black uppercase tracking-tighter italic">"Chapters"</h2>
                                        </div>
                                        <div class="px-6 py-3 rounded-2xl glass border border-white/10 text-xs font-black uppercase tracking-widest">
                                            {data.chapters.len()} " Collections"
                                        </div>
                                    </div>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-[800px] overflow-y-auto pr-4 custom-scrollbar">
                                        {data.chapters.iter().map(|ch| view! {
                                            <a 
                                                href=format!("/komik/read/{}", ch.chapter_id)
                                                class="flex items-center justify-between p-6 rounded-[2rem] glass border border-white/5 hover:border-orange-500/40 hover:bg-orange-500/5 transition-all group active:scale-95 shadow-xl hover:shadow-orange-500/10"
                                            >
                                                <div class="flex items-center gap-6">
                                                    <div class="w-14 h-14 rounded-2xl bg-white/5 flex flex-col items-center justify-center font-black group-hover:bg-orange-500 transition-all duration-500">
                                                        <span class="text-[8px] uppercase tracking-widest opacity-60 group-hover:text-white/80 group-hover:opacity-100">"CH"</span>
                                                        <span class="text-lg group-hover:text-white">{ch.chapter.clone()}</span>
                                                    </div>
                                                    <div class="flex flex-col">
                                                        <span class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground/60">"Released on"</span>
                                                        <span class="font-black text-xs uppercase tracking-tight group-hover:text-orange-400 transition-colors">
                                                            {ch.date.clone()}
                                                        </span>
                                                    </div>
                                                </div>
                                                <div class="w-10 h-10 rounded-xl bg-orange-500/10 flex items-center justify-center text-orange-500 opacity-0 group-hover:opacity-100 group-hover:translate-x-1 transition-all">
                                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                                                    </svg>
                                                </div>
                                            </a>
                                        }).collect_view()}
                                    </div>
                                </section>
                            </div>
                        </div>
                    </div>
                }).collect_view()}
            </Suspense>
        </main>
    }
}

#[component]
fn InfoBlock(label: &'static str, value: String, icon: &'static str, accent: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-5 group">
            <div class=format!("w-12 h-12 rounded-2xl bg-white/5 flex items-center justify-center text-xl border border-white/5 transition-all duration-500 shadow-xl group-hover:scale-110 {}", accent)>
                {icon}
            </div>
            <div class="space-y-0.5">
                <p class="text-[10px] uppercase tracking-[0.2em] text-muted-foreground/60 font-black">
                    {label}
                </p>
                <p class="font-black text-sm italic tracking-tight">{value}</p>
            </div>
        </div>
    }
}
