use leptos::*;
use leptos_router::*;
use leptos_meta::Title;
use crate::providers::use_auth;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth = use_auth();
    // Redirect if not logged in
    create_effect(move |_| {
        if auth.user.get().is_none() {
             let navigate = use_navigate();
             navigate("/login", Default::default());
        }
    });

    let user_name = move || {
        auth.user.get().and_then(|u| u.name).unwrap_or("Guest".to_string())
    };

    let greeting = move || {
        "Welcome back,"
    };

    view! {
         <Show when=move || auth.user.get().is_some()>
            <Title text="Dashboard | Session Overview" />
            <main class="min-h-screen relative overflow-hidden pb-40 scanlines">
                // Background Ambient Systems
                <div class="fixed inset-0 pointer-events-none z-0">
                    <div class="absolute top-[10%] left-[-5%] w-[45rem] h-[45rem] bg-blue-500/5 rounded-full blur-[120px] animate-tilt" />
                    <div class="absolute bottom-[20%] right-[-10%] w-[50rem] h-[50rem] bg-purple-500/5 rounded-full blur-[120px] animate-tilt-reverse" />
                </div>

                <div class="container mx-auto max-w-6xl px-8 pt-24 space-y-16 relative z-10">
                    // Cinematic Hub Header
                    <header class="flex flex-col md:flex-row md:items-end justify-between gap-12 animate-fade-in">
                        <div class="space-y-4">
                            <div class="inline-flex items-center gap-3 px-4 py-1.5 rounded-full glass border border-white/10 text-[10px] font-black uppercase tracking-[0.4em] text-blue-400">
                                "User Dashboard"
                            </div>
                            <h1 class="text-4xl md:text-6xl font-black italic tracking-tighter uppercase leading-tight">
                                {greeting} <span class="text-blue-500">{user_name}</span>
                            </h1>
                        </div>

                        <div class="flex items-center gap-4">
                             <a href="/settings" class="px-8 py-4 rounded-2xl glass border border-white/10 hover:border-white/20 hover:bg-white/5 transition-all group flex items-center gap-3">
                                <svg class="w-5 h-5 text-muted-foreground group-hover:rotate-90 transition-transform duration-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                </svg>
                                <span class="text-[10px] font-black uppercase tracking-widest">"Settings"</span>
                            </a>
                            <button
                                on:click=move |_| auth.logout.dispatch(())
                                class="px-8 py-4 rounded-2xl bg-red-500 text-white font-black uppercase text-xs tracking-widest hover:scale-105 active:scale-95 transition-all shadow-[0_20px_40px_rgba(239,68,68,0.2)] industrial-snap"
                            >
                                "Logout"
                            </button>
                        </div>
                    </header>

                    // Core Sector Navigation
                    <section class="grid grid-cols-1 md:grid-cols-3 gap-8 animate-slide-up [animation-delay:200ms]">
                        <a href="/anime" class="group relative block p-10 rounded-[3rem] glass border border-white/5 overflow-hidden transition-all duration-700 hover:border-blue-500/30 hover:shadow-[0_50px_100px_rgba(0,0,0,0.5)]">
                             <div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
                             <div class="relative z-10 space-y-6">
                                <div class="w-16 h-16 rounded-3xl bg-blue-500/20 flex items-center justify-center text-4xl shadow-2xl group-hover:scale-110 transition-transform duration-500">"📺"</div>
                                <div class="space-y-2">
                                    <h2 class="text-2xl font-black uppercase italic tracking-tighter">"Anime" <span class="text-blue-500">"Library"</span></h2>
                                    <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40 leading-relaxed">"Media Streaming Service"</p>
                                </div>
                             </div>
                        </a>
                        
                        <a href="/komik" class="group relative block p-10 rounded-[3rem] glass border border-white/5 overflow-hidden transition-all duration-700 hover:border-orange-500/30 hover:shadow-[0_50px_100px_rgba(0,0,0,0.5)]">
                             <div class="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
                             <div class="relative z-10 space-y-6">
                                <div class="w-16 h-16 rounded-3xl bg-orange-500/20 flex items-center justify-center text-4xl shadow-2xl group-hover:scale-110 transition-transform duration-500">"📖"</div>
                                <div class="space-y-2">
                                    <h2 class="text-2xl font-black uppercase italic tracking-tighter">"Comic" <span class="text-orange-500">"Library"</span></h2>
                                    <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40 leading-relaxed">"Digital Comic Reader"</p>
                                </div>
                             </div>
                        </a>

                        <a href="/project" class="group relative block p-10 rounded-[3rem] glass border border-white/5 overflow-hidden transition-all duration-700 hover:border-purple-500/30 hover:shadow-[0_50px_100px_rgba(0,0,0,0.5)]">
                             <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
                             <div class="relative z-10 space-y-6">
                                <div class="w-16 h-16 rounded-3xl bg-purple-500/20 flex items-center justify-center text-4xl shadow-2xl group-hover:scale-110 transition-transform duration-500">"💼"</div>
                                <div class="space-y-2">
                                    <h2 class="text-2xl font-black uppercase italic tracking-tighter">"Project" <span class="text-purple-500">"Gallery"</span></h2>
                                    <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40 leading-relaxed">"Portfolio & Case Studies"</p>
                                </div>
                             </div>
                        </a>
                    </section>
                    
                    // Intelligence Feed Placeholder
                    <section class="animate-slide-up [animation-delay:400ms]">
                        <div class="glass-card rounded-[3rem] p-16 text-center border border-white/5 relative overflow-hidden group">
                            <div class="absolute inset-0 bg-white/2 opacity-0 group-hover:opacity-100 transition-opacity" />
                            <div class="relative z-10 space-y-6">
                                <div class="w-20 h-20 rounded-[2rem] bg-white/5 flex items-center justify-center text-5xl mx-auto shadow-2xl">"🔭"</div>
                                <div class="space-y-2">
                                    <h3 class="text-2xl font-black uppercase italic tracking-tighter">"No New Notifications"</h3>
                                    <p class="text-muted-foreground/40 font-medium italic">"You have no new notifications at this time."</p>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </main>
         </Show>
    }
}
