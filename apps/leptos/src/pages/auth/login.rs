use leptos::*;
use leptos_router::*;
use leptos_meta::Title;
use crate::providers::use_auth;
use crate::api::types::LoginRequest;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (error, set_error) = create_signal("".to_string());
    let (show_password, set_show_password) = create_signal(false);

    // Redirect if already logged in
    create_effect(move |_| {
        if auth.user.get().is_some() {
             let navigate = use_navigate();
             navigate("/dashboard", Default::default());
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set("".to_string());
        
        if email.get().is_empty() || password.get().is_empty() {
            set_error.set("Security clearance requires complete credentials.".to_string());
            return;
        }

        auth.login.dispatch(LoginRequest {
            email: email.get(),
            password: password.get(),
            remember_me: false,
        });
    };

    view! {
        <Title text="Authentication | Secure Access Gate"/>
        <main class="min-h-screen flex items-center justify-center p-6 relative overflow-hidden bg-black/20">
            // Cinematic Background Infrastructure
            <div class="fixed inset-0 pointer-events-none z-0">
                <div class="absolute top-[-10%] left-[-10%] w-[50rem] h-[50rem] bg-indigo-500/10 rounded-full blur-[120px] animate-tilt" />
                <div class="absolute bottom-[-15%] right-[-5%] w-[45rem] h-[45rem] bg-blue-500/10 rounded-full blur-[120px] animate-tilt-reverse" />
            </div>

            <div class="w-full max-w-[480px] relative z-10 animate-fade-in group/main">
                <div class="absolute -inset-1 bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500 rounded-[3rem] blur-2xl opacity-10 group-hover/main:opacity-20 transition-opacity duration-1000" />
                
                <div class="glass-card rounded-[3rem] border border-white/10 overflow-hidden shadow-[0_50px_100px_rgba(0,0,0,0.4)] transition-all duration-700 hover:border-white/20">
                    <div class="p-12 space-y-10">
                        // Security Header
                        <header class="text-center space-y-6">
                            <div class="relative w-24 h-24 mx-auto group">
                                <div class="absolute inset-0 bg-blue-500/20 blur-2xl rounded-full scale-125 animate-pulse" />
                                <div class="relative w-full h-full rounded-[2rem] bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-4xl shadow-2xl transition-transform duration-700 group-hover:scale-110">
                                    <span class="relative z-10">"🛡️"</span>
                                </div>
                            </div>
                            <div class="space-y-2">
                                <h1 class="text-3xl font-black italic tracking-tighter uppercase leading-none">
                                    "Access " <span class="bg-gradient-to-r from-blue-400 to-indigo-500 bg-clip-text text-transparent">"Portal"</span>
                                </h1>
                                <p class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40">"Validate Identity Signature"</p>
                            </div>
                        </header>

                        // Error Notification
                        <Show when=move || !error.get().is_empty()>
                            <div class="p-4 rounded-2xl bg-red-500/5 border border-red-500/20 flex items-center gap-4 animate-slide-up">
                                <span class="w-8 h-8 rounded-lg bg-red-500/20 flex items-center justify-center text-red-500 font-black">"!"</span>
                                <p class="text-[10px] font-black uppercase tracking-widest text-red-400 leading-tight">{move || error.get()}</p>
                            </div>
                        </Show>

                        <form on:submit=on_submit class="space-y-8">
                            <div class="space-y-6">
                                <div class="space-y-2 group/input">
                                    <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2 transition-colors group-focus-within/input:text-blue-500">"Uplink ID"</label>
                                    <div class="relative">
                                        <div class="absolute left-6 top-1/2 -translate-y-1/2 text-muted-foreground/30">
                                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.206" />
                                            </svg>
                                        </div>
                                        <input
                                            type="email"
                                            placeholder="identity@network.root"
                                            class="w-full bg-white/2 border border-white/5 rounded-2xl py-5 pl-16 pr-6 focus:outline-none focus:border-blue-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/10"
                                            prop:value=email
                                            on:input=move |ev| set_email.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>

                                <div class="space-y-2 group/input">
                                    <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2 transition-colors group-focus-within/input:text-blue-500">"Encryption Key"</label>
                                    <div class="relative">
                                        <div class="absolute left-6 top-1/2 -translate-y-1/2 text-muted-foreground/30">
                                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                            </svg>
                                        </div>
                                        <input
                                            type=move || if show_password.get() { "text" } else { "password" }
                                            placeholder="••••••••"
                                            class="w-full bg-white/2 border border-white/5 rounded-2xl py-5 pl-16 pr-20 focus:outline-none focus:border-blue-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/10"
                                            prop:value=password
                                            on:input=move |ev| set_password.set(event_target_value(&ev))
                                        />
                                        <button
                                            type="button"
                                            on:click=move |_| set_show_password.update(|s| *s = !*s)
                                            class="absolute right-6 top-1/2 -translate-y-1/2 text-[10px] font-black uppercase tracking-widest text-muted-foreground/40 hover:text-blue-500 transition-colors"
                                        >
                                            {move || if show_password.get() { "Obscure" } else { "Reveal" }}
                                        </button>
                                    </div>
                                </div>
                            </div>

                            <button
                                type="submit"
                                class="w-full py-6 rounded-2xl bg-foreground text-background font-black uppercase text-xs tracking-[0.3em] hover:scale-105 active:scale-95 transition-all shadow-[0_20px_40px_rgba(0,0,0,0.3)] disabled:opacity-30 disabled:scale-100 relative group/btn overflow-hidden"
                                disabled=move || auth.login.pending().get()
                            >
                                <span class="relative z-10 transition-transform group-hover/btn:translate-x-1">
                                    {move || if auth.login.pending().get() { "Decrypting..." } else { "Authenticate" }}
                                </span>
                                <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-indigo-600 opacity-0 group-hover/btn:opacity-10 transition-opacity" />
                            </button>
                        </form>

                        <footer class="text-center">
                            <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">
                                "Unauthorized Access Attempt? "
                                <a href="/register" class="text-blue-500 hover:text-blue-400 transition-colors">"Initialization Protocol"</a>
                            </p>
                        </footer>
                    </div>
                </div>
            </div>
        </main>
    }
}
