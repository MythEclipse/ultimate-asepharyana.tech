use leptos::*;
use leptos_router::*;
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
        if let Some(_) = auth.user.get() {
             let navigate = use_navigate();
             navigate("/dashboard", Default::default());
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set("".to_string());
        
        // Basic validation
        if email.get().is_empty() || password.get().is_empty() {
            set_error.set("Please fill in all fields".to_string());
            return;
        }

        auth.login.dispatch(LoginRequest {
            email: email.get(),
            password: password.get(),
            remember_me: false,
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center p-4 relative overflow-hidden animate-fade-in">
             // Animated Background (Simplified for Leptos)
            <div class="absolute inset-0 -z-10">
                <div class="absolute w-[400px] h-[400px] left-[-10%] top-[20%] bg-primary/20 blur-[100px] rounded-full animate-float" />
                <div class="absolute w-[500px] h-[500px] right-[-15%] bottom-[10%] bg-purple-500/20 blur-[100px] rounded-full animate-float-delayed" />
            </div>

            <div class="w-full max-w-md glass-card rounded-2xl overflow-hidden shadow-2xl transition-all duration-500 hover:shadow-primary/10">
                <div class="p-8">
                    // Header
                    <div class="text-center mb-8">
                        <div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-gradient-to-br from-primary via-accent to-cyan-400 flex items-center justify-center shadow-lg shadow-primary/25 transform transition-transform hover:scale-110 duration-500">
                             <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                             </svg>
                        </div>
                        <h1 class="text-2xl font-bold gradient-text">"Welcome Back"</h1>
                        <p class="text-muted-foreground mt-1">"Sign in to your account"</p>
                    </div>

                    // Error Message
                    <Show when=move || !error.get().is_empty()>
                        <div class="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-xl flex items-start gap-3 animate-fade-in">
                            <span class="text-destructive font-bold">"!"</span>
                            <p class="text-sm text-destructive">{move || error.get()}</p>
                        </div>
                    </Show>

                    <form on:submit=on_submit class="space-y-5">
                        <div class="space-y-2">
                            <label class="text-sm font-medium text-foreground">"Email"</label>
                            <input
                                type="email"
                                placeholder="your@email.com"
                                class="w-full pl-4 pr-4 py-3.5 rounded-xl border border-border bg-background/50 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all placeholder:text-muted-foreground/60"
                                prop:value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                required
                            />
                        </div>

                         <div class="space-y-2">
                            <label class="text-sm font-medium text-foreground">"Password"</label>
                            <div class="relative">
                                <input
                                    type=move || if show_password.get() { "text" } else { "password" }
                                    placeholder="••••••••"
                                    class="w-full pl-4 pr-12 py-3.5 rounded-xl border border-border bg-background/50 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:border-primary transition-all placeholder:text-muted-foreground/60"
                                    prop:value=password
                                    on:input=move |ev| set_password.set(event_target_value(&ev))
                                    required
                                />
                                <button
                                    type="button"
                                    on:click=move |_| set_show_password.update(|s| *s = !*s)
                                    class="absolute right-4 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                                >
                                    {move || if show_password.get() { "Hide" } else { "Show" }}
                                </button>
                            </div>
                        </div>

                        <button
                            type="submit"
                            class="w-full py-3.5 text-base font-semibold rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 transition-all shadow-lg shadow-primary/25 hover:shadow-xl hover:shadow-primary/30"
                            disabled=move || auth.login.pending().get()
                        >
                            {move || if auth.login.pending().get() { "Signing in..." } else { "Sign In" }}
                        </button>
                    </form>

                     <div class="mt-6 text-center">
                        <p class="text-sm text-muted-foreground">
                            "Don't have an account? "
                            <a href="/register" class="text-primary hover:underline font-semibold">"Sign up"</a>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
