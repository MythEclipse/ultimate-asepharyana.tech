use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::api::komik::fetch_chapter;
use urlencoding;

#[component]
pub fn ReadPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.get().get("slug").cloned().unwrap_or_default();
    
    let chapter_data = create_resource(slug, |s| async move {
        if s.is_empty() { return None; }
        fetch_chapter(s).await.ok()
    });

    let (show_controls, set_show_controls) = create_signal(true);

    view! {
        <main class="min-h-screen bg-neutral-950 text-white selection:bg-orange-500/30">
            <Suspense fallback=move || view! { 
                <div class="h-screen flex flex-col items-center justify-center space-y-6 animate-pulse">
                    <div class="w-20 h-20 rounded-3xl bg-white/5 border border-white/10 flex items-center justify-center text-3xl">"📖"</div>
                    <div class="text-xs font-black uppercase tracking-[0.4em] opacity-40">"Synchronizing Scroll"</div>
                </div> 
            }>
                {move || chapter_data.get().flatten().map(|data| view! {
                    <Title text=format!("{} | Reader Mode", data.title)/>
                    
                    // Cinematic Auto-hiding Header
                    <nav class=move || format!("fixed top-0 left-0 right-0 z-[60] transition-all duration-700 ease-in-out {}", if show_controls.get() { "translate-y-0 opacity-100" } else { "-translate-y-full opacity-0" })>
                        <div class="glass border-b border-white/10 py-6 px-10 shadow-2xl">
                            <div class="max-w-7xl mx-auto flex items-center justify-between">
                                <div class="flex items-center gap-8">
                                    <a href=if !data.list_chapter.is_empty() { data.list_chapter.clone() } else { "/komik".to_string() } class="group flex items-center gap-3 px-5 py-2.5 rounded-2xl glass-subtle border border-white/10 hover:border-orange-500/30 transition-all">
                                        <svg class="w-5 h-5 group-hover:-translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7" />
                                        </svg>
                                        <span class="text-[10px] font-black uppercase tracking-widest hidden md:block">"Exit Reader"</span>
                                    </a>
                                    <div class="space-y-1">
                                        <p class="text-[10px] font-black uppercase tracking-[0.2em] text-orange-500">"Now Reading"</p>
                                        <h1 class="font-black italic text-lg truncate max-w-md uppercase tracking-tight">{data.title.clone()}</h1>
                                    </div>
                                </div>
                                <div class="flex items-center gap-4">
                                    <div class="w-12 h-12 rounded-2xl bg-orange-500/20 flex items-center justify-center text-xl shadow-2xl">"👁️"</div>
                                </div>
                            </div>
                        </div>
                    </nav>

                    // Focus Perspective Images
                    <div 
                        class="max-w-4xl mx-auto pt-40 pb-40 relative group cursor-pointer"
                        on:click=move |_| set_show_controls.update(|v| *v = !*v)
                    >
                        // Background Ambient
                        <div class="fixed inset-0 pointer-events-none z-0">
                            <div class="absolute inset-0 bg-gradient-to-b from-orange-500/5 via-transparent to-transparent opacity-20" />
                        </div>

                        <div class="space-y-2 relative z-10">
                            {data.images.iter().enumerate().map(|(i, img)| view! {
                                <div class="relative w-full group/img overflow-hidden transition-all duration-1000">
                                    <img 
                                        src=img.clone() 
                                        class="w-full h-auto opacity-0 animate-fade-in fill-mode-forwards" 
                                        style=format!("animation-delay: {}ms", i % 5 * 100)
                                        loading="lazy" 
                                        alt=format!("Page {}", i + 1) 
                                    />
                                    // Side Progress Indicator
                                    <div class="absolute top-1/2 -right-12 group-hover/img:right-6 transition-all duration-700 opacity-0 group-hover/img:opacity-100">
                                        <div class="glass-card px-3 py-1.5 rounded-xl border border-white/20 text-[10px] font-black text-white/40">
                                            {format!("{:02}", i + 1)}
                                        </div>
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>
                    </div>

                    // Cinematic Auto-hiding Bottom Navigation
                    <footer class=move || format!("fixed bottom-0 left-0 right-0 z-[60] transition-all duration-700 ease-in-out {}", if show_controls.get() { "translate-y-0 opacity-100" } else { "translate-y-full opacity-0" })>
                        <div class="glass border-t border-white/10 p-8 shadow-[0_-50px_100px_rgba(0,0,0,0.5)]">
                            <div class="max-w-3xl mx-auto flex items-center justify-between gap-8">
                                <div class="flex-1">
                                    {if !data.prev_chapter_id.is_empty() {
                                        view! {
                                            <a href=format!("/komik/read/{}", urlencoding::encode(&data.prev_chapter_id)) class="group flex items-center gap-4 px-10 py-5 rounded-[2rem] glass-subtle border border-white/10 hover:border-white/30 transition-all font-black uppercase text-[10px] tracking-widest hover:-translate-x-2">
                                                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M15 19l-7-7 7-7" />
                                                </svg>
                                                "Prev Chapter"
                                            </a>
                                        }.into_view()
                                    } else {
                                        view! { <div class="px-10 py-5 rounded-[2rem] bg-white/5 opacity-20 border border-dashed border-white/10" /> }.into_view()
                                    }}
                                </div>

                                <div class="px-8 flex flex-col items-center">
                                    <span class="text-[8px] font-black uppercase tracking-[0.4em] text-orange-500 opacity-60">"Scroll"</span>
                                    <div class="w-1 h-8 bg-gradient-to-b from-orange-500 to-transparent rounded-full animate-pulse mt-2" />
                                </div>

                                <div class="flex-1 flex justify-end">
                                    {if !data.next_chapter_id.is_empty() {
                                        view! {
                                            <a href=format!("/komik/read/{}", urlencoding::encode(&data.next_chapter_id)) class="group flex items-center gap-4 px-10 py-5 rounded-[2rem] bg-orange-500 text-white hover:scale-105 active:scale-95 transition-all font-black uppercase text-[10px] tracking-widest shadow-2xl shadow-orange-500/20 hover:translate-x-2">
                                                "Next Chapter"
                                                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M9 5l7 7-7 7" />
                                                </svg>
                                            </a>
                                        }.into_view()
                                    } else {
                                        view! { <div class="px-10 py-5 rounded-[2rem] bg-orange-500/20 opacity-50 border border-dashed border-orange-500/30" /> }.into_view()
                                    }}
                                </div>
                            </div>
                        </div>
                    </footer>

                    // High-end End Marker
                    <div class="py-40 text-center space-y-8 animate-fade-in relative overflow-hidden">
                        <div class="absolute inset-0 bg-gradient-to-t from-orange-500/5 to-transparent pointer-events-none" />
                        <div class="relative z-10 flex flex-col items-center gap-6">
                            <div class="w-20 h-20 rounded-[2.5rem] glass border border-white/10 flex items-center justify-center text-3xl shadow-2xl shadow-orange-500/10">
                                "🎉"
                            </div>
                            <div class="space-y-2">
                                <h3 class="text-3xl font-black italic uppercase tracking-tighter">"Adventure Continued"</h3>
                                <p class="text-muted-foreground/60 text-[10px] font-black uppercase tracking-[0.4em]">"Finish Reading Chapter"</p>
                            </div>
                            <div class="h-px w-20 bg-gradient-to-r from-transparent via-white/20 to-transparent" />
                            <p class="font-black text-sm uppercase italic tracking-tight opacity-40">{data.title}</p>
                        </div>
                    </div>

                }).collect_view()}
            </Suspense>
        </main>
    }
}
