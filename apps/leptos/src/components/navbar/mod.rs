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
                     <A href="/project" class="relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 text-muted-foreground hover:text-foreground">
                        "Project"
                     </A>
                     <A href="/sosmed" class="relative px-4 py-2 text-sm font-medium transition-colors rounded-lg hover:bg-white/10 text-muted-foreground hover:text-foreground">
                        "Sosmed"
                     </A>

                     // Theme Toggle
                     <button
                        on:click=cycle_theme
                        class="p-2.5 rounded-lg hover:bg-white/10 transition-all group"
                        title="Toggle Theme"
                     >
                        <div class="relative w-5 h-5">
                            // Simplified Icon
                            <span class="text-xs">{move || format!("{:?}", theme.get())}</span>
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
                    <span class="sr-only">"Toggle menu"</span>
                    // Icon placeholder
                    "Menu"
                </button>
            </div>
            
            // Mobile Menu
            <Show when=move || is_open.get()>
                 <div class="md:hidden glass-card border-t border-white/10 animate-slide-down">
                    <div class="container mx-auto px-4 py-4 space-y-1">
                        <A href="/project" class="block py-3 px-4 text-sm font-medium rounded-lg text-muted-foreground hover:bg-white/5 hover:text-foreground transition-all">
                            "Project"
                        </A>
                        <A href="/sosmed" class="block py-3 px-4 text-sm font-medium rounded-lg text-muted-foreground hover:bg-white/5 hover:text-foreground transition-all">
                            "Sosmed"
                        </A>
                    </div>
                 </div>
            </Show>
        </nav>
    }
}
