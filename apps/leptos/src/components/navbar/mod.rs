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
        <nav class="sticky top-0 z-[100] w-full backdrop-blur-3xl bg-black/5 border-b border-indigo-500/10">
            <div class="container mx-auto flex h-24 items-center justify-between px-8">
                // Logo Protocol
                <A href="/" class="flex items-center space-x-6 group">
                    <div class="relative">
                        <div class="absolute -inset-4 bg-indigo-500/20 rounded-full blur-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-700 animate-pulse" />
                        <div class="w-12 h-12 rounded-2xl bg-foreground flex items-center justify-center shadow-2xl group-hover:scale-110 transition-transform duration-500 relative z-10">
                            <span class="text-background font-black text-xl italic tracking-tighter">"A"</span>
                        </div>
                    </div>
                    <div class="hidden sm:block space-y-0.5">
                        <span class="text-xl font-black italic tracking-tighter uppercase leading-none block group-hover:text-indigo-500 transition-colors">
                            "Asep" <span class="text-indigo-500 group-hover:text-foreground transition-colors">"Haryana"</span>
                        </span>
                        <span class="text-[8px] font-black uppercase tracking-[0.5em] text-muted-foreground/40 block">"Personal Portfolio"</span>
                    </div>
                </A>

                // Cinematic Link Array
                <div class="hidden md:flex items-center space-x-2">
                     <NavLink href="/project" label="Projects" current_path=location.pathname />

                     <div class="w-8 h-px bg-white/5 mx-4" />

                     // Theme Engine
                     <button
                        on:click=cycle_theme
                        class="p-4 rounded-2xl hover:bg-white/5 transition-all group relative overflow-hidden active:scale-95"
                        title=move || format!("Theme: {:?}", theme.get())
                     >
                        <div class="relative z-10 w-5 h-5 flex items-center justify-center">
                            <Show
                                when=move || theme.get() == Theme::Dark
                                fallback=move || view! {
                                    <svg class="w-5 h-5 text-amber-500 group-hover:rotate-180 transition-transform duration-700" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                }
                            >
                                <svg class="w-5 h-5 text-indigo-400 group-hover:-rotate-12 transition-transform duration-700" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                </svg>
                            </Show>
                        </div>
                        <div class="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
                     </button>
                     
                     // Security Handshake
                     <Show
                        when=move || user.get().is_some()
                        fallback=|| view! {
                             <A href="/login" class="ml-4 px-8 py-3.5 text-[10px] font-black uppercase tracking-[0.3em] rounded-2xl bg-foreground text-background hover:scale-105 active:scale-95 transition-all shadow-2xl relative group/auth overflow-hidden">
                                <span class="relative z-10">"Login"</span>
                                <div class="absolute inset-0 bg-gradient-to-r from-indigo-500 to-purple-500 opacity-0 group-hover/auth:opacity-10 transition-opacity" />
                             </A>
                        }
                     >
                        <div class="flex items-center gap-4 ml-4">
                             <A href="/dashboard" class="px-8 py-3.5 text-[10px] font-black uppercase tracking-[0.3em] rounded-2xl glass border border-indigo-500/20 text-indigo-400 hover:bg-indigo-500/5 hover:border-indigo-500/40 transition-all active:scale-95">
                                "Dashboard"
                             </A>
                        </div>
                     </Show>
                </div>

                // Mobile Navigation Trigger
                <button
                    class="md:hidden p-4 rounded-2xl hover:bg-white/5 transition-all active:scale-95"
                    on:click=toggle_menu
                >
                    <div class="space-y-1.5 w-6">
                        <div class=move || format!("h-0.5 bg-foreground rounded-full transition-all duration-500 {}", if is_open.get() { "rotate-45 translate-y-2" } else { "w-full" }) />
                        <div class=move || format!("h-0.5 bg-foreground rounded-full transition-all duration-500 {}", if is_open.get() { "opacity-0" } else { "w-2/3" }) />
                        <div class=move || format!("h-0.5 bg-foreground rounded-full transition-all duration-500 {}", if is_open.get() { "-rotate-45 -translate-y-2" } else { "w-full" }) />
                    </div>
                </button>
            </div>
            
            // Expanded Mobile Interface
            <Show when=move || is_open.get()>
                 <div class="md:hidden border-t border-indigo-500/10 bg-black/95 backdrop-blur-3xl animate-fade-in overflow-hidden">
                    <div class="container mx-auto px-8 py-10 space-y-4">
                        <MobileNavLink 
                            href="/project" 
                            label="Projects" 
                            is_active=move || {
                                let p = location.pathname.get();
                                p == "/project" || p.starts_with("/project/")
                            } 
                            on_click=move |_| set_is_open.set(false) 
                        />
                        
                         <div class="pt-8 mt-8 border-t border-white/5 flex items-center justify-between">
                            <button
                                on:click=cycle_theme
                                class="flex items-center gap-4 text-[10px] font-black uppercase tracking-widest text-muted-foreground px-6 py-4 rounded-2xl glass border border-white/5 hover:border-white/20 transition-all"
                            > 
                                <span class="text-lg">
                                    {move || match theme.get() {
                                        Theme::Light => "☀️",
                                        Theme::Dark => "🌙",
                                        Theme::System => "💻",
                                    }}
                                </span>
                                {move || format!("{:?}", theme.get())}
                            </button>
                            
                            <A href="/login" on:click=move |_| set_is_open.set(false) class="px-8 py-4 rounded-2xl bg-indigo-500 text-white text-[10px] font-black uppercase tracking-[0.3em] shadow-2xl">
                                "Login"
                            </A>
                         </div>
                    </div>
                 </div>
            </Show>
        </nav>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str, current_path: Memo<String>) -> impl IntoView {
    let is_active = move || {
        let path = current_path.get();
        if path == href {
            return true;
        }
        if href != "/" && path.starts_with(href) {
            return path.chars().nth(href.len()) == Some('/');
        }
        false
    };
    
    view! {
        <A href=href class=move || {
            format!(
                "px-6 py-2.5 text-[10px] font-black uppercase tracking-[0.3em] transition-all duration-500 rounded-xl relative group/link {}",
                if is_active() { "text-indigo-400" } else { "text-muted-foreground hover:text-foreground hover:bg-white/5" }
            )
        }>
            <span class="relative z-10">{label}</span>
            <Show when=is_active>
                <div class="absolute inset-0 bg-indigo-500/5 rounded-xl border border-indigo-500/10 animate-fade-in" />
                <div class="absolute -bottom-1 left-1/2 -translate-x-1/2 w-4 h-1 bg-indigo-500 rounded-full blur-[2px] animate-pulse" />
            </Show>
        </A>
    }
}

#[component]
fn MobileNavLink<A, C>(
    href: &'static str, 
    label: &'static str, 
    is_active: A, 
    on_click: C
) -> impl IntoView 
where 
    A: Fn() -> bool + 'static,
    C: Fn(leptos::ev::MouseEvent) + 'static
{
    view! {
        <A 
            href=href 
            class=move || format!(
                "block py-5 px-8 text-lg font-black italic uppercase tracking-tighter border rounded-3xl transition-all duration-500 active:scale-95 {}",
                if is_active() { "bg-indigo-500/10 border-indigo-500/20 text-indigo-400" } else { "border-white/5 hover:bg-white/5 hover:border-white/10 text-muted-foreground hover:text-foreground" }
            )
            on:click=on_click
        >
            {label}
        </A>
    }
}
