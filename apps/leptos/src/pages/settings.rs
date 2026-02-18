use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::providers::{use_auth, use_theme, Theme};

#[component]
pub fn SettingsPage() -> impl IntoView {
    let auth = use_auth();
    let theme = use_theme();
    let navigate = use_navigate();

    // Redirect if not logged in
    create_effect(move |_| {
        if auth.user.get().is_none() {
             navigate("/login", Default::default());
        }
    });

    view! {
        <Title text="Settings | Neural Config"/>
        <Show when=move || auth.user.get().is_some()>
            <main class="min-h-screen relative overflow-hidden pb-40">
                // Background Ambient Systems
                <div class="fixed inset-0 pointer-events-none z-0">
                    <div class="absolute top-[20%] right-[10%] w-[40rem] h-[40rem] bg-indigo-500/5 rounded-full blur-[120px] animate-tilt" />
                    <div class="absolute bottom-[10%] left-[5%] w-[35rem] h-[35rem] bg-blue-500/5 rounded-full blur-[120px] animate-tilt-reverse" />
                </div>

                <div class="container mx-auto max-w-3xl px-8 pt-24 space-y-16 relative z-10">
                    // Cinematic Settings Header
                    <header class="text-center space-y-6 animate-fade-in">
                        <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.4em] text-indigo-400">
                            "System Parameters"
                        </div>
                        <h1 class="text-5xl font-black italic tracking-tighter uppercase leading-none">
                            "Neural " <span class="text-indigo-500">"Config"</span>
                        </h1>
                        <div class="w-12 h-1 bg-gradient-to-r from-indigo-500 to-blue-500 mx-auto rounded-full" />
                    </header>

                    <div class="space-y-12">
                        // Profile Intelligence Section
                        <section class="glass-card rounded-[3rem] p-10 border border-white/5 animate-slide-up [animation-delay:200ms] group hover:border-white/20 transition-all duration-700">
                            <div class="flex items-center gap-4 mb-10">
                                <div class="w-12 h-12 rounded-2xl bg-indigo-500/20 flex items-center justify-center text-2xl shadow-2xl">"🧠"</div>
                                <div class="space-y-1">
                                    <h2 class="text-2xl font-black italic tracking-tighter uppercase">"Profile" <span class="text-muted-foreground/40">"Identity"</span></h2>
                                    <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/30">"Current Instance Parameters"</p>
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                <div class="space-y-2 p-6 rounded-2xl bg-white/2 border border-white/5">
                                    <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">"Alias"</label>
                                    <p class="text-lg font-black italic tracking-tight">{move || auth.user.get().map(|u| u.name).unwrap_or_default()}</p>
                                </div>
                                <div class="space-y-2 p-6 rounded-2xl bg-white/2 border border-white/5">
                                    <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">"Uplink ID"</label>
                                    <p class="text-lg font-black italic tracking-tight">{move || auth.user.get().map(|u| u.email).unwrap_or_default()}</p>
                                </div>
                            </div>
                        </section>
                        
                        // Chromatic Architecture Section
                        <section class="glass-card rounded-[3rem] p-10 border border-white/5 animate-slide-up [animation-delay:400ms] group hover:border-white/20 transition-all duration-700">
                            <div class="flex items-center gap-4 mb-10">
                                <div class="w-12 h-12 rounded-2xl bg-blue-500/20 flex items-center justify-center text-2xl shadow-2xl">"🎨"</div>
                                <div class="space-y-1">
                                    <h2 class="text-2xl font-black italic tracking-tighter uppercase">"Chromatic" <span class="text-muted-foreground/40">"Engine"</span></h2>
                                    <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/30">"Visual Interface Management"</p>
                                </div>
                            </div>

                            <div class="grid grid-cols-1 sm:grid-cols-3 gap-6">
                                <button
                                    on:click=move |_| theme.set_theme.set(Theme::Light)
                                    class=move || format!(
                                        "relative group/btn p-8 rounded-[2rem] glass transition-all duration-500 active:scale-95 {}",
                                        if theme.theme.get() == Theme::Light { "bg-blue-500/10 border-blue-500/40 text-blue-400" } else { "border-white/5 text-muted-foreground hover:bg-white/5 hover:text-foreground" }
                                    )
                                >
                                    <div class="relative z-10 space-y-3">
                                        <span class="text-3xl block transition-transform group-hover/btn:scale-125 duration-500">"☀️"</span>
                                        <span class="text-[10px] font-black uppercase tracking-[0.2em]">"Light"</span>
                                    </div>
                                    <div class="absolute inset-x-0 bottom-0 h-1 bg-blue-500 scale-x-0 group-hover/btn:scale-x-50 transition-transform origin-center" />
                                </button>

                                <button
                                    on:click=move |_| theme.set_theme.set(Theme::Dark)
                                    class=move || format!(
                                        "relative group/btn p-8 rounded-[2rem] glass transition-all duration-500 active:scale-95 {}",
                                        if theme.theme.get() == Theme::Dark { "bg-indigo-500/10 border-indigo-500/40 text-indigo-400" } else { "border-white/5 text-muted-foreground hover:bg-white/5 hover:text-foreground" }
                                    )
                                >
                                    <div class="relative z-10 space-y-3">
                                        <span class="text-3xl block transition-transform group-hover/btn:scale-125 duration-500">"🌙"</span>
                                        <span class="text-[10px] font-black uppercase tracking-[0.2em]">"Dark"</span>
                                    </div>
                                    <div class="absolute inset-x-0 bottom-0 h-1 bg-indigo-500 scale-x-0 group-hover/btn:scale-x-50 transition-transform origin-center" />
                                </button>

                                <button
                                    on:click=move |_| theme.set_theme.set(Theme::System)
                                    class=move || format!(
                                        "relative group/btn p-8 rounded-[2rem] glass transition-all duration-500 active:scale-95 {}",
                                        if theme.theme.get() == Theme::System { "bg-purple-500/10 border-purple-500/40 text-purple-400" } else { "border-white/5 text-muted-foreground hover:bg-white/5 hover:text-foreground" }
                                    )
                                >
                                    <div class="relative z-10 space-y-3">
                                        <span class="text-3xl block transition-transform group-hover/btn:scale-125 duration-500">"💻"</span>
                                        <span class="text-[10px] font-black uppercase tracking-[0.2em]">"System"</span>
                                    </div>
                                    <div class="absolute inset-x-0 bottom-0 h-1 bg-purple-500 scale-x-0 group-hover/btn:scale-x-50 transition-transform origin-center" />
                                </button>
                            </div>
                        </section>

                        // Terminal Protocol Section
                        <section class="glass-card rounded-[3rem] p-10 border border-red-500/10 bg-red-500/5 animate-slide-up [animation-delay:600ms] group hover:border-red-500/30 transition-all duration-700">
                             <div class="flex items-center gap-4 mb-10">
                                <div class="w-12 h-12 rounded-2xl bg-red-500/20 flex items-center justify-center text-2xl shadow-2xl">"⚠️"</div>
                                <div class="space-y-1">
                                    <h2 class="text-2xl font-black italic tracking-tighter uppercase text-red-500">"Terminal" <span class="text-red-500/40">"Protocol"</span></h2>
                                    <p class="text-[10px] font-black uppercase tracking-widest text-red-500/30">"Emergency De-authentication"</p>
                                </div>
                            </div>
                            
                            <p class="text-sm font-medium italic text-red-500/60 mb-8 max-w-lg leading-relaxed">
                                "Executing this protocol will immediately decouple your neural instance from the central cluster and revoke all active session keys."
                            </p>

                             <button
                                on:click=move |_| auth.logout.dispatch(())
                                class="px-12 py-5 rounded-2xl bg-red-500 text-white font-black uppercase text-xs tracking-[0.2em] hover:scale-105 active:scale-95 transition-all shadow-2xl shadow-red-500/20"
                            >
                                "Terminate Instance"
                            </button>
                        </section>
                    </div>
                </div>
            </main>
        </Show>
    }
}
