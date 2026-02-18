use leptos::*;
use leptos_router::*;
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
        // Simple greeting logic (Client-side time)
        // Note: For SSR consistency this might need Hydration handling or be updated client-side only
        "Welcome back,"
    };

    view! {
         <Show when=move || auth.user.get().is_some()>
            <main class="min-h-screen bg-background text-foreground p-4 md:p-8 relative overflow-hidden animate-fade-in">
                <div class="max-w-6xl mx-auto relative z-10">
                    // Header
                    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
                        <div>
                            <p class="text-sm text-muted-foreground mb-1">"Good day"</p>
                            <h1 class="text-3xl md:text-4xl font-bold gradient-text">
                                {greeting} " " {user_name} "!"
                            </h1>
                        </div>
                        <div class="flex gap-3">
                            <a href="/settings" class="px-4 py-2 rounded-xl glass-subtle hover:bg-white/10 transition-all flex items-center gap-2 group">
                                "Settings"
                            </a>
                            <button
                                on:click=move |_| auth.logout.dispatch(())
                                class="px-4 py-2 rounded-xl bg-gradient-to-r from-red-500 to-rose-500 text-white hover:opacity-90 transition-all shadow-lg shadow-red-500/25 flex items-center gap-2"
                            >
                                "Logout"
                            </button>
                        </div>
                    </div>

                    // Quick Links (Simplified port)
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
                        <a href="/anime" class="group relative block p-6 rounded-2xl bg-gradient-to-br from-blue-500 via-purple-500 to-pink-500 text-white text-center overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105">
                             <span class="text-4xl mb-3 block group-hover:scale-110 transition-transform duration-300">"📺"</span>
                             <span class="font-medium">"Watch Anime"</span>
                        </a>
                         <a href="/komik" class="group relative block p-6 rounded-2xl bg-gradient-to-br from-orange-500 via-red-500 to-pink-500 text-white text-center overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105">
                             <span class="text-4xl mb-3 block group-hover:scale-110 transition-transform duration-300">"📖"</span>
                             <span class="font-medium">"Read Komik"</span>
                        </a>
                         <a href="/project" class="group relative block p-6 rounded-2xl bg-gradient-to-br from-pink-500 via-rose-500 to-red-500 text-white text-center overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105">
                             <span class="text-4xl mb-3 block group-hover:scale-110 transition-transform duration-300">"💼"</span>
                             <span class="font-medium">"Projects"</span>
                        </a>
                    </div>
                    
                    // Stats / Recent Activity Placeholders
                     <div class="glass-card rounded-2xl p-8 text-center">
                        <p class="text-muted-foreground">"No recent activity yet"</p>
                     </div>
                </div>
            </main>
         </Show>
    }
}
