use leptos::*;
use leptos_meta::Title;
use crate::components::sosmed::post_card::PostCard;
use crate::components::sosmed::create_post::CreatePost;
use crate::api::social::get_posts;

#[component]
pub fn SosmedPage() -> impl IntoView {
    let posts_resource = create_resource(|| (), |_| async move { get_posts().await });

    let handle_refresh = move |_: crate::types::Post| {
        posts_resource.refetch();
    };

    let handle_delete = move |post_id: String| {
        logging::log!("Refetching after delete of {}", post_id);
        posts_resource.refetch();
    };

    let handle_post_created = move |_: ()| {
        posts_resource.refetch();
    };

    view! {
        <Title text="Social Feed | Digital Agora"/>
        <main class="min-h-screen relative overflow-hidden pb-40">
            // Background Ambient Accents
            <div class="fixed inset-0 pointer-events-none z-0">
                <div class="absolute top-[20%] right-[10%] w-[40rem] h-[40rem] bg-indigo-500/5 rounded-full blur-[120px] animate-tilt" />
                <div class="absolute bottom-[10%] left-[5%] w-[35rem] h-[35rem] bg-purple-500/5 rounded-full blur-[120px] animate-tilt-reverse" />
            </div>

            <div class="container mx-auto max-w-2xl px-6 py-24 space-y-16 relative z-10">
                // Cinematic Header
                <header class="text-center space-y-6 animate-fade-in">
                    <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.2em] text-indigo-400">
                        "Pulse Engine"
                    </div>
                    <h1 class="text-5xl font-black italic tracking-tighter uppercase leading-none">
                        "Social " <span class="text-indigo-500">"Feed"</span>
                    </h1>
                    <div class="w-12 h-1 bg-gradient-to-r from-indigo-500 to-purple-500 mx-auto rounded-full" />
                </header>
                
                <section class="animate-slide-up [animation-delay:200ms]">
                    <CreatePost on_post_created=handle_post_created />
                </section>

                <Suspense fallback=move || view! { 
                    <div class="space-y-8">
                        {(0..3).map(|_| view! { <div class="h-64 rounded-3xl bg-white/5 animate-pulse border border-white/5" /> }).collect_view()}
                    </div>
                }>
                    {move || posts_resource.get().map(|res| match res {
                        Ok(posts) => if posts.is_empty() {
                            view! { 
                                <div class="glass-card p-24 text-center rounded-[2.5rem] border border-white/10 space-y-8 animate-fade-in">
                                    <div class="w-24 h-24 rounded-[2rem] bg-white/5 border border-white/5 flex items-center justify-center text-5xl mx-auto shadow-2xl">"🌌"</div>
                                    <div class="space-y-3">
                                        <h3 class="text-2xl font-black uppercase tracking-tighter italic">"Radio Silence"</h3>
                                        <p class="text-muted-foreground/60 font-medium italic leading-relaxed">"The digital agora is quiet. Be the first to broadcast into the void."</p>
                                    </div>
                                </div> 
                            }.into_view()
                        } else {
                            view! {
                                <div class="space-y-8">
                                    {posts.into_iter().enumerate().map(|(i, post)| view! {
                                        <div 
                                            class="animate-slide-up opacity-0 fill-mode-forwards" 
                                            style=format!("animation-delay: {}ms", i * 100 + 400)
                                        >
                                            <PostCard
                                                post=post
                                                on_post_updated=handle_refresh
                                                on_delete=handle_delete
                                            />
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        },
                        Err(_) => view! { 
                            <div class="glass-card p-12 text-center rounded-3xl border border-red-500/20 bg-red-500/5">
                                <span class="text-4xl mb-4 block">"⚠️"</span>
                                <p class="text-red-400 font-black uppercase tracking-widest text-[10px]">"Uplink Failure"</p>
                                <p class="text-muted-foreground mt-2">"Unable to synchronize feed. Attempting reconnection..."</p>
                            </div> 
                        }.into_view()
                    })}
                </Suspense>
            </div>
        </main>
    }
}
