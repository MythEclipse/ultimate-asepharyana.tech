use leptos::*;
use leptos_router::*;
use crate::providers::{use_theme, use_auth, Theme};

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let theme_ctx = use_theme();
    let auth_ctx = use_auth();
    
    let theme = theme_ctx.theme;
    let set_theme = theme_ctx.set_theme;
    let user = auth_ctx.user;

    let toggle_menu = move |_| set_is_open.update(|open| *open = !*open);

    let cycle_theme = move |_| {
        set_theme.update(|t| *t = match *t {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::System,
            Theme::System => Theme::Light,
        });
    };

    let location = use_location();

    view! {
        <nav class="sticky top-0 z-50 w-full glass-subtle border-b border-white/10">
            <div class="container mx-auto flex h-16 items-center justify-between px-4">
                // Logo
                <A href="/" class="flex items-center space-x-2 group">
                    <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary via-accent to-neon-cyan flex items-center justify-center shadow-lg group-hover:shadow-primary/30 transition-shadow">
                        <span class="text-white font-bold text-sm">"A"</span>
                    </div>
                    <span class="text-xl font-bold gradient-text-animated">
                        "Asepharyana"
                    </span>
                </A>

                // Desktop Navigation
                <div class="hidden md:flex items-center space-x-1">
                     <A href="/project" class=move || {
                        let is_active = location.pathname.get().starts_with("/project");
                        if is_active {
                            "relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 text-primary"
                        } else {
                            "relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 text-muted-foreground hover:text-foreground"
                        }
                     }>
                        "Project"
                        <Show when=move || location.pathname.get().starts_with("/project")>
                            <span class="absolute bottom-0 left-1/2 -translate-x-1/2 w-4 h-0.5 bg-gradient-to-r from-primary to-accent rounded-full" />
                        </Show>
                     </A>
                     <A href="/sosmed" class=move || {
                        let is_active = location.pathname.get().starts_with("/sosmed");
                        if is_active {
                            "relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 text-primary"
                        } else {
                            "relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 text-muted-foreground hover:text-foreground"
                        }
                     }>
                        "Sosmed"
                        <Show when=move || location.pathname.get().starts_with("/sosmed")>
                            <span class="absolute bottom-0 left-1/2 -translate-x-1/2 w-4 h-0.5 bg-gradient-to-r from-primary to-accent rounded-full" />
                        </Show>
                     </A>

                     // Theme Toggle
                     <button
                        on:click=cycle_theme
                        class="p-2.5 rounded-lg hover:bg-white/10 transition-all group"
                        title=move || format!("Theme: {:?}", theme.get())
                     >
                        <div class="relative w-5 h-5">
                            <Show
                                when=move || theme.get() == Theme::Dark
                                fallback=move || view! {
                                    <svg class="w-5 h-5 text-amber-500 group-hover:rotate-45 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                }
                            >
                                <svg class="w-5 h-5 text-indigo-400 group-hover:-rotate-12 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                </svg>
                            </Show>
                        </div>
                     </button>
                     
                     // Auth
                     <Show
                        when=move || user.get().is_some()
                        fallback=|| view! {
                             <A href="/login" class="ml-2 px-4 py-2 text-sm font-medium rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-all hover:shadow-lg hover:shadow-primary/25">
                                "Sign In"
                             </A>
                        }
                     >
                        <div class="flex items-center gap-2 ml-2">
                             <A href="/dashboard" class="px-4 py-2 text-sm font-medium rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors">
                                "Dashboard"
                             </A>
                        </div>
                     </Show>
                </div>

                // Mobile Menu Button
                <button
                    class="md:hidden p-2.5 rounded-lg hover:bg-white/10 transition-colors"
                    on:click=toggle_menu
                >
                    <Show
                        when=move || is_open.get()
                        fallback=move || view! {
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        }
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </Show>
                </button>
            </div>
            
            // Mobile Menu
            <Show when=move || is_open.get()>
                 <div class="md:hidden glass-card border-t border-white/10 animate-slide-down">
                    <div class="container mx-auto px-4 py-4 space-y-1">
                        <A href="/project" 
                           class=move || {
                               let is_active = location.pathname.get().starts_with("/project");
                               format!("block py-3 px-4 text-sm font-medium rounded-lg transition-all {}",
                                   if is_active { "bg-primary/10 text-primary" } else { "text-muted-foreground hover:bg-white/5 hover:text-foreground" }
                               )
                           }
                           on:click=move |_| set_is_open.set(false)
                        >
                            "Project"
                        </A>
                        <A href="/sosmed" 
                           class=move || {
                               let is_active = location.pathname.get().starts_with("/sosmed");
                               format!("block py-3 px-4 text-sm font-medium rounded-lg transition-all {}",
                                   if is_active { "bg-primary/10 text-primary" } else { "text-muted-foreground hover:bg-white/5 hover:text-foreground" }
                               )
                           }
                           on:click=move |_| set_is_open.set(false)
                        >
                            "Sosmed"
                        </A>
                        
                         <div class="flex items-center justify-between pt-4 mt-4 border-t border-border/50">
                            <button
                                on:click=cycle_theme
                                class="flex items-center gap-2 text-sm text-muted-foreground px-4 py-2 rounded-lg hover:bg-white/5"
                            > 
                                <Show
                                    when=move || theme.get() == Theme::Dark
                                    fallback=move || view! {
                                        <svg class="w-4 h-4 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                        </svg>
                                    }
                                >
                                    <svg class="w-4 h-4 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                    </svg>
                                </Show>
                                {move || format!("{:?}", theme.get())}
                            </button>
                         </div>
                    </div>
                 </div>
            </Show>
        </nav>
    }
}
