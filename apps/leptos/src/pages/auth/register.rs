use leptos::*;
use leptos_router::*;
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
        
        if password.get() != confirm_password.get() {
            set_error.set("Passwords do not match".to_string());
            return;
        }

        if password.get().len() < 6 {
            set_error.set("Password must be at least 6 characters".to_string());
            return;
        }

        // Simulating registration by logging in for now
        // Ideally we should have a register action, but for now reuse login as per previous code
        // Note: Real registration API logic should be implemented later. Use LoginRequest for compatibility.
        auth.login.dispatch(LoginRequest {
            email: email.get(),
            password: password.get(),
            remember_me: false,
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center p-4 relative overflow-hidden animate-fade-in">
            <div class="absolute inset-0 -z-10">
                // Background effects
            </div>

            <div class="w-full max-w-md glass-card rounded-2xl overflow-hidden shadow-2xl">
                <div class="p-8">
                     <div class="text-center mb-8">
                        <h1 class="text-3xl font-bold gradient-text">"Create Account"</h1>
                        <p class="text-muted-foreground mt-2">"Join us and start your journey"</p>
                    </div>

                    <Show when=move || !error.get().is_empty()>
                         <div class="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-xl">
                            <p class="text-sm text-destructive">{move || error.get()}</p>
                        </div>
                    </Show>

                    <form on:submit=on_submit class="space-y-5">
                         <div class="space-y-2">
                            <label class="text-sm font-medium text-foreground">"Name"</label>
                            <input
                                type="text"
                                placeholder="Your name"
                                class="w-full pl-4 pr-4 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all"
                                prop:value=name
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                required
                            />
                        </div>

                         <div class="space-y-2">
                            <label class="text-sm font-medium text-foreground">"Email"</label>
                            <input
                                type="email"
                                placeholder="your@email.com"
                                class="w-full pl-4 pr-4 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all"
                                prop:value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                required
                            />
                        </div>

                        <div class="space-y-2">
                            <label class="text-sm font-medium text-foreground">"Password"</label>
                            <input
                                type="password"
                                placeholder="••••••••"
                                class="w-full pl-4 pr-4 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all"
                                prop:value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                required
                            />
                        </div>

                        <div class="space-y-2">
                            <label class="text-sm font-medium text-foreground">"Confirm Password"</label>
                            <input
                                type="password"
                                placeholder="••••••••"
                                class="w-full pl-4 pr-4 py-3.5 rounded-xl glass-subtle border border-border/50 focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all"
                                prop:value=confirm_password
                                on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                                required
                            />
                        </div>

                        <button
                            type="submit"
                            class="w-full py-4 text-base font-semibold rounded-xl bg-gradient-to-r from-primary via-purple-600 to-primary text-white hover:opacity-90 transition-all shadow-lg shadow-primary/30"
                            disabled=move || auth.login.pending().get()
                        >
                             {move || if auth.login.pending().get() { "Creating account..." } else { "Create Account" }}
                        </button>
                    </form>

                    <div class="mt-6 text-center">
                        <p class="text-sm text-muted-foreground">
                            "Already have an account? "
                            <a href="/login" class="text-primary hover:underline font-semibold">"Sign in"</a>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
