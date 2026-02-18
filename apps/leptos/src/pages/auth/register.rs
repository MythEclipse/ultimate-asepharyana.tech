use leptos::*;
use leptos_router::*;
use leptos_meta::Title;
use crate::providers::use_auth;
use crate::api::types::LoginRequest;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let auth = use_auth();
    let (name, set_name) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (confirm_password, set_confirm_password) = create_signal("".to_string());
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
        
        if password.get() != confirm_password.get() {
            set_error.set("Encryption keys do not synchronize. Verification failed.".to_string());
            return;
        }

        if password.get().len() < 6 {
            set_error.set("Security mandates a minimum 6-character encryption key.".to_string());
            return;
        }

        auth.login.dispatch(LoginRequest {
            email: email.get(),
            password: password.get(),
            remember_me: false,
        });
    };

    view! {
        <Title text="Initialization | Identity Protocol"/>
        <main class="min-h-screen flex items-center justify-center p-6 relative overflow-hidden bg-black/20">
            // Cinematic Background Infrastructure
            <div class="fixed inset-0 pointer-events-none z-0">
                <div class="absolute bottom-[-10%] left-[-10%] w-[50rem] h-[50rem] bg-purple-500/10 rounded-full blur-[120px] animate-tilt" />
                <div class="absolute top-[-15%] right-[-5%] w-[45rem] h-[45rem] bg-indigo-500/10 rounded-full blur-[120px] animate-tilt-reverse" />
            </div>

            <div class="w-full max-w-[520px] relative z-10 animate-fade-in group/main">
                <div class="absolute -inset-1 bg-gradient-to-r from-purple-500 via-indigo-500 to-blue-500 rounded-[3rem] blur-2xl opacity-10 group-hover/main:opacity-20 transition-opacity duration-1000" />
                
                <div class="glass-card rounded-[3rem] border border-white/10 overflow-hidden shadow-[0_50px_100px_rgba(0,0,0,0.4)] transition-all duration-700 hover:border-white/20">
                    <div class="p-10 md:p-12 space-y-10">
                        // Security Header
                        <header class="text-center space-y-4">
                            <div class="space-y-2">
                                <h1 class="text-3xl font-black italic tracking-tighter uppercase leading-none">
                                    "Identity " <span class="bg-gradient-to-r from-purple-400 to-indigo-500 bg-clip-text text-transparent">"Protocol"</span>
                                </h1>
                                <p class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40">"Initialize New User Instance"</p>
                            </div>
                        </header>

                        // Error Notification
                        <Show when=move || !error.get().is_empty()>
                            <div class="p-4 rounded-2xl bg-red-500/5 border border-red-500/20 flex items-center gap-4 animate-slide-up">
                                <span class="w-8 h-8 rounded-lg bg-red-500/20 flex items-center justify-center text-red-500 font-black">"!"</span>
                                <p class="text-[10px] font-black uppercase tracking-widest text-red-400 leading-tight">{move || error.get()}</p>
                            </div>
                        </Show>

                        <form on:submit=on_submit class="space-y-6">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div class="space-y-2 group/input">
                                    <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2 transition-colors group-focus-within/input:text-purple-500">"Alias"</label>
                                    <input
                                        type="text"
                                        placeholder="User Entity"
                                        class="w-full bg-white/2 border border-white/5 rounded-2xl py-4 px-6 focus:outline-none focus:border-purple-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/10"
                                        prop:value=name
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                        required
                                    />
                                </div>
                                <div class="space-y-2 group/input">
                                    <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2 transition-colors group-focus-within/input:text-purple-500">"Uplink ID"</label>
                                    <input
                                        type="email"
                                        placeholder="id@network.root"
                                        class="w-full bg-white/2 border border-white/5 rounded-2xl py-4 px-6 focus:outline-none focus:border-purple-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/10"
                                        prop:value=email
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                        required
                                    />
                                </div>
                            </div>

                            <div class="space-y-2 group/input">
                                <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2 transition-colors group-focus-within/input:text-purple-500">"Initial Encryption Key"</label>
                                <div class="relative">
                                    <input
                                        type=move || if show_password.get() { "text" } else { "password" }
                                        placeholder="••••••••"
                                        class="w-full bg-white/2 border border-white/5 rounded-2xl py-4 px-6 focus:outline-none focus:border-purple-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/10"
                                        prop:value=password
                                        on:input=move |ev| set_password.set(event_target_value(&ev))
                                        required
                                    />
                                    <button
                                        type="button"
                                        on:click=move |_| set_show_password.update(|s| *s = !*s)
                                        class="absolute right-6 top-1/2 -translate-y-1/2 text-[10px] font-black uppercase tracking-widest text-muted-foreground/40 hover:text-purple-500 transition-colors"
                                    >
                                        {move || if show_password.get() { "Obscure" } else { "Reveal" }}
                                    </button>
                                </div>
                            </div>

                            <div class="space-y-2 group/input">
                                <label class="text-[10px] font-black uppercase tracking-[0.4em] text-muted-foreground/40 px-2 transition-colors group-focus-within/input:text-purple-500">"Key Verification"</label>
                                <input
                                    type=move || if show_password.get() { "text" } else { "password" }
                                    placeholder="••••••••"
                                    class="w-full bg-white/2 border border-white/5 rounded-2xl py-4 px-6 focus:outline-none focus:border-purple-500/30 transition-all text-sm font-medium tracking-tight placeholder:text-muted-foreground/10"
                                    prop:value=confirm_password
                                    on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                    required
                                />
                            </div>

                            <button
                                type="submit"
                                class="w-full py-6 rounded-2xl bg-foreground text-background font-black uppercase text-xs tracking-[0.3em] hover:scale-105 active:scale-95 transition-all shadow-2xl disabled:opacity-30 disabled:scale-100 relative group/btn overflow-hidden"
                                disabled=move || auth.login.pending().get()
                            >
                                <span class="relative z-10 transition-transform group-hover/btn:translate-x-1">
                                    {move || if auth.login.pending().get() { "Synchronizing..." } else { "Initialize Identity" }}
                                </span>
                                <div class="absolute inset-0 bg-gradient-to-r from-purple-500 to-indigo-600 opacity-0 group-hover/btn:opacity-10 transition-opacity" />
                            </button>
                        </form>

                        <footer class="text-center pt-4">
                            <p class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/40">
                                "Already Authenticated? "
                                <a href="/login" class="text-purple-500 hover:text-purple-400 transition-colors">"Access Gate"</a>
                            </p>
                        </footer>
                    </div>
                </div>
            </div>
        </main>
    }
}
